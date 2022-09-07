#![allow(dead_code, unused_imports)]
use super::*;
use crate::ipc::*;
pub struct IpcNs
{
    base:NsBase,
    usr_ns:KoID,
    sem_ids:Vec<SemId>,
    shm_ids:Vec<ShmId>,
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
        Some(NsEnum::IpcNs(self))
    }
    fn get_usr_ns(&self)->KoID
    {
        self.usr_ns
    }
}
impl IpcNs{
    fn new(parent:Option<KoID>,usr_id:KoID)->Self
    {
        let ipcns=IpcNs{
            base:NsBase::new(NSType::CLONE_NEWNS,parent),
            usr_ns:usr_id,
            sem_ids:Vec::new(),
            shm_ids:Vec::new(),
        };
        ipcns
    }
    pub fn new_root(usr_id:KoID)->KoID
    {
        let root=IpcNs::new(None,usr_id);
        let root_id=root.get_ns_id();
        warn!("new ipc ns with id {}",root_id);
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
        let child = IpcNs::new(Some(self.get_ns_id()),self.usr_ns);
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
    pub fn insert_sem(&mut self,sem_id:SemId){
        self.sem_ids.push(sem_id);
    }
    pub fn insert_shm(&mut self,shm_id:ShmId){
        self.shm_ids.push(shm_id);
    }
    pub fn sem_accessible(&self,sem_id:SemId)->bool{
        let search=self.sem_ids.get(sem_id);
        match search{
            Some(_)=>{return true;},
            None=>{return false;}
        }
    }
    pub fn shm_accessible(&self,shm_id:ShmId)->bool
    {
        let search=self.shm_ids.get(shm_id);
        match search{
            Some(_)=>{return true;},
            None=>{return false;}
        }
    }
}
pub fn insert_sem(ns_id:KoID,sem_id:SemId)
{
    let nsmanager=NS_MANAGER.lock();
    let nsenum=nsmanager.get_ns(ns_id);
    match nsenum{
        Some(mutex_ns)=>
        {
            let mut e=mutex_ns.lock();
            match e.deref_mut(){
                NsEnum::IpcNs(i)=>{
                    i.insert_sem(sem_id);
                }
                _=>()
            }
        }
        None=>()
    }
}
pub fn insert_shm(ns_id:KoID,shm_id:SemId)
{
    let nsmanager=NS_MANAGER.lock();
    let nsenum=nsmanager.get_ns(ns_id);
    match nsenum{
        Some(mutex_ns)=>
        {
            let mut e=mutex_ns.lock();
            match e.deref_mut(){
                NsEnum::IpcNs(i)=>{
                    i.insert_shm(shm_id);
                }
                _=>()
            }
        }
        None=>()
    }
}
pub fn sem_accessible(ns_id:KoID,sem_id:SemId)->bool
{
    let nsmanager=NS_MANAGER.lock();
    let nsenum=nsmanager.get_ns(ns_id);
    match nsenum{
        Some(mutex_ns)=>
        {
            let e=mutex_ns.lock();
            match e.deref(){
                NsEnum::IpcNs(i)=>{
                    i.sem_accessible(sem_id)
                }
                _=>{
                    warn!("unaccessible");
                    false
                }
            }
        }
        None=>{
            warn!("unaccessible");
            false
        }
    }
}
pub fn shm_accessible(ns_id:KoID,shm_id:SemId)->bool
{
    let nsmanager=NS_MANAGER.lock();
    let nsenum=nsmanager.get_ns(ns_id);
    match nsenum{
        Some(mutex_ns)=>
        {
            let e=mutex_ns.lock();
            match e.deref(){
                NsEnum::IpcNs(i)=>{
                    i.shm_accessible(shm_id)
                }
                _=>{
                    warn!("unaccessible");
                    false
                }
            }
        }
        None=>{
            warn!("unaccessible");
            false
        }
    }
}