#![allow(dead_code, unused_imports)]
use core::ops::DerefMut;
use super::*;
use crate::alloc::string::ToString;
pub struct UtsNs
{
    base:NsBase,
    usr_ns:KoID,
    sysname:String,
    hostname:String,    //also nodename
    release:String,
    version:String,
    machine_arch:String,
    domainname:String,
}
impl NS for UtsNs{
    fn get_ns_id(&self)->KoID
    {
        self.base.base.id
    }
    fn get_ns_type(&self)->NSType
    {
        self.base.nstype
    }
    fn get_ns_base(&self)->&NsBase
    {
        &self.base
    }
    fn get_parent_ns(&self)->Option<KoID>
    {
        self.base.parent
    }
    fn get_ns_instance(self)->Option<NsEnum>
    {
        Some(NsEnum::UtsNs(self))
    }
    fn get_usr_ns(&self)->KoID
    {
        self.usr_ns
    }
}
impl UtsNs{
    fn new(parent:Option<KoID>,hostname:String,domainname:String,usr_id:KoID)->Self
    {
        let release = alloc::string::String::from(concat!(env!("CARGO_PKG_VERSION"), "-zcore"));
        #[cfg(not(target_os = "none"))]
        let release = release + "-libos";

        let vdso_const = kernel_hal::vdso::vdso_constants();

        let arch = if cfg!(target_arch = "x86_64") {
            "x86_64"
        } else if cfg!(target_arch = "aarch64") {
            "aarch64"
        } else if cfg!(target_arch = "riscv64") {
            "riscv64"
        } else {
            "unknown"
        };
        let utsns=UtsNs{
            base:NsBase::new(NSType::CLONE_NEWNS,parent),
            usr_ns:usr_id,
            sysname:"Linux".to_string(),
            hostname:hostname,
            release:release.to_string(),
            version:vdso_const.version_string.as_str().to_string(),
            machine_arch:arch.to_string(),
            domainname:domainname,
        };
        utsns
    }
    pub fn new_root(usr_id:KoID)->KoID
    {
        let root=UtsNs::new(None,
            "zcore".to_string(),
            "rcore-os".to_string(),
            usr_id,
        );
        let root_id=root.get_ns_id();
        NS_MANAGER.lock().set_init_ns(NSType::CLONE_NEWUTS,root_id);
        let ins=root.get_ns_instance();
        match ins{
            Some(i)=>{
                NS_MANAGER.lock().insert(Mutex::new(i));
                root_id
            },
            None=>{
                0
            }
        }
    }
    pub fn new_child(&self)->KoID
    {
        let child = UtsNs::new(Some(self.get_ns_id()),
            self.hostname.clone(),
            self.domainname.clone(),
            self.usr_ns
        );
        //insert child to parent's vec
        let child_id=child.get_ns_id();
        let arc_vec=&self.base.child_ns_vec;
        arc_vec.lock().push(child_id);
        let ins=child.get_ns_instance();
        match ins{
            Some(i)=>{
                NS_MANAGER.lock().insert(Mutex::new(i));
                child_id.clone()
            },
            None=>{
                0
            }
        }
    }
    pub fn set_host_name(&mut self,base: UserInPtr<u8>, len: usize)->Option<String>
    {
        let buf=base.as_slice(len);
        match buf{
            Ok(sbuf)=>{
                let s=String::from_utf8_lossy(sbuf).to_string();
                let res=s.clone();
                self.hostname=s;
                return Some(res);
            },
            Err(_)=>{return None;}
        }
    }
    pub fn set_domain_name(&mut self,base: UserInPtr<u8>, len: usize)->Option<String>
    {
        let buf=base.as_slice(len);
        match buf{
            Ok(sbuf)=>{
                let s=String::from_utf8_lossy(sbuf).to_string();
                let res=s.clone();
                self.domainname=s;
                return Some(res);
            },
            Err(_)=>{return None;}
        }
    }
    pub fn get_host_name(&self)->String
    {
        self.hostname.clone()
    }
    pub fn get_domain_name(&self)->String
    {
        self.domainname.clone()
    }
}
pub fn copy_utsname(father_ns_id:KoID)->Option<KoID>
{
    let nsmanager=NS_MANAGER.lock();
    let nsenum=nsmanager.get_ns(father_ns_id);
    match nsenum{
        Some(mutex_ns)=>
        {
            let e=mutex_ns.lock();
            match e.deref(){
                NsEnum::UtsNs(uts)=>{
                    return Some(uts.new_child());
                },
                _=>{return None;}
            }
        },
        None=>{ return None; }
    }
}
pub fn set_host_name(ns_id:KoID, base: UserInPtr<u8>, len: usize)->Option<String>
{
    let nsmanager=NS_MANAGER.lock();
    let nsenum=nsmanager.get_ns(ns_id);
    match nsenum{
        Some(mutex_ns)=>
        {
            let mut e=mutex_ns.lock();
            match e.deref_mut(){
                NsEnum::UtsNs(uts)=>{
                    return uts.set_host_name(base,len);
                },
                _=> {return None;}
            }
        },
        None=>{return None;}
    }
}
pub fn set_domain_name(ns_id:KoID, base: UserInPtr<u8>, len: usize)->Option<String>
{
    let nsmanager=NS_MANAGER.lock();
    let nsenum=nsmanager.get_ns(ns_id);
    match nsenum{
        Some(mutex_ns)=>
        {
            let mut e=mutex_ns.lock();
            match e.deref_mut(){
                NsEnum::UtsNs(uts)=>{
                    return uts.set_domain_name(base,len);
                },
                _=>{return None;}
            }
        },
        None=>{return None;}
    }
}
fn get_host_name(ns_id:KoID)->Option<String>
{
    let nsmanager=NS_MANAGER.lock();
    let nsenum=nsmanager.get_ns(ns_id);
    match nsenum{
        Some(mutex_ns)=>
        {
            let e=mutex_ns.lock();
            match e.deref(){
                NsEnum::UtsNs(uts)=>{
                    return Some(uts.get_host_name());
                },
                _=>{return None;}
            }
        },
        None=>{ return None; }
    }
}
fn get_domain_name(ns_id:KoID)->Option<String>
{
    let nsmanager=NS_MANAGER.lock();
    let nsenum=nsmanager.get_ns(ns_id);
    match nsenum{
        Some(mutex_ns)=>
        {
            let e=mutex_ns.lock();
            match e.deref(){
                NsEnum::UtsNs(uts)=>{
                    return Some(uts.get_domain_name());
                },
                _=>{return None;}
            }
        },
        None=>{ return None; }
    }
}
pub fn get_uname(ns_id:KoID,buf: UserOutPtr<u8>)->SysResult
{
    let nsmanager=NS_MANAGER.lock();
    let nsenum=nsmanager.get_ns(ns_id);
    match nsenum{
        Some(mutex_ns)=>
        {
            let e=mutex_ns.lock();
            match e.deref(){
                NsEnum::UtsNs(uts)=>{
                    let strings=[
                        uts.sysname.as_str(),
                        uts.hostname.as_str(),
                        uts.release.as_str(),
                        uts.version.as_str(),
                        uts.machine_arch.as_str(),
                        uts.domainname.as_str(),
                    ];
                    for (i, &s) in strings.iter().enumerate() {
                        const OFFSET: usize = 65;
                        buf.add(i * OFFSET).write_cstring(s)?;
                    }
                    return Ok(0);
                },
                _=>Err(LxError::EUNDEF),
            }
        },
        None=>Err(LxError::EUNDEF)
    }
}