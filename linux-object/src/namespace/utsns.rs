use super::*;
use crate::alloc::string::ToString;
pub struct UtsNs
{
    base:NsBase,
    hostname:String,
    kernel_version:String,
}
impl NS for UtsNs{
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
impl UtsNs{
    fn new(parent:Option<KoID>,hostname:String,kernel_version:String)->Self
    {
        let utsns=UtsNs{
            base:NsBase::new(NSType::CLONE_NEWNS,parent),
            hostname:hostname,
            kernel_version:kernel_version,
        };
        utsns
    }
    pub fn new_root()->Self
    {
        let root=UtsNs::new(None,
            "init".to_string(),
            "0.0.1".to_string(),
        );
        root
    }
    pub fn new_child(self)->UtsNs
    {
        let child = UtsNs::new(Some(self.get_ns_id()),
            self.hostname.clone(),
            self.kernel_version.clone(),
        );
        //insert child to parent's vec
        let child_id=child.get_ns_id();
        let arc_vec=&self.base.child_ns_vec;
        arc_vec.lock().push(child_id);
        NS_MANAGER.lock().insert(Mutex::new(child.get_ns_instance()));
        child
    }
}