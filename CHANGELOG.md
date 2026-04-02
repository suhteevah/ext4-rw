# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2026-04-02

### Added

- Superblock parsing and serialization with full ext4 field support
- Block group descriptor table management (32-bit and 64-bit modes)
- Inode reading and writing with extent tree support
- Extent tree traversal (leaf and multi-level index nodes)
- Directory entry parsing, creation, lookup, and iteration
- Block and inode bitmap allocation (single, contiguous, free range)
- High-level `Ext4Fs` API:
  - `mount()` - mount an ext4 filesystem from a `BlockDevice`
  - `read_file()` - read file contents by absolute path
  - `write_file()` - create or overwrite a file
  - `mkdir()` - create directories with `.` and `..` entries
  - `list_dir()` - list directory contents
  - `lookup_path()` - resolve absolute paths to inodes
  - `sync_metadata()` - flush superblock and block group descriptors
- `BlockDevice` trait for pluggable storage backends
- Full `no_std` support (requires only `alloc`)
