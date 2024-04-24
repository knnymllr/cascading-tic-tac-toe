# Cascading Tic Tac Toe

[Downloads & Release notes][github-release-link] | [Contributing to Cascading Tic Tac Toe](#contributing) | [Latest Update](#latest-update)

## Build status

| Architecture | Solution (Main) | Installer (Main) |
|--------------|-----------------|------------------|
| x64          | Working         | Working          |
| darwin       | Working         | Working          |

## About

Cascading Tic Tac Toe is a novel take on the more well-known Tic Tac Toe with a dynamic game board and increased complexity.

|                 | Current Features: |                |
|-----------------|-------------------|----------------|
| Cascading Board | Target Score Mode | Interactive UI |

![This is a mockup of Cascading Tic-Tac-Toe](image.png)

### Rules
1. Games can be played until either a target score reached or a target time is elapsed.
2. A tie or a valid combination of three spaces in a row generates an additional 3x3 board that overlaps the previous board cascading down and to the right.
3. When a space has been used in a valid combination of three it may still be used in another combination of three if it does not create a combination of four or more.

## Installing & Running Cascading Tic Tac Toe

### Requirements

- Windows 11+
- macOS 11+

### Via GitHub Release [Recommended]

Go to the [Cascading Tic Tac Toe GitHub releases page][github-release-link] and click on `Assets` at the bottom to show the files available in the release. Use the appropriate Cascading Tic Tac Toe installer that matches your machine's architecture.

<!-- items that need to be updated with every release -->
[X64]: https://github.com/cis3296s24/cascading-tic-tac-toe/releases/download/v0.5/cascading-tic-tac-toe-0.5-x64.zip
[darwin]: https://github.com/cis3296s24/cascading-tic-tac-toe/releases/download/v0.5/cascading-tic-tac-toe-0.5-dmg.zip
 
| Description | Filename                                    |
|-------------|---------------------------------------------|
| X64         | [cascading-tic-tac-toe-0.5-x64.zip][X64]    |
| darwin      | [cascading-tic-tac-toe-0.5-dmg.zip][darwin] |

## Contributing

This project welcomes contributions of all types. We are excited to work with the rust community to create working examples using Bevy.

### Requirements

- Windows 11+ or macOS 11+
- [Rust 1.77+](https://www.rust-lang.org/tools/install)
- Other dependencies outlined in [Cargo.toml](https://github.com/cis3296s24/cascading-tic-tac-toe/blob/main/cascading-tic-tac-toe/Cargo.toml)

### Fork, Clone, Branch, & Create PR

1. Fork the repo on [GitHub]()
2. Clone fork locally
3. Execute `cargo run` under src directory to run the game
4. Create feature branch
5. Work on changes
6. Create Pull Request

### Contributors 
[Xu Jiang](https://github.com/XJ114514)<br>
[Kenny Miller](https://github.com/knnymllr)<br>
[Paul Sedra](https://github.com/PaulSedra)<br>
[Dongting Gao](https://github.com/gdtmax)

## Latest Update 

### 0.5 - January 2024 Update

In this release, we focused on bug fixes and documentation.

#### Highlights
- continuous generating board by [Kenny Miller](https://github.com/knnymllr) in [#101](https://github.com/cis3296s24/cascading-tic-tac-toe/pull/101)
- Ingame mainmenu bugfix by [Kenny Miller](https://github.com/knnymllr) in [#102](https://github.com/cis3296s24/cascading-tic-tac-toe/pull/102)
- update README.md for macOS users by [Paul Sedra](https://github.com/PaulSedra) in [#103](https://github.com/cis3296s24/cascading-tic-tac-toe/pull/103)

#### What is planned for version 0.6

For v0.6, we'll work on the items below:

--

<!-- Links -->
[github-repo]: https://github.com/cis3296s24/cascading-tic-tac-toe
[github-release-link]: https://github.com/cis3296s24/cascading-tic-tac-toe/releases