//#[cfg(feature = "namespace")]
//This mod is try to let zcore have namespace and cgroup
#![allow(dead_code, unused_imports)]
pub mod cgroupns;
pub mod ipcns;
pub mod mntns;
pub mod netns;
pub mod pidns;
pub mod usrns;
pub mod utsns;

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
use lock::Mutex;
use hashbrown::HashMap;
use zircon_object::object::*;
use core::convert::TryFrom;
use core::ops::DerefMut;
use kernel_hal::user::{UserInPtr,UserOutPtr};
use crate::error::{LxError, SysResult};
use crate::fs::vfs::FileSystem;

pub type KoID = u64;
pub struct NsManager{
    ns_hash:HashMap<KoID,Mutex<NsEnum>>,
    init_ns:NsProxy,
}
impl NsManager{
    pub fn init()->Arc<Mutex<Self>>{
        Arc::new(
            Mutex::new(
                NsManager{
                    ns_hash:HashMap::new(),
                    init_ns:NsProxy::new(),
                }
            )
        )
    }
    pub fn set_root_ns(&mut self,rootfs: Arc<dyn FileSystem>)
    {
        let root_mnt_ns=MntNs::new_root(rootfs);
        self.init_ns.change_proxy(NSType::CLONE_NEWNS, root_mnt_ns);
        let root_cgroup_ns=CgroupNs::new_root();
        self.init_ns.change_proxy(NSType::CLONE_NEWCGROUP, root_cgroup_ns);
        let root_ipc_ns=IpcNs::new_root();
        self.init_ns.change_proxy(NSType::CLONE_NEWIPC, root_ipc_ns);
        let root_net_ns=NetNs::new_root();
        self.init_ns.change_proxy(NSType::CLONE_NEWNET, root_net_ns);
        let root_pid_ns=PidNs::new_root();
        self.init_ns.change_proxy(NSType::CLONE_NEWPID, root_pid_ns);
        let root_usr_ns=UsrNs::new_root();
        self.init_ns.change_proxy(NSType::CLONE_NEWUSER, root_usr_ns);
        let root_uts_ns=UtsNs::new_root();
        self.init_ns.change_proxy(NSType::CLONE_NEWUTS, root_uts_ns);
    }
    pub fn get_root_ns(self)->NsProxy{
        self.init_ns
    }
    pub fn get_ns(&self,ns_id:KoID)->Option<&Mutex<NsEnum>>
    {
        match self.ns_hash.get(&ns_id)
        {
            Some(ns) => {
                return Some(ns);
            },
            None => {return None;},
        }
    }
    pub fn insert(&mut self,ns: Mutex<NsEnum>)->KoID
    {
        let id=ns.lock().get_ns_id();
        self.ns_hash.insert(id,ns);
        id
    }
    pub fn set_init_ns(&mut self,ns:NSType,ns_id:KoID)
    {
        self.init_ns.change_proxy(ns, ns_id);
    }
}
lazy_static!{
    pub static ref NS_MANAGER:Arc<Mutex<NsManager>>= NsManager::init();
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
impl Clone for NsProxy{
    fn clone(&self) -> Self {
        NsProxy { 
            mnt_ns: self.mnt_ns.clone(), 
            uts_ns: self.uts_ns.clone(), 
            ipc_ns: self.ipc_ns.clone(), 
            pid_ns: self.pid_ns.clone(), 
            net_ns: self.net_ns.clone(), 
            usr_ns: self.usr_ns.clone(), 
            cgroup_ns: self.cgroup_ns.clone() 
        }
    }
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
    pub fn change_proxy(&mut self,ns:NSType,id:KoID)
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
    pub fn get_proxy_ns(&self,ns:NSType)->Option<KoID>
    {
        match ns{
            NSType::NSTYPE_ANY          =>  None,
            NSType::CLONE_NEWCGROUP     =>  Some(self.cgroup_ns),
            NSType::CLONE_NEWIPC        =>  Some(self.ipc_ns),
            NSType::CLONE_NEWNET        =>  Some(self.net_ns),
            NSType::CLONE_NEWNS         =>  Some(self.mnt_ns),
            NSType::CLONE_NEWPID        =>  Some(self.pid_ns),
            NSType::CLONE_NEWTIME       =>  None,
            NSType::CLONE_NEWUSER       =>  Some(self.usr_ns),
            NSType::CLONE_NEWUTS        =>  Some(self.uts_ns),
            _=>None,
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
    use_cnt:Mutex<usize>,  //use count
}
impl_kobject!(NsBase);
impl NsBase{
    pub fn new(nstype:NSType,parent:Option<KoID>)->Self{
        NsBase { 
            base: KObjectBase::new(), 
            nstype: nstype,
            parent: parent,
            child_ns_vec: Arc::new(Mutex::new(Vec::new())),
            use_cnt:Mutex::new(0),
        }
    }
    pub fn add_cnt(&mut self){
        let mut cnt=self.use_cnt.lock();
        let mut_cnt=cnt.deref_mut();
        *mut_cnt+=1;
    }
    pub fn sub_cnt(&mut self){
        let mut cnt=self.use_cnt.lock();
        let mut_cnt=cnt.deref_mut();
        *mut_cnt-=1;
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
        if value.get_ns_type()==NSType::CLONE_NEWUSER
        {
            Ok(NsEnum::UsrNs(value))
        }
        else {
            Err(())
        }
    }
    type Error = ();
}
impl TryFrom<CgroupNs> for NsEnum{
    fn try_from(value: CgroupNs) -> Result<Self, Self::Error> {
        if value.get_ns_type()==NSType::CLONE_NEWCGROUP
        {
            Ok(NsEnum::CgroupNs(value))
        }
        else {
            Err(())
        }
    }
    type Error = ();
}

impl NS for NsEnum{
    fn get_ns_id(&self)->KoID
    {
        match self{
            NsEnum::CgroupNs(ns)=>ns.get_ns_id(),
            NsEnum::IpcNs(ns)=>ns.get_ns_id(),
            NsEnum::MntNs(ns)=>ns.get_ns_id(),
            NsEnum::NetNs(ns)=>ns.get_ns_id(),
            NsEnum::PidNs(ns)=>ns.get_ns_id(),
            NsEnum::UsrNs(ns)=>ns.get_ns_id(),
            NsEnum::UtsNs(ns)=>ns.get_ns_id(),
        }
    }
    fn get_ns_type(&self)->NSType
    {
        match self{
            NsEnum::CgroupNs(ns)=>ns.get_ns_type(),
            NsEnum::IpcNs(ns)=>ns.get_ns_type(),
            NsEnum::MntNs(ns)=>ns.get_ns_type(),
            NsEnum::NetNs(ns)=>ns.get_ns_type(),
            NsEnum::PidNs(ns)=>ns.get_ns_type(),
            NsEnum::UsrNs(ns)=>ns.get_ns_type(),
            NsEnum::UtsNs(ns)=>ns.get_ns_type(),
        }
    }
    fn get_ns_base(&self)->&NsBase
    {
        match self{
            NsEnum::CgroupNs(ns)=>ns.get_ns_base(),
            NsEnum::IpcNs(ns)=>ns.get_ns_base(),
            NsEnum::MntNs(ns)=>ns.get_ns_base(),
            NsEnum::NetNs(ns)=>ns.get_ns_base(),
            NsEnum::PidNs(ns)=>ns.get_ns_base(),
            NsEnum::UsrNs(ns)=>ns.get_ns_base(),
            NsEnum::UtsNs(ns)=>ns.get_ns_base(),
        }
    }
    fn get_parent_ns(&self)->Option<KoID>
    {
        match self{
            NsEnum::CgroupNs(ns)=>ns.get_parent_ns(),
            NsEnum::IpcNs(ns)=>ns.get_parent_ns(),
            NsEnum::MntNs(ns)=>ns.get_parent_ns(),
            NsEnum::NetNs(ns)=>ns.get_parent_ns(),
            NsEnum::PidNs(ns)=>ns.get_parent_ns(),
            NsEnum::UsrNs(ns)=>ns.get_parent_ns(),
            NsEnum::UtsNs(ns)=>ns.get_parent_ns(),
        }
    }
    fn get_ns_instance(self)->NsEnum
    {
        match self{
            NsEnum::CgroupNs(ns)=>ns.get_ns_instance(),
            NsEnum::IpcNs(ns)=>ns.get_ns_instance(),
            NsEnum::MntNs(ns)=>ns.get_ns_instance(),
            NsEnum::NetNs(ns)=>ns.get_ns_instance(),
            NsEnum::PidNs(ns)=>ns.get_ns_instance(),
            NsEnum::UsrNs(ns)=>ns.get_ns_instance(),
            NsEnum::UtsNs(ns)=>ns.get_ns_instance(),
        }
    }
}
pub fn sys_init_ns(rootfs: Arc<dyn FileSystem>)
{
    NS_MANAGER.lock().set_root_ns(rootfs);
}
