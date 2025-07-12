# ChatColor Plugin

A minimalist Pumpkin plugin that allows OP players to customize their chat and/or name color on your Minecraft server.

## Features
- **Only OPs** can use `/chatcolor` and `/namecolor`.
- **No default color**: if a player has not chosen a color, their chat and name remain vanilla (unmodified).
- **Independent settings**: a player can set only chat color, only name color, or both.
- **No custom permissions**: only native Pumpkin permissions, OPs only.
- **No tab-completion** for colors.
- **No forced color on join**: the plugin does nothing until a player chooses a color.

## Usage

- `/chatcolor <color|gradient>`: change your chat message color.
- `/namecolor <color|gradient>`: change your name color in chat.

Examples:
```
/chatcolor red
/chatcolor rainbow
/namecolor ocean
```

## Configuration

The `config.yml` file defines available simple colors and gradients:

```yaml
simple_colors:
  red: "§c"
  blue: "§9"
  green: "§a"
  # ...
gradients:
  rainbow:
    type: "hsv"
    start_hue: 0.0
    end_hue: 360.0
    saturation: 1.0
    value: 1.0
  fire:
    type: "rgb"
    colors:
      - [255, 0, 0]
      - [255, 165, 0]
      - [255, 255, 0]
  # ...
```

## How it works
- If the player has not chosen a color, the plugin **does not intercept** chat: vanilla behavior is preserved.
- If the player has set a chat or name color, only the chosen field is modified.
- Colors are persisted in `data.yml`.

## Robustness
- No crash possible if the config is incomplete or the player has no data.
- The plugin never blocks vanilla chat.

## Dependencies
- Pumpkin (Rust Minecraft server)

## Author
- Plugin adapted and maintained for Pumpkin by [your name here] 