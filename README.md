# Directory Alias

A CLI tool for aliasing directories for faster directory switching

## Config

The config file for Dira is located at `$HOME/.config/dira/config.json`

## Usage

```sh

pwd # OUTPUT: /Users/cameron/dev/my-cool-project

dira --alias mcp

dira mcp # OUTPUT: cd "/Users/cameron/dev/my-cool-project" has been added to the clipboard

dira my-cool-project # OUTPUT: Alias does not exist.

dira --list

## OUTPUT:
## {
##    "mcp": "/Users/cameron/dev/my-cool-project"
## }

dira --remove mcp

dira --list # OUTPUT: {}

...

```

It should be noted that as of 0.3.0, when you run dira with the alias you wish to navigate to (ex: `dira mcp`), the command to change to the directory associated with that alias will be added to your clipboard for ease of use.
