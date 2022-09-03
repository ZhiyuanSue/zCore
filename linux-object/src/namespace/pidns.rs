#![allow(dead_code, unused_imports)]
use super::*;

pub type NsPid = u64;
pub struct PidNs
{
    base:NsBase,
    pid_map:HashMap<KoID,NsPid>,
}
impl NS for PidNs{
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
                warn!("no such a pid instance");
                None
            }
        }
    }
}
impl PidNs{
    fn new(parent:Option<KoID>)->Self
    {
        let pidns=PidNs{
            base:NsBase::new(NSType::CLONE_NEWNS,parent),
            pid_map:HashMap::new(),
        };
        pidns
    }
    pub fn new_root()->KoID
    {
        let root=PidNs::new(None);
        let root_id=root.get_ns_id();
        NS_MANAGER.lock().set_init_ns(NSType::CLONE_NEWPID,root_id);
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
        let child = PidNs::new(Some(self.get_ns_id()));
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
    pub fn insert_pid(mut self,processer_id:KoID,namespace_pid:NsPid){
        self.pid_map.insert(processer_id, namespace_pid);
    }
}