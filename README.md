# KDE Color Scheme Matcher

This utility reads the `AccentColor` setting from your KDE configuration and sets various other color settings across your computer.

## Intro

KDE comes with the nifty ability to set your wallpaper to various random/photo of the days from across the internet.

It also has the ability to automatically set your system accent color to match that wallpaper.

While KDE has an extensive catalogue of applications and programs in its ecosystem, there will always be a time when you find something outside of its walls you prefer, such as a special terminal emulator or IDE, etc.

The aim of this utility is to extend the KDE color scheme to programs outside of the KDE ecosystem.

## Currently Supported Setups

Currently, this utility only supports [Starship](https://github.com/starship/starship) running with a configuration that takes a palette ([See this repo for an example](https://github.com/PercyJax/dotfiles)) and [Alacritty](https://github.com/alacritty/alacritty) that imports a `palette.yml`, but it can easily be extended to update the colors of various other configuration files.

- KDE
  - This tool currently only supports pulling color values from a KDE config file located at `~/.config/kdeglobals`
- Starship
  - Config file currently needs to be located at `~/.config/starship.toml`
  - Its design should make use of various shades of the same hue
  - [palettes.primary] the following values will be updated when this tool is run:
    - `shade1` (lightest) through `shade5` (darkest)
    - `text1` (darkest) through `text4` (lightest)
- Alacritty
  - Pallet config file currently needs to be located at `~/.config/alacritty/palette.yml`

## Automating the Color Changing

In progress. In the meantime, you should try installing a systemd path unit to [monitor for a file change at `~/.config/kdeglobals`](https://unix.stackexchange.com/questions/391613/monitor-file-with-systemd-service).