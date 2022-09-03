#![allow(dead_code, unused_imports)]
use super::*;
pub struct IpcNs
{
    base:NsBase,
}
impl NS for IpcNs{
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
                warn!("no such a ipc instance");
                None
            }
        }
    }
}
impl IpcNs{
    fn new(parent:Option<KoID>)->Self
    {
        let ipcns=IpcNs{
            base:NsBase::new(NSType::CLONE_NEWNS,parent),

        };
        ipcns
    }
    pub fn new_root()->KoID
    {
        let root=IpcNs::new(None);
        let root_id=root.get_ns_id();
        NS_MANAGER.lock().set_init_ns(NSType::CLONE_NEWIPC,root_id);
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
        let child = IpcNs::new(Some(self.get_ns_id()));
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
}