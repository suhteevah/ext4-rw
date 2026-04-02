//! # ext4-rw
//!
//! A `no_std` ext4 filesystem implementation in Rust with read and write support.
//!
//! This crate provides read and write access to ext4 filesystems, suitable for
//! bare-metal, embedded, and OS development environments. It requires only `alloc`
//! and has no dependency on `std`.
//!
//! ## Features
//!
//! - Superblock parsing and validation
//! - Block group descriptor table management (32-bit and 64-bit)
//! - Inode reading and writing with extent tree support
//! - Directory entry parsing, creation, and lookup
//! - Block and inode bitmap allocation
//! - High-level file read/write/create/mkdir API
//! - Linear directory scanning
//!
//! ## Usage
//!
//! ```rust,no_run
//! use ext4_rw::{Ext4Fs, BlockDevice, Ext4Error};
//!
//! struct MyDisk { /* ... */ }
//!
//! impl BlockDevice for MyDisk {
//!     fn read_bytes(&self, offset: u64, buf: &mut [u8]) -> Result<(), Ext4Error> {
//!         // Read from your storage backend
//!         Ok(())
//!     }
//!     fn write_bytes(&self, offset: u64, buf: &[u8]) -> Result<(), Ext4Error> {
//!         // Write to your storage backend
//!         Ok(())
//!     }
//! }
//!
//! let disk = MyDisk { /* ... */ };
//! let fs = Ext4Fs::mount(disk).expect("failed to mount ext4");
//! let data = fs.read_file(b"/etc/hostname").expect("read failed");
//! ```

#![no_std]
#![warn(missing_docs)]

extern crate alloc;

pub mod bitmap;
pub mod block_group;
pub mod dir;
pub mod extent;
pub mod inode;
pub mod readwrite;
pub mod superblock;

pub use readwrite::{BlockDevice, Ext4Fs, Ext4Error};
pub use superblock::Superblock;
pub use block_group::BlockGroupDesc;
pub use inode::Inode;
pub use dir::DirEntry;
pub use extent::{ExtentHeader, ExtentIndex, ExtentLeaf};
pub use bitmap::BitmapAllocator;
