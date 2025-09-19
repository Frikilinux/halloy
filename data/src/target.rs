use std::hash::Hash;
use std::sync::Arc;
use std::{cmp, fmt};

use irc::proto;
use serde::{Deserialize, Serialize};

use crate::isupport;
use crate::user::{Nick, NickRef, User};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Target {
    Channel(Channel),
    Query(Query),
}

impl Target {
    pub fn as_channel(&self) -> Option<&Channel> {
        match self {
            Target::Channel(channel) => Some(channel),
            Target::Query(_) => None,
        }
    }

    pub fn to_channel(self) -> Option<Channel> {
        match self {
            Target::Channel(channel) => Some(channel),
            Target::Query(_) => None,
        }
    }

    pub fn as_query(&self) -> Option<&Query> {
        match self {
            Target::Channel(_) => None,
            Target::Query(query) => Some(query),
        }
    }

    pub fn to_query(self) -> Option<Query> {
        match self {
            Target::Channel(_) => None,
            Target::Query(query) => Some(query),
        }
    }

    pub fn as_normalized_str(&self) -> &str {
        match self {
            Target::Channel(channel) => channel.as_normalized_str(),
            Target::Query(query) => query.as_normalized_str(),
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            Target::Channel(channel) => channel.as_str(),
            Target::Query(query) => query.as_str(),
        }
    }

    pub fn parse(
        target: &str,
        chantypes: &[char],
        statusmsg: &[char],
        casemapping: isupport::CaseMap,
    ) -> Self {
        if let Some((prefixes, channel)) =
            proto::parse_channel_from_target(target, chantypes, statusmsg)
        {
            Target::Channel(Channel::from(ChannelData {
                prefixes,
                normalized: casemapping.normalize(&channel),
                raw: target.to_string(),
            }))
        } else {
            Target::Query(Query::from(QueryData {
                normalized: casemapping.normalize(target),
                raw: target.to_string(),
            }))
        }
    }
}

impl PartialEq for Target {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Target::Channel(channel), Target::Channel(other_channel)) => {
                channel.eq(other_channel)
            }
            (Target::Query(query), Target::Query(other_query)) => {
                query.eq(other_query)
            }
            _ => false,
        }
    }
}

impl Eq for Target {}

impl Hash for Target {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self {
            Target::Channel(channel) => channel.hash(state),
            Target::Query(query) => query.hash(state),
        }
    }
}

impl Ord for Target {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        match (self, other) {
            (Target::Channel(channel), Target::Channel(other_channel)) => {
                channel.cmp(other_channel)
            }
            (Target::Channel(_), Target::Query(_)) => cmp::Ordering::Less,
            (Target::Query(query), Target::Query(other_query)) => {
                query.cmp(other_query)
            }
            (Target::Query(_), Target::Channel(_)) => cmp::Ordering::Greater,
        }
    }
}

impl PartialOrd for Target {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl fmt::Display for Target {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Target::Channel(channel) => channel.fmt(f),
            Target::Query(query) => query.fmt(f),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct ChannelData {
    prefixes: Vec<char>,
    normalized: String,
    raw: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Channel(Arc<ChannelData>);

impl From<ChannelData> for Channel {
    fn from(inner: ChannelData) -> Self {
        Channel(Arc::new(inner))
    }
}

impl Channel {
    pub fn as_normalized_str(&self) -> &str {
        self.0.normalized.as_ref()
    }

    pub fn as_str(&self) -> &str {
        self.0.raw.as_ref()
    }

    pub fn from_str(
        target: &str,
        chantypes: &[char],
        casemapping: isupport::CaseMap,
    ) -> Self {
        let inner = if let Some(index) = target.find(chantypes) {
            // This will not panic, since `find` always returns a valid codepoint index.
            // We call `find` -> `split_at` because it is an _inclusive_ split, which includes the match.
            let (prefixes, channel) = target.split_at(index);

            ChannelData {
                prefixes: prefixes.chars().collect(),
                normalized: casemapping.normalize(channel),
                raw: target.to_string(),
            }
        } else {
            ChannelData {
                prefixes: vec![],
                normalized: casemapping.normalize(target),
                raw: target.to_string(),
            }
        };
        Channel::from(inner)
    }

    pub fn parse(
        target: &str,
        chantypes: &[char],
        statusmsg: &[char],
        casemapping: isupport::CaseMap,
    ) -> Result<Self, ParseError> {
        if let Some((prefixes, channel)) =
            proto::parse_channel_from_target(target, chantypes, statusmsg)
        {
            Ok(Channel::from(ChannelData {
                prefixes,
                normalized: casemapping.normalize(&channel),
                raw: target.to_string(),
            }))
        } else {
            Err(ParseError::InvalidChannel(target.to_string()))
        }
    }

    pub fn prefixes(&self) -> &[char] {
        &self.0.prefixes
    }

    pub fn to_target(&self) -> Target {
        Target::Channel(self.clone())
    }
}

impl PartialEq for Channel {
    fn eq(&self, other: &Self) -> bool {
        self.0.normalized.eq(&other.0.normalized)
    }
}

impl Eq for Channel {}

impl Hash for Channel {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.normalized.hash(state);
    }
}

impl Ord for Channel {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.0.normalized.cmp(&other.0.normalized)
    }
}

impl PartialOrd for Channel {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl fmt::Display for Channel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.raw.fmt(f)
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct QueryData {
    normalized: String,
    raw: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Query(Arc<QueryData>);

impl From<QueryData> for Query {
    fn from(inner: QueryData) -> Self {
        Query(Arc::new(inner))
    }
}

impl From<User> for Query {
    fn from(user: User) -> Self {
        Query::from(<User as std::convert::Into<Nick>>::into(user))
    }
}

impl From<&User> for Query {
    fn from(user: &User) -> Self {
        Query::from(user.nickname())
    }
}

impl From<Nick> for Query {
    fn from(nick: Nick) -> Self {
        let (raw, normalized) = nick.into();

        Query::from(QueryData { raw, normalized })
    }
}

impl From<NickRef<'_>> for Query {
    fn from(nickref: NickRef) -> Self {
        Query::from(QueryData {
            raw: nickref.as_str().to_string(),
            normalized: nickref.as_normalized_str().to_string(),
        })
    }
}

impl Query {
    pub fn as_normalized_str(&self) -> &str {
        self.0.normalized.as_ref()
    }

    pub fn as_str(&self) -> &str {
        self.0.raw.as_ref()
    }

    pub fn parse(
        target: &str,
        chantypes: &[char],
        statusmsg: &[char],
        casemapping: isupport::CaseMap,
    ) -> Result<Self, ParseError> {
        if let Some((_, _)) =
            proto::parse_channel_from_target(target, chantypes, statusmsg)
        {
            Err(ParseError::InvalidQuery(target.to_string()))
        } else {
            Ok(Query::from(QueryData {
                normalized: casemapping.normalize(target),
                raw: target.to_string(),
            }))
        }
    }

    pub fn to_target(&self) -> Target {
        Target::Query(self.clone())
    }
}

impl PartialEq for Query {
    fn eq(&self, other: &Self) -> bool {
        self.0.normalized.eq(&other.0.normalized)
    }
}

impl Eq for Query {}

impl Hash for Query {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.normalized.hash(state);
    }
}

impl Ord for Query {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.0.normalized.cmp(&other.0.normalized)
    }
}

impl PartialOrd for Query {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl fmt::Display for Query {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.raw.fmt(f)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ParseError {
    #[error("unable to parse channel from {0}")]
    InvalidChannel(String),
    #[error("unable to parse query from {0}")]
    InvalidQuery(String),
}

pub fn join_targets(targets: Vec<&str>) -> String {
    if let Some((last_target, targets)) = targets.split_last() {
        if let Some((first_target, targets)) = targets.split_first() {
            if targets.is_empty() {
                format!("{first_target} and {last_target}")
            } else {
                format!(
                    "{first_target}, {}, and {last_target}",
                    targets.join(", ")
                )
            }
        } else {
            last_target.to_string()
        }
    } else {
        String::new()
    }
}
