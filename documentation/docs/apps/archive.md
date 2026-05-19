# Archive

Archive extraction and compression tool supporting **ZIP**, **TAR**, **GZ**, **BZ2**, and **XZ** formats.

<br>

## Features

- Open archive from Finder via right-click → Extract
- List contents with file sizes, compression ratio, and modification dates
- Extract to chosen destination directory
- **Path traversal protection** — all extraction paths validated
- Create ZIP archives from selected files/folders

<br>

## Backend

| Format | Extract Command | Compress Command |
|--------|----------------|------------------|
| ZIP | `unzip -o` | `zip -r` |
| TAR | `tar -xf` | `tar -cf` |
| Gzip TAR | `tar -xzf` | `tar -czf` |
| Bzip2 TAR | `tar -xjf` | `tar -cjf` |
| XZ TAR | `tar -xJf` | `tar -cJf` |

All extractions are path-traversal-checked via `isWithin(safeBase, targetPath)` to prevent symlink/slipstream attacks.

<br>

## Related

- [Filesystem API](../apis/fs-api.md)
- [Security Architecture](../architecture/security.md)
- [Finder App](finder.md)

<br>

---

[← Back: Documentation Index](../index.md)
