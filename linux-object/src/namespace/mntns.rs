#![allow(dead_code, unused_imports)]
use super::*;
use rcore_fs::vfs::FileSystem;
use rcore_fs::vfs::INode;
use crate::fs::*;
pub struct MntNs
{
    base:NsBase,
    usr_ns:KoID,
    rootfs:Arc<dyn FileSystem>,
    root_inode:Arc<dyn INode>,
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
    fn get_usr_ns(&self)->KoID
    {
        self.usr_ns
    }
}
impl MntNs{
    fn new(parent:Option<KoID>,init_root_fs:Arc<dyn FileSystem>,usr_id:KoID,root_inode:Arc<dyn INode>)->Self
    {
        let mntns=MntNs{
            base:NsBase::new(NSType::CLONE_NEWNS,parent),
            rootfs:init_root_fs,
            usr_ns:usr_id,
            root_inode:root_inode,
        };
        mntns
    }
    pub fn new_root(init_root_fs:Arc<dyn FileSystem>,usr_id:KoID)->KoID
    {
        let root_inode=create_root_fs(init_root_fs.clone());
        let root=MntNs::new(None,init_root_fs,usr_id,root_inode);
        let root_id=root.get_ns_id();
        warn!("new mnt ns with id {}",root_id);
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
    pub fn new_child(&mut self) ->KoID
    {
        let parent=self.get_ns_id();
        let root_inode=create_root_fs(self.rootfs.clone());
        let child = MntNs::new(Some(parent),
            self.rootfs.clone(),
            self.usr_ns,
            root_inode,
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
    pub fn get_root_inode(&self)->&Arc<dyn INode>
    {
        &self.root_inode
    }
}