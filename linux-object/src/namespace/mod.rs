//#[cfg(feature = "namespace")]
#![deny(missing_docs)]
//This mod is try to let zcore have namespace and cgroup
mod namespace;
mod cgroup;

use rcore_fs::vfs::{FileSystem, FileType, INode, Result};
use rcore_fs_devfs::{
    special::{NullINode, ZeroINode},
    DevFS,
};
use rcore_fs_mountfs::MountFS;
use rcore_fs_ramfs::RamFS;

//there must have one struct to manage all the namespaces

pub struct ns_manager{
    
}