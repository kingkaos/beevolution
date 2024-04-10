# beevolution

`beevolution` is a desktop application (MacOS only) which provides a user interface to handle bee databases. It should visualize time critical bee and hive dates.

Settings are saved in a separate file located at `~/.config/beevolution/config.json`. This file will be created every time if not found at the startup of `beevolution`. The back part ensures existence of the store file which can be accessed from front part.

If the store file is incomplete, the missing components are set with default values. The application will panic if initialization fails.

## Development

`beevolution` is a tauri application which uses Rust in the back and Vue in the front (see also folder names). The most recent tauri version is used (currently: 2.0.0) which is unstable and might lead to unexpected behaviour.

### Preparation

Install necessary frontend modules by running from root directory:

```sh
npm install
```

Rust dependencies are loaded upon running `cargo build` from `back` folder or running `npm run tauri dev` from root directory. `npm run tauri dev` builds everything in dev mode and runs cargo build.
