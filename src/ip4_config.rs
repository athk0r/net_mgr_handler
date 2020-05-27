use std::collections::HashMap;
use dbus::arg::{Variant, RefArg};
use dbus::blocking::Connection;
use std::time::Duration;
use std::fmt;

type DbusOptions = HashMap<String, Variant<Box<dyn RefArg>>>;

#[derive(Clone)]
pub struct Ip4Config<'a> {
    pub path: dbus::Path<'a>,
    pub ip: String,
    pub nameserver: String
}

impl<'a> Ip4Config<'a> {
    pub fn from_path(p: dbus::Path<'a>) -> Option<Self> {
        if p.eq(&dbus::strings::Path::new("/").unwrap()){
            return None;
        }
        let nameserver = self::Ip4Config::get_nameserver(&p);
        let ip_address = self::Ip4Config::get_ip(&p);

        Some(Ip4Config {
            path: p,
            ip: ip_address.to_string(),
            nameserver: nameserver.to_string()
        })
    }

    fn get_nameserver<'f>(p: &'a dbus::Path) -> String {
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

    fn get_ip<'f>(p: &'a dbus::Path) -> String {
        use crate::nm_ip4_config::OrgFreedesktopNetworkManagerIP4Config;
        let con = Connection::new_system().unwrap();
        let proxy = con.with_proxy("org.freedesktop.NetworkManager",
                                   p,
                                   Duration::new(5, 0));
        let ip_data: Vec<DbusOptions> = proxy.address_data().unwrap();
        for data in ip_data {
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
        let wireless_device = crate::device::WirelessDevice::new_from_device(&device);
        let ap = wireless_device.get_access_point_by_ssid("UPC22AC955").unwrap();
        let result = manager.activate_connection(wireless_device.clone(), ap);
        manager.deactivate_connection(device.clone());
        assert_eq!(device.ip4_config.unwrap().nameserver, "192.168.0.1".to_string())
    }
}