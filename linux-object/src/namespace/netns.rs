use super::*;
pub struct NetNs
{
    base:NsBase,
}
impl NS for NetNs{
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
impl NetNs{
    fn new(parent:Option<KoID>)->Self
    {
        let netns=NetNs{
            base:NsBase::new(NSType::CLONE_NEWNS,parent),

        };
        netns
    }
    pub fn new_root()->Self
    {
        let root=NetNs::new(None);
        root
    }
    pub fn new_child(&self)->KoID
    {
        let child = NetNs::new(Some(self.get_ns_id()));
        //insert child to parent's vec
        let child_id=child.get_ns_id();
        let arc_vec=&self.base.child_ns_vec;
        arc_vec.lock().push(child_id);
        NS_MANAGER.lock().insert(Mutex::new(child.get_ns_instance()));
        child_id.clone()
    }
}