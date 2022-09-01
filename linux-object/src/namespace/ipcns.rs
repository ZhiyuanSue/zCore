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
    fn get_ns_instance(self)->NsEnum
    {
        NsEnum::try_from(self).unwrap()
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
    pub fn new_root()->Self
    {
        let root=IpcNs::new(None);
        root
    }
    pub fn new_child(&mut self)
    {
        let child = IpcNs::new(Some(self.get_ns_id()));
        //insert child to parent's vec
        let child_id=child.get_ns_id();
        let arc_vec=&self.base.child_ns_vec;
        arc_vec.lock().push(child_id);
        NS_MANAGER.lock().insert(Mutex::new(child.get_ns_instance()));
    }
}