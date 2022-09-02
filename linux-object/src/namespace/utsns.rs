use core::ops::DerefMut;

use super::*;
use crate::alloc::string::ToString;
pub struct UtsNs
{
    base:NsBase,
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
    fn get_ns_instance(self)->NsEnum
    {
        NsEnum::try_from(self).unwrap()
    }
}
impl UtsNs{
    fn new(parent:Option<KoID>,hostname:String,domainname:String)->Self
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
            sysname:"Linux".to_string(),
            hostname:hostname,
            release:release.to_string(),
            version:vdso_const.version_string.as_str().to_string(),
            machine_arch:arch.to_string(),
            domainname:domainname,
        };
        utsns
    }
    pub fn new_root()->Self
    {
        let root=UtsNs::new(None,
            "zcore".to_string(),
            "rcore-os".to_string(),
        );
        root
    }
    pub fn new_child(&self)->KoID
    {
        let child = UtsNs::new(Some(self.get_ns_id()),
            self.hostname.clone(),
            self.domainname.clone(),
        );
        //insert child to parent's vec
        let child_id=child.get_ns_id();
        let arc_vec=&self.base.child_ns_vec;
        arc_vec.lock().push(child_id);
        NS_MANAGER.lock().insert(Mutex::new(child.get_ns_instance()));
        child_id.clone()
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
pub fn get_uname(ns_id:KoID)->Option<String>
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
                        uts.sysname,
                        
                    ];
                    return Some(strings);
                },
                _=>{return None;}
            }
        },
        None=>{ return None; }
    }
}