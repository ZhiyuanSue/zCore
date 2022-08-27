use alloc::string::ToString;
//#[cfg(feature = "namespace")]
use bitflags::bitflags;
use super::*;
//https://man7.org/linux/man-pages/man2/setns.2.html
bitflags! {
    pub struct NSType:u8{
        const NSTypeAny =           0;
        const CLONE_NEWCGROUP =     1;
        const CLONE_NEWIPC =     1<<1;
        const CLONE_NEWNET =     1<<2;
        const CLONE_NEWNS =      1<<3;
        const CLONE_NEWPID =     1<<4;
        const CLONE_NEWTIME =    1<<5;
        const CLONE_NEWUSER =    1<<6;
        const CLONE_NEWUTS =     1<<7;
    }
}
pub struct NsBase{
    base: KObjectBase,
    nstype:NSType,  //it should use the namespace.rs::NSType
    parent:Option<Arc<NsEnum>>,   //the parent might be none,so use option
    child_ns_vec:Arc<Vec<NsEnum>>,
}
impl NsBase{
    pub fn new(nstype:NSType,parent:Option<Arc<NsEnum>>)->Self{
        NsBase { 
            base: KObjectBase::new(), 
            nstype: NSType::NSTypeAny, 
            parent: parent, 
            child_ns_vec: Arc::new(Vec::new()),
        }
    }
}
pub trait NS :Send + Sync{
    fn get_ns_id(&self)->KoID;
    fn get_ns_type(&self)->NSType;
    fn get_ns_base(&self)->NsBase;
    fn get_parent_ns(&self)->Option<KoID>;
    fn get_ns_instance(&self)->NsEnum;
}

pub struct MntNs
{
    base:NsBase,
    //rootfs:Inode,

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
    fn get_ns_base(&self)->NsBase
    {
        self.base
    }
    fn get_parent_ns(&self)->Option<KoID>
    {
        match self.base.parent{
            Some(base)=> {
                let nsenum=*base.as_ref();
                match nsenum.into(){
                    NsEnum::MntNs(m) => {
                        return Some(m.base.base.id)
                    },
                    _ => {return None;}    //the parent must be a mnt ns struct 
                }
            },
            None=> {return None;},
        }
    }
    fn get_ns_instance(&self)->NsEnum
    {
        NsEnum::MntNs(*self)
    }
}
impl MntNs{
    fn new(parent:Option<Arc<NsEnum>>)->Self
    {
        let mntns=MntNs{
            base:NsBase::new(NSType::CLONE_NEWNS,parent),

        };
        mntns
    }
    pub fn new_root()->Self
    {
        let root=MntNs::new(None);
        root
    }
    pub fn new_child(self)->MntNs
    {
        let child = MntNs::new(Some(Arc::new(self.get_ns_instance())));

        child
    }
}
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
    fn get_ns_base(&self)->NsBase
    {
        self.base
    }
    fn get_parent_ns(&self)->Option<KoID>
    {
        match self.base.parent{
            Some(base)=> {
                let nsenum=*base.as_ref();
                match nsenum.into(){
                    NsEnum::UtsNs(u) => {
                        return Some(u.base.base.id)
                    },
                    _ => {return None;}    //the parent must be a uts ns struct 
                }
            },
            None=> {return None;},
        }
    }
    fn get_ns_instance(&self)->NsEnum
    {
        NsEnum::UtsNs(*self)
    }
}
impl UtsNs{
    fn new(parent:Option<Arc<NsEnum>>,hostname:String,kernel_version:String)->Self
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
        let child = UtsNs::new(Some(Arc::new(self.get_ns_instance())),
            self.hostname,
            self.kernel_version,
        );
        child
    }
}
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
    fn get_ns_base(&self)->NsBase
    {
        self.base
    }
    fn get_parent_ns(&self)->Option<KoID>
    {
        match self.base.parent{
            Some(base)=> {
                let nsenum=*base.as_ref();
                match nsenum.into(){
                    NsEnum::IpcNs(i) => {
                        return Some(i.base.base.id)
                    },
                    _ => {return None;}    //the parent must be a mnt ns struct 
                }
            },
            None=> {return None;},
        }
    }
    fn get_ns_instance(&self)->NsEnum
    {
        NsEnum::IpcNs(*self)
    }
}
impl IpcNs{
    fn new(parent:Option<Arc<NsEnum>>)->Self
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
    pub fn new_child(self)->IpcNs
    {
        let child = IpcNs::new(Some(Arc::new(self.get_ns_instance())));

        child
    }
}

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
    fn get_ns_base(&self)->NsBase
    {
        self.base
    }
    fn get_parent_ns(&self)->Option<KoID>
    {
        match self.base.parent{
            Some(base)=> {
                let nsenum=*base.as_ref();
                match nsenum.into(){
                    NsEnum::PidNs(p) => {
                        return Some(p.base.base.id)
                    },
                    _ => {return None;}    //the parent must be a mnt ns struct 
                }
            },
            None=> {return None;},
        }
    }
    fn get_ns_instance(&self)->NsEnum
    {
        NsEnum::PidNs(*self)
    }
}
impl PidNs{
    fn new(parent:Option<Arc<NsEnum>>)->Self
    {
        let pidns=PidNs{
            base:NsBase::new(NSType::CLONE_NEWNS,parent),
            pid_map:HashMap::new(),
        };
        pidns
    }
    pub fn new_root()->Self
    {
        let root=PidNs::new(None);
        root
    }
    pub fn new_child(self)->PidNs
    {
        let child = PidNs::new(Some(Arc::new(self.get_ns_instance())));

        child
    }
}
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
    fn get_ns_base(&self)->NsBase
    {
        self.base
    }
    fn get_parent_ns(&self)->Option<KoID>
    {
        match self.base.parent{
            Some(base)=> {
                let nsenum=*base.as_ref();
                match nsenum.into(){
                    NsEnum::NetNs(n) => {
                        return Some(n.base.base.id)
                    },
                    _ => {return None;}    //the parent must be a mnt ns struct 
                }
            },
            None=> {return None;},
        }
    }
    fn get_ns_instance(&self)->NsEnum
    {
        NsEnum::NetNs(*self)
    }
}
impl NetNs{
    fn new(parent:Option<Arc<NsEnum>>)->Self
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
    pub fn new_child(self)->NetNs
    {
        let child = NetNs::new(Some(Arc::new(self.get_ns_instance())));

        child
    }
}
pub struct UsrNs
{
    base:NsBase,
    usrname:String,
}

pub enum NsEnum{
    MntNs(MntNs),
    UtsNs(UtsNs),
    IpcNs(IpcNs),
    PidNs(PidNs),
    NetNs(NetNs),
    UsrNs(UsrNs),
}