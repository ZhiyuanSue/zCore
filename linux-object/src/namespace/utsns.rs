use core::ops::DerefMut;

use super::*;
use crate::alloc::string::ToString;
pub struct UtsNs
{
    base:NsBase,
    hostname:String,
    domainname:String,
    kernel_version:String,
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
    fn new(parent:Option<KoID>,hostname:String,domainname:String,kernel_version:String)->Self
    {
        let utsns=UtsNs{
            base:NsBase::new(NSType::CLONE_NEWNS,parent),
            hostname:hostname,
            domainname:domainname,
            kernel_version:kernel_version,
        };
        utsns
    }
    pub fn new_root()->Self
    {
        let root=UtsNs::new(None,
            "zcore".to_string(),
            "rcore-os".to_string(),
            "0.1.0".to_string(),
        );
        root
    }
    pub fn new_child(&self)->KoID
    {
        let child = UtsNs::new(Some(self.get_ns_id()),
            self.hostname.clone(),
            self.domainname.clone(),
            self.kernel_version.clone(),
        );
        //insert child to parent's vec
        let child_id=child.get_ns_id();
        let arc_vec=&self.base.child_ns_vec;
        arc_vec.lock().push(child_id);
        NS_MANAGER.lock().insert(Mutex::new(child.get_ns_instance()));
        child_id.clone()
    }
    pub fn set_host_name(&mut self,base: UserInPtr<u8>, len: usize)
    {
        let set_string="set".to_string();
        self.hostname=set_string;
    }
    pub fn set_domain_name(&mut self,base: UserInPtr<u8>, len: usize)
    {
        let set_string="set".to_string();
        self.domainname=set_string;
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
pub fn set_host_name(ns_id:KoID, base: UserInPtr<u8>, len: usize)
{
    let nsmanager=NS_MANAGER.lock();
    let nsenum=nsmanager.get_ns(ns_id);
    match nsenum{
        Some(mutex_ns)=>
        {
            let mut e=mutex_ns.lock();
            match e.deref_mut(){
                NsEnum::UtsNs(uts)=>{
                    uts.set_host_name(base,len);
                },
                _=>()
            }
        },
        None=>()
    }
}
pub fn set_domain_name(ns_id:KoID, base: UserInPtr<u8>, len: usize)
{
    let nsmanager=NS_MANAGER.lock();
    let nsenum=nsmanager.get_ns(ns_id);
    match nsenum{
        Some(mutex_ns)=>
        {
            let mut e=mutex_ns.lock();
            match e.deref_mut(){
                NsEnum::UtsNs(uts)=>{
                    uts.set_domain_name(base,len);
                },
                _=>()
            }
        },
        None=>()
    }
}
pub fn get_host_name(ns_id:KoID)->Option<String>
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
pub fn get_domain_name(ns_id:KoID)->Option<String>
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