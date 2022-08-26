//#[cfg(feature = "namespace")]
//This mod is try to let zcore have namespace and cgroup

use super::*;
use lazy_static::*;
mod namespace;
mod cgroup;

use alloc::vec::Vec;
use alloc::sync::Arc;
use hashbrown::HashMap;
use zircon_object::object::KObjectBase;

pub type KoID = u64;
pub struct NsManager{
    ns_hash:HashMap<KoID,Arc<NsProxy>>,
    init_ns:KoID,
}
impl NsManager{
    pub fn init()->Arc<Self>{
        Arc::new(NsManager{
            ns_hash:HashMap::new(),
            init_ns:0,
        })
    }
    pub fn get_ns_proxy(self,ns_id:KoID)->Option<&Arc<NsProxy>>
    {
        match self.ns_hash.get(&ns_id).unwrap()
        {
            Some(ns_proxy) => ns_proxy.clone(),
            None => return Err(ZxError::BAD_HANDLE),
        }
    }
}
lazy_static!{
    pub static ref NS_MANAGER:Arc<NsManager>= NsManager::init();
}
pub struct NsProxy{
    
}