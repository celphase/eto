# eto -- bleh!

A small updater.

![bleh](bleh.png)

## Installation

For Windows, pre-built binaries are provided in the GitHub releases section.

If you are on another system, or want to build eto from scratch, you can install the tools using
"cargo". The best way to get "cargo" is using [rustup](https://rustup.rs/).

```sh
cargo install --release
```

## Usage

To make sure the old and new directories are correct, you need to create an `eto.json` file, that
contains some metadata.

```json
{
  "version": "1.0",
  "ignore": []
}
```

`version` will be checked by the updater when applying a package.
Files matching glob patterns in `ignore` will be ignored when scanning the state of a directory.

### Creating a Package

```
eto package -a ./my-old-dir -b ./my-new-dir -o ./package.etopack
```

### Applying a Package

If you are an end-user of software that uses eto, check its documentation for applying packages, as
this can differ depending on the setup.

If you are a developer, check the `help` documentation for the `eto patch` subcommand.
You may want to provide a batch script to your users for doing this automatically, as most users are
not comfortable using the command line.
You can also apply a patch programmatically using the eto library, or call `eto patch` from your
software, and have it wait for your software to close and restart it afterwards.

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT License (Expat) ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
