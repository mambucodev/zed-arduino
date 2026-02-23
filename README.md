# Arduino for Zed

Arduino language support for [Zed](https://zed.dev), with syntax highlighting and [arduino-language-server](https://github.com/arduino/arduino-language-server) integration.

## Features

- Dedicated Arduino language for `.ino` files (instead of C++)
- Syntax highlighting via [tree-sitter-arduino](https://github.com/tree-sitter-grammars/tree-sitter-arduino)
- Code outline, bracket matching, indentation, and text objects
- Auto-download of `arduino-language-server` and `arduino-cli`

## Requirements

The language server requires [clangd](https://clangd.llvm.org/) installed on your system:

- **macOS**: `brew install llvm`
- **Ubuntu/Debian**: `sudo apt install clangd`
- **Arch**: `sudo pacman -S clang`
- **Fedora**: `sudo dnf install clang-tools-extra`

## Configuration

To get full LSP support (diagnostics, autocomplete, go-to-definition), set your board's FQBN in `.zed/settings.json`:

```json
{
  "lsp": {
    "arduino-language-server": {
      "binary": {
        "arguments": ["-fqbn", "arduino:avr:uno"]
      }
    }
  }
}
```

Replace `arduino:avr:uno` with your board. Run `arduino-cli board listall` to find available FQBNs.
