# Directory Alias

A CLI tool for aliasing directories for faster directory switching

## Config

The config file for Dira is located at `$HOME/.config/dira/config.json`

## Usage

```sh

pwd # OUTPUT: /Users/cameron/dev/my-cool-project

dira --alias mcp

dira mcp # OUTPUT: cd "/Users/cameron/dev/my-cool-project"

dira my-cool-project # OUTPUT: Alias does not exist.

...

```