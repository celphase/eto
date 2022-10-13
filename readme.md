# eto -- bleh!

A small updater.

![bleh](bleh.png)

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

### eto-package

```
eto-packager --old ./my-old-dir --new ./my-new-dir -p ./package.zip
```

### eto-updater

Place in target directory along with the package zip and run.
Updater automatically finds the package and applies.

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT License (Expat) ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
