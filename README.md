# Thalassophobia
A 3D submarine game set on a hostile alien planet. Running on the [Bevy](https://bevyengine.org/) game engine.

## Building
See [here](https://bevyengine.org/learn/quick-start/getting-started/setup/#installing-os-dependencies) for dependencies needed by Bevy.

The `client` package has several feature flags to disable parts of the game. All are enabled by default.
- `multiplayer` - Allows the client to join, but not host, multiplayer lobbies.
- `hosting` - Allows the client to host multiplayer lobbies. Implies `multiplayer`.

## Contributing
Before contributing, please read the [licensing](#license) section to understand how your work will be used.

The following table describes where you can go to contribute to specific parts of the game.

| Folder          | Project component       | GitHub project link                                          |
| --------------- | ----------------------- | ------------------------------------------------------------ |
| `crates/client` | Game client source      | [Client crate](https://github.com/users/Veritius/projects/8) |
| `crates/dedi`   | Dedicated server source | [Dedi crate](https://github.com/users/Veritius/projects/9/)  |
| `crates/server` | Shared server source    | [Server crate](https://github.com/users/Veritius/projects/7) |
| `crates/shared` | Shared game source      | [Shared crate](https://github.com/users/Veritius/projects/6) |
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