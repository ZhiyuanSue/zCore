#![allow(dead_code, unused_imports)]
use alloc::string::ToString;

use super::*;
pub struct UsrNs
{
    base:NsBase,
    usrname:String,
}
impl NS for UsrNs{
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
        let res=NsEnum::try_from(self);
        match res{
            Ok(e)=>Some(e),
            Err(_)=>{
                warn!("no such a usr instance");
                None
            }
        }
    }
}
impl UsrNs{
    fn new(parent:Option<KoID>)->Self
    {
        let usrns=UsrNs{
            base:NsBase::new(NSType::CLONE_NEWNS,parent),
            usrname:"zcore".to_string(),

        };
        usrns
    }
    pub fn new_root()->KoID
    {
        let root=UsrNs::new(None);
        let root_id=root.get_ns_id();
        NS_MANAGER.lock().set_init_ns(NSType::CLONE_NEWUSER,root_id);
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
        let child = UsrNs::new(Some(self.get_ns_id()));
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
    pub fn get_usrname(&self)->&str{
        self.usrname.as_str()
    }
}