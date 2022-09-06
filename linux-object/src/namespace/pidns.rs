#![allow(dead_code, unused_imports)]
use super::*;

pub type NsPid = u64;
pub type NsTid = u64;
pub struct PidNs
{
    base:NsBase,
    pid_map:HashMap<KoID,NsPid>,
    tid_map:HashMap<KoID,NsTid>,
    usr_ns:KoID,
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
        Some(NsEnum::PidNs(self))
    }
    fn get_usr_ns(&self)->KoID
    {
        self.usr_ns
    }
}
impl PidNs{
    fn new(parent:Option<KoID>,usr_id:KoID)->Self
    {
        let pidns=PidNs{
            base:NsBase::new(NSType::CLONE_NEWNS,parent),
            pid_map:HashMap::new(),
            tid_map:HashMap::new(),
            usr_ns:usr_id,
        };
        pidns
    }
    pub fn new_root(usr_id:KoID)->KoID
    {
        let root=PidNs::new(None,usr_id);
        let root_id=root.get_ns_id();
        warn!("new pid ns with id {}",root_id);
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
        let child = PidNs::new(Some(self.get_ns_id()),self.usr_ns);
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
    fn get_new_pid(&mut self)->NsPid{
        0
    }
    fn get_new_tid(&mut self)->NsTid{
        0
    }
    pub fn insert_pid(&mut self,processer_id:KoID){
        //alloc a new pid in the current ns for this processer and insert
        let namespace_pid=self.get_new_pid();
        self.pid_map.insert(processer_id, namespace_pid);
        //insert to the parent recursively
        //I don't know why my code seems strange. should I use '?' ?
        let parent_ns=self.base.parent;
        let nsmanager=NS_MANAGER.lock();
        match parent_ns{
            Some(ns_id)=>{
                let ns_parent_enum=nsmanager.get_ns(ns_id);
                match ns_parent_enum{
                    Some(mutex_e)=>{
                        let mut e=mutex_e.lock();
                        match e.deref_mut(){
                            NsEnum::PidNs(p)=>
                            {
                                p.insert_pid(processer_id);
                            },
                            _=>()}
                    },None=>()}
            },None=>()};
    }
    pub fn insert_tid(&mut self,thread_id:KoID){
        //alloc a new pid in the current ns for this processer and insert
        let namespace_tid=self.get_new_tid();
        self.tid_map.insert(thread_id, namespace_tid);
        //insert to the parent recursively
        let parent_ns=self.base.parent;
        let nsmanager=NS_MANAGER.lock();
        match parent_ns{
            Some(ns_id)=>{
                let ns_parent_enum=nsmanager.get_ns(ns_id);
                match ns_parent_enum{
                    Some(mutex_e)=>{
                        let mut e=mutex_e.lock();
                        match e.deref_mut(){
                            NsEnum::PidNs(p)=>
                            {
                                p.insert_pid(thread_id);
                            },
                            _=>()}
                    },None=>()}
            },None=>()};
    }
    pub fn get_pid(&self,processer_id:KoID)->Option<&u64>{
        self.tid_map.get(&(processer_id as u64))
    }
    pub fn get_tid(&self,thread_id:KoID)->Option<&u64>{
        self.tid_map.get(&(thread_id as u64))
    }
}