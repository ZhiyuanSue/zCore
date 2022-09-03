//#[cfg(feature = "namespace")]
#![allow(dead_code, unused_imports)]
use super::*;
pub struct CgroupNs{
    base:NsBase,
}
impl NS for CgroupNs{
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
impl CgroupNs{
    fn new(parent:Option<KoID>)->Self
    {
        let cgroupns=CgroupNs{
            base:NsBase::new(NSType::CLONE_NEWNS,parent),

        };
        cgroupns
    }
    pub fn new_root()->KoID
    {
        let root=CgroupNs::new(None);
        let root_id=root.get_ns_id();
        NS_MANAGER.lock().set_init_ns(NSType::CLONE_NEWCGROUP,root_id);
        NS_MANAGER.lock().insert(Mutex::new(root.get_ns_instance()));
        root_id
    }
    pub fn new_child(&self)->KoID
    {
        let child = CgroupNs::new(Some(self.get_ns_id()));
        //insert child to parent's vec
        let child_id=child.get_ns_id();
        let arc_vec=&self.base.child_ns_vec;
        arc_vec.lock().push(child_id);
        //insert child to the ns manager
        NS_MANAGER.lock().insert(Mutex::new(child.get_ns_instance()));
        child_id.clone()
    }
}