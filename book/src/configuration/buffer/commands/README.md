# Commands

Commands settings.

- [Commands](#commands)
  - [Configuration](#configuration)
    - [show\_description](#show_description)
  - [Sub-sections](#sub-sections)
    - [Sysinfo](#sysinfo)

## Configuration

### show_description

Show or hide the description for a command

```toml
# Type: boolean
# Values: true, false
# Default: true

[buffer.commands]
show_description = true
```

## Sub-sections

### [Sysinfo](sysinfo.md)

Configure which system information components to display when using the `/sysinfo` command
