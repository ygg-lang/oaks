#[cfg(feature = "disk")]
pub mod disk;
pub mod memory;
#[cfg(feature = "disk")]
pub mod watch;

#[cfg(feature = "disk")]
pub use disk::DiskVfs;
pub use memory::MemoryVfs;
#[cfg(feature = "disk")]
pub use watch::{DiskWatcher, VfsEvent, VfsWatcher};
