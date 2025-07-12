# ChatColor Plugin for Pumpkin

A chat color plugin for Pumpkin servers that allows players to customize their chat and name colors with persistent storage.

## Features

- **Chat Colors**: Set custom colors for your chat messages
- **Name Colors**: Set custom colors for your display name
- **Gradients**: Support for rainbow and fire gradient effects
- **Persistent Storage**: Player colors are saved to YAML files and restored on server restart
- **Permission System**: Control who can use colors with permissions
- **Color Codes**: Support for Minecraft color codes (`&a`, `&b`, `&c`, etc.)

## Commands

### `/chatcolor <color|style>`
Set your default chat color or gradient style.

**Available colors:**
- `black`, `dark_blue`, `dark_green`, `dark_aqua`, `dark_red`, `dark_purple`
- `gold`, `gray`, `dark_gray`, `blue`, `green`, `aqua`, `red`
- `light_purple`, `yellow`, `white`
- `rainbow` - Rainbow gradient effect
- `fire` - Fire gradient effect

**Examples:**
```
/chatcolor red
/chatcolor rainbow
/chatcolor fire
```

### `/namecolor <color|style>`
Set your display name color or gradient style.

**Available colors:** Same as `/chatcolor`

**Examples:**
```
/namecolor blue
/namecolor rainbow
/namecolor fire
```

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

- `chatcolor:command.chatcolor` - Allows players to use the /chatcolor command
- `chatcolor:command.namecolor` - Allows players to use the /namecolor command

## Configuration Files

The plugin creates two YAML files in the `plugins/ChatColor/` directory:

### `config.yml`
```yaml
save_interval: 300  # Save interval in seconds (default: 5 minutes)
auto_save: true     # Enable automatic saving
```

### `data.yml`
```yaml
players:
  "player-uuid-here":
    uuid: "player-uuid-here"
    chat_color: "red"      # or "rainbow", "fire", etc.
    name_color: "blue"     # or "rainbow", "fire", etc.
```

## Data Persistence

- Player colors are automatically saved when changed
- Data is loaded when the server starts
- Data is saved when the server shuts down
- Each player's UUID is used as the unique identifier

## Installation

1. Compile the plugin: `cargo build --release`
2. Copy the compiled library (`libchat_color.so`) to your Pumpkin plugins directory
3. Restart your server
4. The plugin will automatically create the configuration files on first run

## Usage

Players can use color codes in their chat messages:

```
&aHello &bWorld! &lThis is bold!
```

This will display as: **Hello World!** with colors and formatting applied.

## License

This project is licensed under the MIT License. 