//#[cfg(feature = "namespace")]
#![allow(dead_code, unused_imports)]
use super::*;
pub struct CgroupNs{
    base:NsBase,
    usr_ns:KoID,
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
    fn get_ns_instance(self)->Option<NsEnum>
    {
        Some(NsEnum::CgroupNs(self))
    }
    fn get_usr_ns(&self)->KoID
    {
        self.usr_ns
    }
}
impl CgroupNs{
    fn new(parent:Option<KoID>,usr_id:KoID)->Self
    {
        let cgroupns=CgroupNs{
            base:NsBase::new(NSType::CLONE_NEWNS,parent),
            usr_ns:usr_id,
        };
        cgroupns
    }
    pub fn new_root(usr_id:KoID)->KoID
    {
        let root=CgroupNs::new(None,usr_id);
        let root_id=root.get_ns_id();
        warn!("new cgroup ns with id {}",root_id);
        NS_MANAGER.lock().set_init_ns(NSType::CLONE_NEWCGROUP,root_id);
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
        let child = CgroupNs::new(Some(self.get_ns_id()),self.usr_ns);
        //insert child to parent's vec
        let child_id=child.get_ns_id();
        let arc_vec=&self.base.child_ns_vec;
        arc_vec.lock().push(child_id);
        //insert child to the ns manager
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
}