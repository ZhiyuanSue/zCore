//#[cfg(feature = "namespace")]
use bitflags::bitflags;
use super::*;
pub struct NsBase{
    base: KObjectBase,
    nstype:u8,  //it should use the namespace.rs::NSType
    parent:Option<Arc<NsBase>>,   //the parent might be none,so use option
    child_ns_vec:Arc<Vec<NsBase>>,
}
pub trait NS :{
    fn get_ns_id(&self)->KoID;
    fn get_ns_type(&self)->NSType;
    fn get_ns_base(&self)->NsBase;
    fn get_parent_ns(&self)->KoID;
}
pub struct MntNs
{
    base:NsBase,
    rootfs:Inode,
}
pub struct UtsNs
{
    base:NsBase,
}
pub struct IpcNs
{
    base:NsBase,
}
pub struct PidNs
{
    base:NsBase,
}
pub struct NetNs
{
    base:NsBase,
}
pub struct UsrNs
{
    base:NsBase,
}

pub enum NsEnum{
    MntNs(MntNs),
    UtsNs(UtsNs),
    IpcNs(IpcNs),
    PidNs(PidNs),
    NetNs(NetNs),
    UsrNs(UsrNs),
}


//https://man7.org/linux/man-pages/man2/setns.2.html
bitflags! {
    pub struct NSType:u8{
        const NSTypeAny =           0;
        const CLONE_NEWCGROUP =     1;
        const CLONE_NEWIPC =     1<<1;
        const CLONE_NEWNET =     1<<2;
        const CLONE_NEWNS =      1<<3;
        const CLONE_NEWPID =     1<<4;
        const CLONE_NEWTIME =    1<<5;
        const CLONE_NEWUSER =    1<<6;
        const CLONE_NEWUTS =     1<<7;
    }
}