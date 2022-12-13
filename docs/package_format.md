# Package Format

## Base container format

A package file contains, in order:

- header
- manifest data block
- any amount of additional data blocks

Numbers written in binary format always use little-endian byte ordering.

### Header

| Bytes | Description            |
| ----- | ---------------------- |
| 8     | Magic number "EtoPack1" |

### Data block

| Bytes | Description              |
| ----- | ------------------------ |
| 4     | Size of content in bytes |
| ...   | Content                  |

## Manifest data block

Manifest JSON in UTF-8.

```json5
{
  // Semantic version, used while reading to check compatibility.
  version: "0.1.0",
  diff: {
    /* ... */
  },
}
```

## Files data block

Files as a zstd compressed tar.

## Extension

The `.etopack` extension should be used where possible.
