//#[cfg(feature = "namespace")]
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
    pub fn new_root()->Self
    {
        let root=CgroupNs::new(None);
        root
    }
    pub fn new_child(self)->CgroupNs
    {
        let child = CgroupNs::new(Some(self.get_ns_id()));

        child
    }
}