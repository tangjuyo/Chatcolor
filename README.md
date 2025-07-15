# ChatColor Plugin

A minimalist Pumpkin plugin that allows OP players to customize their chat and/or name color on your Minecraft server.

## SETUP
- when running for the first time the plugin, config.yml and data.yml will be created.
- You need to copy paste the config.yml in the github to your config.yml ( the default one will not work correctly )

## How it works
- colors are created on the config.yml so you can play and create your own set of colors and gradients

## Incoming Features
- **Only OPs** Right now only ops can use `/chatcolor` and `/namecolor`.
- **No default color**: if a player has not chosen a color, their chat and name remain vanilla (unmodified).
- **Independent settings**: a player can set only chat color, only name color, or both. ( might add group or stuff like that )
- **custom permissions**: Right now only native Pumpkin permissions ( OPs only ).
- **tab-completion** for colors.
- **Fixing the config.yml**: the default generation config.yml isnt working properly

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

## Dependencies
- Pumpkin (Rust Minecraft server)

## Author
- Plugin maintained by tangjuyo