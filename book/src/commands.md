# Commands

Commands in Halloy are prefixed with `/`.

- [Commands](#commands)
  - [Example](#example)
  - [Types](#types)

## Example

```sh
/me says halloy!
```

## Types

Halloy will first try to run below commands, and lastly send it directly to the server.
The argument(s) for a command are shown in [tooltips](configuration/tooltips.md), and those marked with a `*` will show an additional tooltip with argument-specific information on mouseover.

| Command       | Alias      | Description                                                                        |
| ------------- | ---------- | ---------------------------------------------------------------------------------- |
| `away`        |            | Mark yourself as away. If already away, the status is removed                      |
| `chathistory` |            | Retrieve message history[^5]                                                       |
| `clear`       |            | Clear the message history in the current buffer                                    |
| `cleartopic`  | `ct`       | Clear the topic of a channel[^1]                                                   |
| `ctcp`        |            | Client-To-Client requests[^2]                                                      |
| `delay`       |            | Delay the specified number of seconds[^7]                                          |
| `detach`      |            | Hide the channel, but leave the bouncer's connection to the channel active[^5][^6] |
| `format`      | `f`        | Format text with markdown and colors                                               |
| `hop`         | `rejoin`   | Part the current channel and join a new one                                        |
| `join`        | `j`        | Join channel(s) with optional key(s)                                               |
| `kick`        |            | Kick a user from a channel[^1]                                                     |
| `knock`       |            | Request an invite from an invitation-only channel[^5]                              |
| `list`        |            | List channel(s) on the server[^5]                                                  |
| `me`          | `describe` | Send an action message to the channel                                              |
| `mode`        | `m`        | Set mode(s) on a channel or retrieve the current mode(s) set[^3]                   |
| `monitor`     |            | System to notify when users become online/offline[^5]                              |
| `motd`        |            | Request the message of the day                                                     |
| `msg`         | `query`    | Open a query with a nickname and send an optional message                          |
| `nick`        |            | Change your nickname on the current server                                         |
| `notice`      |            | Send a notice message to a target                                                  |
| `part`        | `leave`    | Leave and close channel(s)/quer(ies) with an optional reason [^4]                  |
| `quit`        |            | Disconnect from the server with an optional reason                                 |
| `raw`         |            | Send data to the server without modifying it                                       |
| `setname`     |            | Change your realname[^5]                                                           |
| `sysinfo`     |            | Send system information (OS, CPU, memory, GPU, uptime)                             |
| `topic`       | `t`        | Retrieve the topic of a channel or set a new topic[^1]                             |
| `whois`       |            | Retrieve information about user(s)                                                 |

[^1]: The `channel` argument can be skipped when used in a channel buffer to target the channel in the buffer.
[^2]: The `nick` argument can be skipped when used in a query buffer to target the other user in the buffer.
[^3]: The `target` argument can be skipped; in a channel buffer it will target the channel in the buffer, in a query buffer it will target the other user in the buffer, and in a server buffer it will target your user.
[^4]: The `targets` argument can be skipped; in a channel or query buffer it will target the current buffer.
[^5]: Command must be supported by the bouncer/server to be executed successfully; if not supported then the command will not appear in the command picker.
[^6]: See [soju](https://soju.im/)'s [documentation on detaching from channels](https://man.sr.ht/chat.sr.ht/bouncer-usage.md#detaching-from-channels) for more information.
[^7]: Can only be used in [on_connect](./configuration/servers/index.md#on_connect).
