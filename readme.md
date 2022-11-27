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

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT License (Expat) ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
