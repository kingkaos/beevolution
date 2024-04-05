# beevolution

`beevolution` is a desktop application (MacOS only) which provides a user interface to handle bee databases. It should visualize time critical bee and hive dates.

Settings are saved in a separate file located at `~/.config/beevolution/config.json`. This file will be created every time if not found at the startup of `beevolution`.

## Development

`beevolution` is a tauri application which uses Rust in the back and Vue in the front (see also folder names).
