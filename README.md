# ext4-rw

[![no_std](https://img.shields.io/badge/no__std-yes-blue)](https://rust-embedded.github.io/book/)
[![crates.io](https://img.shields.io/crates/v/ext4-rw.svg)](https://crates.io/crates/ext4-rw)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](LICENSE-MIT)

A `no_std` ext4 filesystem implementation in Rust with full read and write support.

Designed for bare-metal, embedded, and OS development environments where the standard
library is not available. Requires only `alloc`.

## Features

- **Superblock** parsing and serialization with full ext4 field support
- **Block group descriptor** table management (32-bit and 64-bit modes)
- **Inode** reading and writing with extent tree support
- **Extent tree** traversal (leaf and multi-level index nodes)
- **Directory** entry parsing, creation, lookup, and linear iteration
- **Bitmap** allocation for blocks and inodes (single, contiguous runs, free ranges)
- **High-level API**: mount, read files, write files, create directories, list directories
- **`BlockDevice` trait** for pluggable storage backends (NVMe, virtio-blk, RAM disk, etc.)
- **`no_std`** compatible -- only depends on `alloc` and `log`
- **64-bit** block number support (INCOMPAT_64BIT)

## Usage

Add to your `Cargo.toml`:

```toml
[dependencies]
ext4-rw = "0.1"
```

Implement the `BlockDevice` trait for your storage backend:

```rust
use ext4_rw::{Ext4Fs, BlockDevice, Ext4Error};

struct RamDisk {
    data: Vec<u8>,
}

impl BlockDevice for RamDisk {
    fn read_bytes(&self, offset: u64, buf: &mut [u8]) -> Result<(), Ext4Error> {
        let start = offset as usize;
        let end = start + buf.len();
        buf.copy_from_slice(&self.data[start..end]);
        Ok(())
    }

    fn write_bytes(&self, offset: u64, buf: &[u8]) -> Result<(), Ext4Error> {
        // For a real implementation, write to your backing store
        Ok(())
    }
}

// Mount and use the filesystem
let disk = RamDisk { data: load_disk_image() };
let mut fs = Ext4Fs::mount(disk).expect("failed to mount ext4");

// Read a file
let data = fs.read_file(b"/etc/hostname").expect("read failed");

// Write a file
fs.write_file(b"/output.txt", b"Hello from ext4-rw!").expect("write failed");

// Create a directory
fs.mkdir(b"/mydir").expect("mkdir failed");

// List directory contents
let entries = fs.list_dir(b"/").expect("list failed");
for entry in &entries {
    println!("{} (inode {})", entry.name_str(), entry.inode);
}

// Persist metadata changes
fs.sync_metadata().expect("sync failed");
```

## Limitations

- **No journaling**: journal replay is not implemented. Mount only clean filesystems.
- **No HTree**: directory lookup is linear scan (fine for small/medium directories).
- **No legacy block map**: only extent-based files are supported (standard for ext4).
- **No checksums**: metadata checksums are not verified or computed.
- **No encryption**: encrypted inodes are not supported.
- **Extent tree splitting**: files larger than 4 extents in the root node are not yet supported for writes.

These are planned for future releases.

## Minimum Supported Rust Version

Rust 1.70 or later.

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT License ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

## Contributing

Contributions are welcome! Please open an issue or pull request on
[GitHub](https://github.com/suhteevah/ext4-rw).

---

---

---

---

---

---

---

---

---

---

---

## Support This Project

If you find this project useful, consider buying me a coffee! Your support helps me keep building and sharing open-source tools.

[![Donate via PayPal](https://img.shields.io/badge/Donate-PayPal-blue.svg?logo=paypal)](https://www.paypal.me/baal_hosting)

**PayPal:** [baal_hosting@live.com](https://paypal.me/baal_hosting)

Every donation, no matter how small, is greatly appreciated and motivates continued development. Thank you!
