#![allow(dead_code, unused_imports)]
use super::*;
use rcore_fs::vfs::FileSystem;
pub struct MntNs
{
    base:NsBase,
    rootfs:Arc<dyn FileSystem>,

}
impl NS for MntNs{
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
        Some(NsEnum::MntNs(self))
    }
}
impl MntNs{
    fn new(parent:Option<KoID>,init_root_fs:Arc<dyn FileSystem>)->Self
    {
        let mntns=MntNs{
            base:NsBase::new(NSType::CLONE_NEWNS,parent),
            rootfs:init_root_fs,
        };
        mntns
    }
    fn copy_fs(&self,old_root_fs:Arc<dyn FileSystem>)->Arc<dyn FileSystem>
    {
        //This function copy the file system and all the inode
        //so the new one can be isolated with the old one
        old_root_fs.clone()
    }
    pub fn new_root(init_root_fs:Arc<dyn FileSystem>)->KoID
    {
        let root=MntNs::new(None,init_root_fs);
        let root_id=root.get_ns_id();
        NS_MANAGER.lock().set_init_ns(NSType::CLONE_NEWNS,root_id);
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
    pub fn new_child(&self,old_root_fs:Arc<dyn FileSystem>) ->KoID
    {
        let new_root_fs=self.copy_fs(old_root_fs);
        let parent=self.get_ns_id();
        let child = MntNs::new(Some(parent),
            new_root_fs
        );
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
    pub fn get_root_fs(&self)->&Arc<dyn FileSystem>
    {
        &self.rootfs
    }
    pub fn set_root_fs(mut self,root_fs:Arc<dyn FileSystem>)
    {
        self.rootfs=root_fs;
    }
}