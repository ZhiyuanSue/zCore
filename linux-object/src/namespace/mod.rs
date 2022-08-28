//#[cfg(feature = "namespace")]
//This mod is try to let zcore have namespace and cgroup
mod cgroupns;
mod ipcns;
mod mntns;
mod netns;
mod pidns;
mod usrns;
mod utsns;

use super::*;
use lazy_static::*;
use cgroupns::*;
use ipcns::*;
use mntns::*;
use netns::*;
use pidns::*;
use usrns::*;
use utsns::*;

use alloc::vec::Vec;
use alloc::string::String;
use alloc::sync::Arc;
use lock::mutex::Mutex;
use hashbrown::HashMap;
use zircon_object::object::KObjectBase;
use core::convert::TryFrom;

pub type KoID = u64;
pub struct NsManager{
    ns_hash:HashMap<KoID,Arc<dyn NS + Send + Sync>>,
    init_ns:KoID,
}
impl NsManager{
    pub fn init()->Arc<Self>{
        Arc::new(NsManager{
            ns_hash:HashMap::new(),
            init_ns:0,
        })
    }
    pub fn get_root(self)->KoID{
        self.init_ns
    }
    pub fn get_ns(&self,ns_id:KoID)->Option<&Arc<dyn NS + Send + Sync>>
    {
        match self.ns_hash.get(&ns_id)
        {
            Some(ns) => {
                return Some(ns);
            },
            None => {return None;},
        }
    }
    pub fn insert(mut self,ns: Arc<dyn NS + Send + Sync>)->KoID
    {
        let id=ns.get_ns_id();
        self.ns_hash.insert(id,ns);
        id
    }
}
lazy_static!{
    pub static ref NS_MANAGER:Arc<NsManager>= NsManager::init();
}
#[derive(Default)]
pub struct NsProxy{
    mnt_ns:     KoID,
    uts_ns:     KoID,
    ipc_ns:     KoID,
    pid_ns:     KoID,
    net_ns:     KoID,
    usr_ns:     KoID,
    cgroup_ns:  KoID,
}
impl NsProxy{
    pub fn new()->Self
    {
        NsProxy{
            mnt_ns:0,
            uts_ns:0,
            ipc_ns:0,
            pid_ns:0,
            net_ns:0,
            usr_ns:0,
            cgroup_ns:0,
        }
    }
    pub fn change_proxy(mut self,ns:NSType,id:KoID)
    {
        match ns{
            NSType::NSTYPE_ANY          =>  (),
            NSType::CLONE_NEWCGROUP     =>  self.cgroup_ns=id,
            NSType::CLONE_NEWIPC        =>  self.ipc_ns=id,
            NSType::CLONE_NEWNET        =>  self.net_ns=id,
            NSType::CLONE_NEWNS         =>  self.mnt_ns=id,
            NSType::CLONE_NEWPID        =>  self.pid_ns=id,
            NSType::CLONE_NEWTIME       =>  (),
            NSType::CLONE_NEWUSER       =>  self.usr_ns=id,
            NSType::CLONE_NEWUTS        =>  self.uts_ns=id,
            _=>()
        }
    }
}
//#[cfg(feature = "namespace")]
use bitflags::bitflags;
//https://man7.org/linux/man-pages/man2/setns.2.html
bitflags! {
    pub struct NSType:u8{
        const NSTYPE_ANY =           0;
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
pub struct NsBase{
    base: KObjectBase,
    nstype:NSType,  //it should use the namespace.rs::NSType
    parent:Option<KoID>,   //the parent might be none,so use option
    child_ns_vec:Arc<Mutex<Vec<KoID>>>,
}
impl NsBase{
    pub fn new(nstype:NSType,parent:Option<KoID>)->Self{
        NsBase { 
            base: KObjectBase::new(), 
            nstype: nstype,
            parent: parent,
            child_ns_vec: Arc::new(Mutex::new(Vec::new())),
        }
    }
}
pub trait NS :Send + Sync{
    fn get_ns_id(&self)->KoID;
    fn get_ns_type(&self)->NSType;
    fn get_ns_base(&self)->&NsBase;
    fn get_parent_ns(&self)->Option<KoID>;
    fn get_ns_instance(self)->NsEnum;
}

pub enum NsEnum{
    MntNs(MntNs),
    UtsNs(UtsNs),
    IpcNs(IpcNs),
    PidNs(PidNs),
    NetNs(NetNs),
    UsrNs(UsrNs),
    CgroupNs(CgroupNs),
}
impl TryFrom<MntNs> for NsEnum{
    fn try_from(value: MntNs) -> Result<Self, Self::Error> {
        if value.get_ns_type()==NSType::CLONE_NEWNS
        {
            Ok(NsEnum::MntNs(value))
        }
        else {
            Err(())
        }
    }
    type Error = ();
}
impl TryFrom<UtsNs> for NsEnum{
    fn try_from(value: UtsNs) -> Result<Self, Self::Error> {
        if value.get_ns_type()==NSType::CLONE_NEWUTS
        {
            Ok(NsEnum::UtsNs(value))
        }
        else {
            Err(())
        }
    }
    type Error = ();
}
impl TryFrom<IpcNs> for NsEnum{
    fn try_from(value: IpcNs) -> Result<Self, Self::Error> {
        if value.get_ns_type()==NSType::CLONE_NEWIPC
        {
            Ok(NsEnum::IpcNs(value))
        }
        else {
            Err(())
        }
    }
    type Error = ();
}
impl TryFrom<PidNs> for NsEnum{
    fn try_from(value: PidNs) -> Result<Self, Self::Error> {
        if value.get_ns_type()==NSType::CLONE_NEWPID
        {
            Ok(NsEnum::PidNs(value))
        }
        else {
            Err(())
        }
    }
    type Error = ();
}
impl TryFrom<NetNs> for NsEnum{
    fn try_from(value: NetNs) -> Result<Self, Self::Error> {
        if value.get_ns_type()==NSType::CLONE_NEWNET
        {
            Ok(NsEnum::NetNs(value))
        }
        else {
            Err(())
        }
    }
    type Error = ();
}
impl TryFrom<UsrNs> for NsEnum{
    fn try_from(value: UsrNs) -> Result<Self, Self::Error> {
        if value.get_ns_type()==NSType::CLONE_NEWNET
        {
            Ok(NsEnum::UsrNs(value))
        }
        else {
            Err(())
        }
    }
    type Error = ();
}
