use std::collections::HashMap;
use dbus::arg::{Variant, RefArg};
use dbus::blocking::Connection;
use std::time::Duration;
use std::fmt;

type DbusOptions = HashMap<String, Variant<Box<dyn RefArg>>>;

pub struct Ip4Config<'a> {
    pub path: dbus::Path<'a>,
    pub nameserver: String
}

impl<'a> Ip4Config<'a> {
    pub fn from_path(p: dbus::Path<'a>) -> Option<Self> {
        if p.eq(&dbus::strings::Path::new("/").unwrap()){
            return None;
        }
        let nameserver = self::Ip4Config::get_nameserver(&p);

        Some(Ip4Config {
            path: p,
            nameserver: nameserver.to_string()
        })
    }

    fn get_nameserver<'f>(p: &'a dbus::Path ) -> String {
        use crate::nm_ip4_config::OrgFreedesktopNetworkManagerIP4Config;
        let con = Connection::new_system().unwrap();
        let proxy = con.with_proxy("org.freedesktop.NetworkManager",
                                   p,
                                   Duration::new(5, 0));

        let nameserver: Vec<DbusOptions> = proxy.nameserver_data().unwrap();
        for data in nameserver {
            if data.contains_key("address") {
                let addr_data = data.get_key_value("address").unwrap();
                let ret = addr_data.1.as_str().clone().unwrap();
                return ret.to_string();
            }
        }
        "".to_string()
    }
}

impl fmt::Debug for Ip4Config<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Ip4Config")
            .field("Path", &self.path)
            .field("Nameserver", &self.nameserver)
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::NetworkManager;

    #[test]
    fn test_get_nameserver() {
        let manager = NetworkManager::new_system();
        let device = manager.get_device_by_ip_iface("wlp2s0").unwrap();
        //println!("{:?}", device.ip4_config);
        assert_eq!(device.ip4_config.unwrap().nameserver, "192.168.2.1".to_string())
    }
}