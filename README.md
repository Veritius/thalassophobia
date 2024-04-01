# Thalassophobia
A 3D submarine game set on a hostile alien planet. Running on the [Bevy](https://bevyengine.org/) game engine.

## Building
The minimum supported Rust version is currently **nightly Rust** due to the use of `"-Zshare-generics=off"`.

## Contributing
Before contributing, please read the [licensing](#license) section to understand how your work will be used.

The following table describes where you can go to contribute to specific parts of the game.

| Folder          | Project component       |
| --------------- | ----------------------- |
| `crates/client` | Game client source      |
| `crates/dedi`   | Dedicated server source |
| `crates/server` | Shared server source    |
| `crates/shared` | Shared game source      |
| `assets`        | Minimal game assets     |
| `packages/core` | Core content package    |

## License
Thalassophobia is free and open-source software, and will stay that way forever. Each part of this project contains licensing information in files named `README.md` - make sure you read them! The below table also documents which sections are under what license, though you should still read the licensing section of each README.

You can click on the license name to open a copy, or if viewing this document as plain text, go to the bottom of the page to find links.

| Path       | License                              |
| ---------- | ------------------------------------ |
| `crates`   | [Mozilla Public License 2.0]         |
| `assets`   | [Creative Commons BY-SA 4.0 License] |
| `packages` | [Mixed licenses][package-licensing]  |

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work, will be licensed as above. All assets included in the work are credited in [CREDITS.md](./CREDITS.md).

<!-- Read this file for more information about how the packages folder is licensed -->
[package-licensing]: ./packages/README.md#licensing

<!-- Links to the licenses -->
[Mozilla Public License 2.0]: ./LICENSE-MPL
[Creative Commons BY-SA 4.0 License]: ./LICENSE-CC