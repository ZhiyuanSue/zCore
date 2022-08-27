//#[cfg(feature = "namespace")]
//This mod is try to let zcore have namespace and cgroup
mod namespace;
mod cgroup;

use super::*;
use lazy_static::*;
use namespace::*;
use cgroup::*;

use alloc::vec::Vec;
use alloc::string::String;
use alloc::sync::Arc;
use hashbrown::HashMap;
use zircon_object::object::KObjectBase;

pub type KoID = u64;
pub struct NsManager{
    ns_hash:HashMap<KoID,Arc<dyn NS + Send + Sync>>,
    init_ns:KoID,
}
impl NsManager{
    pub fn init()->Arc<Self>{
        Arc::new(NsManager{
            ns_hash:HashMap::new(),
            init_ns:0,
        })
    }
    pub fn get_ns_proxy(&self,ns_id:KoID)->Option<&Arc<NsProxy>>
    {
        match self.ns_hash.get(&ns_id).unwrap()
        {
            Some(ns_proxy) => ns_proxy.clone(),
            None => return Err(ZxError::BAD_HANDLE),
        }
    }
    pub fn insert(self,ns: Arc<&dyn NS>)->KoID
    {
        let id=ns.get_ns_id();
        self.ns_hash.insert(id,Arc::new(ns));
        id
    }
}
lazy_static!{
    pub static ref NS_MANAGER:Arc<NsManager>= NsManager::init();
}
pub struct NsProxy{
    
}