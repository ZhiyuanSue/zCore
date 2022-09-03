
//#[cfg(feature = "namespace")]
#![allow(dead_code, unused_imports)]
use super::*;
use linux_object::namespace::*;
use linux_object::namespace::{
    utsns::*,
};
impl Syscall<'_> {
    //uts ns
    /// set host name in uts namespace
    pub fn sethostname(&self,base: UserInPtr<u8>, len: usize)-> SysResult
    {
        let inner=self.thread.inner();
        let proc=inner.proc();
        let uts_ns_id=proc.linux().nsproxy_get().get_proxy_ns(NSType::CLONE_NEWUTS);
        match uts_ns_id
        {
            Some(id)=>{
                let res=set_host_name(id,base,len);
                match res{
                    Some(_)=>{
                        Ok(0)
                    }
                    None=>{
                        Err(LxError::EUNDEF)
                    }
                }
            }
            None=>{
                Err(LxError::EUNDEF)
            }
        }
    }
    /// set domain name in uts ns
    pub fn setdomainname(&self,base: UserInPtr<u8>, len: usize)->SysResult
    {
        let inner=self.thread.inner();
        let proc=inner.proc();
        let uts_ns_id=proc.linux().nsproxy_get().get_proxy_ns(NSType::CLONE_NEWUTS);
        match uts_ns_id{
            Some(id)=>{
                let res=set_domain_name(id,base,len);
                match res{
                    Some(_)=>{
                        Ok(0)
                    }
                    None=>{
                        Err(LxError::EUNDEF)
                    }
                }
            }
            None=>{
                Err(LxError::EUNDEF)
            }
        }
        
    }
}