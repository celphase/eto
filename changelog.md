# Changelog

## 0.2.2

- *Actually* fix it, whoops.

## 0.2.1

- Fix deletion not being done relative to the target directory.

## 0.2.0

- **breaking**: Change package format to JSON + custom GZip + tar data blocks.
- Add specifying package location to `eto apply`.
- Add `eto list` subcommand.
- Add `wait-for` and `on-complete` flags on `apply` subcommand.
- Add specific required `target` flag to `apply` subcommand.
- Replace separate eto binaries with sub-commands.
- Remove "not_running" feature, `eto-updater` (now `eto apply`) should no longer be run directly by
  a user, an external batch script should check for this instead if you still want this
  functionality.

## 0.1.4

- Improve error handling on missing `eto.json`.

## 0.1.3

- Add `not_running` to metadata.

## 0.1.2

- Fix filesystem errors not being correctly passed up in state scans.
- Fix compression errors not being correctly passed up.
- Fix parent directory creating potentially crashing.

## 0.1.1

- Ignore file deletions that don't exist in the first place.

## 0.1.0

Initial version.
