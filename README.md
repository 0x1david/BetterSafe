# BetterSafe
BetterSafe is a CLI tool developed to protect you from making dumb, irrevertable changes with a single command.

It does this by archiving files you remove for a certain time, the more dangerous the action, the longer time it takes to remove it from the archive permanently.

By default arguments are similar to classic bash rm command - *Force*, *Interactive*, *Recursive*, *Dir*, *Verbose*

> **Disclaimer:** This project is principally intended for learning purposes as a newcomer to Rust. Contributions, advice, or suggestions are warmly welcomed, but please note, certain optimizations and improvements might never be delivered due to the learning-focused nature of this project.

## 🚀 Usage

BetterSafe offers several command-line options to manage files effectively:

- **`--help` or `-h`**: Display the help message.
- **`--version` or `-v`**: Display the current version of the tool.
- **`--trash` or `-t`**: Move a file or folder to trash.
- **`--restore`**: Restore a previously trashed file or folder.
- **`--archive` or `-a`**: Archive a file or folder.
- **`--portal` or `-p`**: (Describe functionality).
- **`--force` or `-f`**: Force an action.
- **`--portal`**: Move to equivalent archive directory or its parent if it doesn't exist.
- **`--interactive` or `-i`**: Run in interactive mode.
- **`--recursive` or `-r`**: Perform actions recursively on directories.
- **`--directory` or `-d`**: Perform actions on a directory.
- **`--verbose`**: Run in verbose mode.

### 📖 Example

To use the tool with a specific command:
```sh
$ bettersafe --trash /path/to/file
```
## 🌟 Contributing

Anyone is welcome to contribute to BetterSafe! Whether it's reporting bugs, discussing improvements, or contributing code, we welcome you to join us!

- **To report a bug or discuss a feature**: Open a [new issue](https://github.com/0x1_david/BetterSafe/issues/new).
- **To contribute code**: 
  1. Fork the project.
  2. Create a new branch.
  3. Make your changes.
  4. Open a pull request.


## ⚖️ License

BetterSafe is unlicensed, which means you are free to use, modify, and distribute the project in any form, without any restrictions. Use it at your will!

## 🙏 Acknowledgments

My gf for letting me code all day fr lol

