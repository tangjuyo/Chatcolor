# ChatColor Plugin for Pumpkin

A simple chat color plugin for Pumpkin servers that allows players to use color codes in their messages.

## Features

- **Color Codes**: Support for Minecraft color codes (`&a`, `&b`, `&c`, etc.)
- **Formatting Codes**: Support for formatting codes (`&l`, `&n`, `&o`, etc.)
- **Permission System**: Control who can use colors with permissions
- **Configurable**: Easy to configure and customize

## Color Codes

| Code | Color | Name |
|------|-------|------|
| `&0` | Black | Black |
| `&1` | Dark Blue | Dark Blue |
| `&2` | Dark Green | Dark Green |
| `&3` | Dark Aqua | Dark Aqua |
| `&4` | Dark Red | Dark Red |
| `&5` | Dark Purple | Dark Purple |
| `&6` | Gold | Gold |
| `&7` | Gray | Gray |
| `&8` | Dark Gray | Dark Gray |
| `&9` | Blue | Blue |
| `&a` | Green | Green |
| `&b` | Aqua | Aqua |
| `&c` | Red | Red |
| `&d` | Light Purple | Light Purple |
| `&e` | Yellow | Yellow |
| `&f` | White | White |

## Formatting Codes

| Code | Effect |
|------|--------|
| `&l` | Bold |
| `&n` | Underlined |
| `&o` | Italic |
| `&m` | Strikethrough |
| `&k` | Obfuscated |
| `&r` | Reset |

## Permissions

- `chatcolor.use` - Allows players to use color codes in chat
- `chatcolor.admin` - Allows players to use all color codes (including admin colors)

## Usage

Players can use color codes in their chat messages:

```
&aHello &bWorld! &lThis is bold!
```

This will display as: **Hello World!** with colors and formatting applied.

## Installation

1. Compile the plugin: `cargo build --release`
2. Copy the compiled library to your Pumpkin plugins directory
3. Restart your server

## Configuration

The plugin can be configured through the server configuration or by editing the plugin's config file.

## License

This project is licensed under the MIT License. 