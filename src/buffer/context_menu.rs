use chrono::{DateTime, Local, Utc};
use data::dashboard::BufferAction;
use data::user::Nick;
use data::{Config, Server, User, config, ctcp, isupport, target};
use iced::widget::{Space, button, column, container, row, rule, text};
use iced::{Length, Padding, padding};

use crate::widget::{Element, context_menu, double_pass};
use crate::{Theme, font, theme, widget};

pub enum Context<'a> {
    User {
        server: &'a Server,
        prefix: &'a [isupport::PrefixMap],
        channel: Option<&'a target::Channel>,
        user: &'a User,
        current_user: Option<&'a User>,
    },
    Url(&'a String),
    Timestamp(&'a DateTime<Utc>),
}

#[derive(Debug, Clone, Copy)]
pub enum Entry {
    // user context
    Whois,
    Query,
    ToggleAccessLevelOp,
    ToggleAccessLevelVoice,
    SendFile,
    UserInfo,
    HorizontalRule,
    CtcpRequestTime,
    CtcpRequestVersion,
    // url context
    CopyUrl,
    // timestamp context
    Timestamp,
}

impl Entry {
    pub fn timestamp_list() -> Vec<Self> {
        vec![Entry::Timestamp]
    }

    pub fn url_list() -> Vec<Self> {
        vec![Entry::CopyUrl]
    }

    pub fn user_list(is_channel: bool, our_user: Option<&User>) -> Vec<Self> {
        if is_channel {
            if our_user.is_some_and(|u| {
                u.has_access_level(data::user::AccessLevel::Oper)
            }) {
                vec![
                    Entry::UserInfo,
                    Entry::HorizontalRule,
                    Entry::Whois,
                    Entry::Query,
                    Entry::SendFile,
                    Entry::HorizontalRule,
                    Entry::ToggleAccessLevelOp,
                    Entry::ToggleAccessLevelVoice,
                    Entry::HorizontalRule,
                    Entry::CtcpRequestVersion,
                    Entry::CtcpRequestTime,
                ]
            } else {
                vec![
                    Entry::UserInfo,
                    Entry::HorizontalRule,
                    Entry::Whois,
                    Entry::Query,
                    Entry::SendFile,
                    Entry::HorizontalRule,
                    Entry::CtcpRequestVersion,
                    Entry::CtcpRequestTime,
                ]
            }
        } else {
            vec![Entry::Whois, Entry::SendFile]
        }
    }

    pub fn view<'a>(
        self,
        context: Option<Context<'_>>,
        length: Length,
        config: &Config,
        theme: &Theme,
    ) -> Element<'a, Message> {
        context.map_or(row![].into(), |context| match (self, context) {
            (Entry::Whois, Context::User { server, user, .. }) => {
                let message =
                    Message::Whois(server.clone(), user.nickname().to_owned());

                menu_button("Whois".to_string(), Some(message), length, theme)
            }
            (Entry::Query, Context::User { server, user, .. }) => {
                let message = Message::Query(
                    server.clone(),
                    target::Query::from(user.clone()),
                    config.actions.buffer.message_user,
                );

                menu_button("Message".to_string(), Some(message), length, theme)
            }
            (
                Entry::ToggleAccessLevelOp,
                Context::User {
                    server,
                    prefix,
                    channel,
                    user,
                    ..
                },
            ) => {
                let operator_mode = prefix.iter().find_map(|prefix_map| {
                    (prefix_map.prefix == '@').then_some(prefix_map.mode)
                });

                let (label, message) =
                    if let (Some(channel), Some(operator_mode)) =
                        (channel, operator_mode)
                    {
                        let is_op = user
                            .has_access_level(data::user::AccessLevel::Oper);
                        let prefix = if is_op { "-" } else { "+" };
                        let action = format!("{prefix}{operator_mode}");

                        (
                            format!(
                                "{} Op ({action})",
                                if is_op { "Take" } else { "Give" }
                            ),
                            Some(Message::ToggleAccessLevel(
                                server.clone(),
                                channel.clone(),
                                user.nickname().to_owned(),
                                action,
                            )),
                        )
                    } else {
                        (String::new(), None)
                    };

                menu_button(label, message, length, theme)
            }
            (
                Entry::ToggleAccessLevelVoice,
                Context::User {
                    server,
                    prefix,
                    channel,
                    user,
                    ..
                },
            ) => {
                let voice_mode = prefix.iter().find_map(|prefix_map| {
                    (prefix_map.prefix == '+').then_some(prefix_map.mode)
                });

                let (label, message) =
                    if let (Some(channel), Some(voice_mode)) =
                        (channel, voice_mode)
                    {
                        let has_voice = user
                            .has_access_level(data::user::AccessLevel::Voice);
                        let prefix = if has_voice { "-" } else { "+" };
                        let action = format!("{prefix}{voice_mode}");

                        (
                            format!(
                                "{} Voice ({action})",
                                if has_voice { "Take" } else { "Give" }
                            ),
                            Some(Message::ToggleAccessLevel(
                                server.clone(),
                                channel.clone(),
                                user.nickname().to_owned(),
                                action,
                            )),
                        )
                    } else {
                        (String::new(), None)
                    };

                menu_button(label, message, length, theme)
            }
            (Entry::SendFile, Context::User { server, user, .. }) => {
                let message = Message::SendFile(server.clone(), user.clone());

                menu_button(
                    "Send File".to_string(),
                    Some(message),
                    length,
                    theme,
                )
            }
            (
                Entry::UserInfo,
                Context::User {
                    user, current_user, ..
                },
            ) => user_info(
                current_user,
                user.nickname().to_owned(),
                length,
                config,
                theme,
            ),
            (Entry::HorizontalRule, _) => match length {
                Length::Fill => {
                    container(rule::horizontal(1)).padding([0, 6]).into()
                }
                _ => Space::new(length, 1).into(),
            },
            (Entry::CtcpRequestTime, Context::User { server, user, .. }) => {
                let message = Message::CtcpRequest(
                    ctcp::Command::Time,
                    server.clone(),
                    user.nickname().to_owned(),
                    None,
                );

                menu_button(
                    "Local Time (TIME)".to_string(),
                    Some(message),
                    length,
                    theme,
                )
            }
            (Entry::CtcpRequestVersion, Context::User { server, user, .. }) => {
                let message = Message::CtcpRequest(
                    ctcp::Command::Version,
                    server.clone(),
                    user.nickname().to_owned(),
                    None,
                );

                menu_button(
                    "Client (VERSION)".to_string(),
                    Some(message),
                    length,
                    theme,
                )
            }
            (Entry::CopyUrl, Context::Url(url)) => {
                let message = Message::CopyUrl(url.clone());

                menu_button(
                    "Copy URL".to_string(),
                    Some(message),
                    length,
                    theme,
                )
            }
            (Entry::Timestamp, Context::Timestamp(date_time)) => {
                let message = Message::CopyTimestamp(
                    *date_time,
                    config.buffer.timestamp.copy_format.clone(),
                );

                menu_button(
                    format!(
                        "{}",
                        date_time.with_timezone(&Local).format(
                            &config.buffer.timestamp.context_menu_format
                        )
                    ),
                    Some(message),
                    length,
                    theme,
                )
            }
            _ => row![].into(),
        })
    }
}

#[derive(Clone, Debug)]
pub enum Message {
    Whois(Server, Nick),
    Query(Server, target::Query, BufferAction),
    ToggleAccessLevel(Server, target::Channel, Nick, String),
    SendFile(Server, User),
    InsertNickname(Nick),
    CtcpRequest(ctcp::Command, Server, Nick, Option<String>),
    CopyUrl(String),
    CopyTimestamp(DateTime<Utc>, Option<String>),
}

#[derive(Debug, Clone)]
pub enum Event {
    SendWhois(Server, Nick),
    OpenQuery(Server, target::Query, BufferAction),
    ToggleAccessLevel(Server, target::Channel, Nick, String),
    SendFile(Server, User),
    InsertNickname(Nick),
    CtcpRequest(ctcp::Command, Server, Nick, Option<String>),
    CopyUrl(String),
    CopyTimestamp(DateTime<Utc>, Option<String>),
}

pub fn update(message: Message) -> Event {
    match message {
        Message::Whois(server, nick) => Event::SendWhois(server, nick),
        Message::Query(server, nick, buffer_action) => {
            Event::OpenQuery(server, nick, buffer_action)
        }
        Message::ToggleAccessLevel(server, target, nick, mode) => {
            Event::ToggleAccessLevel(server, target, nick, mode)
        }
        Message::SendFile(server, user) => Event::SendFile(server, user),
        Message::InsertNickname(nick) => Event::InsertNickname(nick),
        Message::CtcpRequest(command, server, nick, params) => {
            Event::CtcpRequest(command, server, nick, params)
        }
        Message::CopyUrl(url) => Event::CopyUrl(url),
        Message::CopyTimestamp(date_time, format) => {
            Event::CopyTimestamp(date_time, format)
        }
    }
}

pub fn user<'a>(
    content: impl Into<Element<'a, Message>>,
    server: &'a Server,
    prefix: &'a [isupport::PrefixMap],
    channel: Option<&'a target::Channel>,
    user: &'a User,
    current_user: Option<&'a User>,
    our_user: Option<&'a User>,
    config: &'a Config,
    theme: &'a Theme,
    click: &'a config::buffer::NicknameClickAction,
) -> Element<'a, Message> {
    let entries = Entry::user_list(channel.is_some(), our_user);

    let message = match click {
        data::config::buffer::NicknameClickAction::OpenQuery => Message::Query(
            server.clone(),
            target::Query::from(user),
            config.actions.buffer.click_username,
        ),
        data::config::buffer::NicknameClickAction::InsertNickname => {
            Message::InsertNickname(user.nickname().to_owned())
        }
    };

    let base = widget::button::transparent_button(content, message);

    context_menu(
        context_menu::MouseButton::default(),
        context_menu::Anchor::Cursor,
        context_menu::ToggleBehavior::KeepOpen,
        base,
        entries,
        move |entry, length| {
            entry.view(
                Some(Context::User {
                    server,
                    prefix,
                    channel,
                    user,
                    current_user,
                }),
                length,
                config,
                theme,
            )
        },
    )
    .into()
}

pub fn timestamp<'a>(
    content: impl Into<Element<'a, Message>>,
    date_time: &'a DateTime<Utc>,
    config: &'a Config,
    theme: &'a Theme,
) -> Element<'a, Message> {
    let entries = Entry::timestamp_list();

    context_menu(
        context_menu::MouseButton::default(),
        context_menu::Anchor::Cursor,
        context_menu::ToggleBehavior::KeepOpen,
        content,
        entries,
        move |entry, length| {
            entry.view(
                Some(Context::Timestamp(date_time)),
                length,
                config,
                theme,
            )
        },
    )
    .into()
}

fn menu_button(
    content: String,
    message: Option<Message>,
    length: Length,
    theme: &Theme,
) -> Element<'static, Message> {
    button(
        text(content)
            .style(theme::text::primary)
            .font_maybe(theme::font_style::primary(theme).map(font::get)),
    )
    .padding(5)
    .width(length)
    .on_press_maybe(message)
    .into()
}

fn right_justified_padding() -> Padding {
    padding::all(5).right(5.0 + double_pass::horizontal_expansion())
}

fn user_info<'a>(
    current_user: Option<&User>,
    nickname: Nick,
    length: Length,
    config: &Config,
    theme: &Theme,
) -> Element<'a, Message> {
    let state = match current_user {
        Some(user) => {
            if user.is_away() {
                Some(
                    text("(Away)")
                        .style(theme::text::secondary)
                        .font_maybe(
                            theme::font_style::secondary(theme).map(font::get),
                        )
                        .width(length),
                )
            } else {
                None
            }
        }
        None => Some(
            text("(Offline)")
                .style(theme::text::secondary)
                .font_maybe(theme::font_style::secondary(theme).map(font::get))
                .width(length),
        ),
    };

    // Dimmed if away or offline.
    let is_user_away = config
        .buffer
        .nickname
        .away
        .is_away(current_user.is_none_or(User::is_away));
    let is_user_offline = config
        .buffer
        .nickname
        .offline
        .is_offline(current_user.is_none());
    let seed = match config.buffer.nickname.color {
        data::buffer::Color::Solid => None,
        data::buffer::Color::Unique => Some(nickname.seed()),
    };

    let style =
        theme::text::nickname(theme, seed, is_user_away, is_user_offline);

    let nickname = text(nickname.to_string()).style(move |_| style).font_maybe(
        theme::font_style::nickname(theme, is_user_offline).map(font::get),
    );

    column![
        container(row![nickname, state].width(length).spacing(4))
            .padding(right_justified_padding())
    ]
    .into()
}
