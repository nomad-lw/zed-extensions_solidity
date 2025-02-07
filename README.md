# Zed Solidity

A [Solidity](https://soliditylang.org/) extension for [Zed](https://zed.dev).

This extension adds support for the Solidity programming language, providing syntax highlighting and language server functionality for smart contract development.

## Features

- Syntax highlighting for solidity files
- Language server support via [vscode-solidity-server](https://www.npmjs.com/package/vscode-solidity-server)
- Auto-installation and updates of the language server
- Independent from framework dependencies
- Code completion and diagnostics (broken)

## Development

To develop this extension, see the [Developing Extensions](https://zed.dev/docs/extensions/developing-extensions) section of the Zed docs.

## Initialization Options

You can configure the language server behavior through initialization options in your Zed settings:
> Reminder! This is mostly untested WIP, barely any settings have been tested.

```jsonc
{
  "lsp": {
    "solidity": {
      "initialization_options": {
        "solidity": {
          // Path to local solc binary (optional)
          "compilerPath": "/usr/local/bin/solc",

          // Compiler optimization settings
          "optimizer": {
            "enabled": true,
            "runs": 200
          },

          // Enable/disable certain features
          "enabledFeatures": {
            "completion": true,
            "hover": true,
            "diagnostics": true,
            "codeActions": true
          }
        }
      }
    }
  }
}
```

## Requirements

- Node.js (for the language server)
- Zed editor

## Status

This is an initial developmental version with basic functionality. Future updates will include:

- Better integration with development frameworks (hardhat/forge)
- Import resolution
- Enhanced type checking

## Contributing

Feel free to open issues and submit pull requests. All contributions are welcome!

## Credits

- JoranHonig Tree-sitter grammar: [tree-sitter-solidity](https://github.com/JoranHonig/tree-sitter-solidity)
- Juan Blanco's VScode Solidity Language Server: [vscode-solidity-server](https://www.npmjs.com/package/vscode-solidity-server)

## License

<p align="center">
This project is licensed under the GNU General Public License v3.0 - see the <a href="LICENSE.md">LICENSE.md</a> file for details.
</p>

<p align="center">
	Copyright &copy; 2025-present <a href="https://github.com/nomad-lw" target="_blank">Sambot</a> (<a href="mailto:0xsambot@protonmail.com">0xsambot@protonmail.com</a>)
</p>
