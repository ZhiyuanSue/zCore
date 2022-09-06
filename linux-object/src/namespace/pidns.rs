#![allow(dead_code, unused_imports)]
use super::*;

pub type NsPid = u64;
pub type NsTid = u64;
pub struct PidNs
{
    base:NsBase,
    max_pid:Mutex<NsPid>,
    max_tid:Mutex<NsTid>,
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
            max_pid:Mutex::new(0),
            max_tid:Mutex::new(0),
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
        let mut max_pid=self.max_pid.lock();
        let pid=max_pid.deref_mut();
        (*pid)+=1;
        let curr_max=*(max_pid.deref());
        curr_max as NsPid
    }
    fn get_new_tid(&mut self)->NsTid{
        let mut max_tid=self.max_tid.lock();
        let tid=max_tid.deref_mut();
        (*tid)+=1;
        let curr_max=*(max_tid.deref());
        curr_max as NsPid
    }
    pub fn insert_pid(&mut self,processer_id:KoID,nsmanager:&NsManager)->Option<NsPid>{
        //alloc a new pid in the current ns for this processer and insert
        let namespace_pid=self.get_new_pid();
        self.pid_map.insert(processer_id, namespace_pid);
        //insert to the parent recursively
        let parent_ns_id=self.base.parent?;
        let ns_parent=nsmanager.get_ns(parent_ns_id)?;
        let mut e=ns_parent.lock();
        match e.deref_mut(){
            NsEnum::PidNs(p)=>
            {
                p.insert_pid(processer_id,nsmanager);
                return Some(processer_id as NsPid);
            },
            _=>{return None;}
        }
    }
    pub fn insert_tid(&mut self,thread_id:KoID,nsmanager:&NsManager)->Option<NsTid>{
        //alloc a new pid in the current ns for this processer and insert
        let namespace_tid=self.get_new_tid();
        self.tid_map.insert(thread_id, namespace_tid);
        //insert to the parent recursively
        let parent_ns_id=self.base.parent?;
        let ns_parent=nsmanager.get_ns(parent_ns_id)?;
        let mut e=ns_parent.lock();
        match e.deref_mut(){
            NsEnum::PidNs(p)=>
            {
                p.insert_tid(thread_id,nsmanager);
                return Some(thread_id as NsTid);
            },
            _=>{return None;}
        }
    }
    pub fn get_pid(&self,processer_id:KoID)->Option<&NsPid>{
        self.pid_map.get(&(processer_id as u64))
    }
    pub fn get_tid(&self,thread_id:KoID)->Option<&NsTid>{
        self.tid_map.get(&(thread_id as u64))
    }
}
pub fn insert_pid(processer_id:KoID,pid_ns:KoID)->Option<NsPid>
{
    warn!("insert {}",processer_id);
    let nsmanager=NS_MANAGER.lock();
    let ns_curr_enum=nsmanager.get_ns(pid_ns)?;
    let mut ns_curr=ns_curr_enum.lock();
    match ns_curr.deref_mut(){
        NsEnum::PidNs(p)=>
        {
            return p.insert_pid(processer_id,nsmanager.deref());
        }
        _=>{return None;}
    }
}
pub fn insert_tid(thread_id:KoID,pid_ns:KoID)->Option<NsTid>
{
    let nsmanager=NS_MANAGER.lock();
    let ns_curr_enum=nsmanager.get_ns(pid_ns)?;
    let mut ns_curr=ns_curr_enum.lock();
    match ns_curr.deref_mut(){
        NsEnum::PidNs(p)=>
        {
            return p.insert_tid(thread_id,nsmanager.deref());
        }
        _=>{return None;}
    }
}
pub fn get_pid_ns(processer_id:KoID,pid_ns:KoID)->Option<NsPid>
{
    let nsmanager=NS_MANAGER.lock();
    let ns_curr_enum=nsmanager.get_ns(pid_ns)?;
    let mut ns_curr=ns_curr_enum.lock();
    match ns_curr.deref_mut(){
        NsEnum::PidNs(p)=>
        {
            let find_res=p.get_pid(processer_id);
            match find_res{
                Some(pid)=>{
                    return Some(*pid);
                },
                None=>{
                    None
                },
            }
        }
        _=>{return None;}
    }
}
pub fn get_tid_ns(thread_id:KoID,pid_ns:KoID)->Option<NsTid>
{
    let nsmanager=NS_MANAGER.lock();
    let ns_curr_enum=nsmanager.get_ns(pid_ns)?;
    let mut ns_curr=ns_curr_enum.lock();
    match ns_curr.deref_mut(){
        NsEnum::PidNs(p)=>
        {
            let find_res = p.get_tid(thread_id);
            match find_res{
                Some(tid)=>{
                    return Some(*tid);
                },
                None=>None,
            }
        }
        _=>{return None;}
    }
}