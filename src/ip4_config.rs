use dbus::arg::{RefArg, Variant};
use dbus::blocking::Connection;
use std::collections::HashMap;
use std::fmt;
use std::time::Duration;

type DbusOptions = HashMap<String, Variant<Box<dyn RefArg>>>;

#[derive(Clone)]
pub struct Ip4Config<'a> {
    pub path: dbus::Path<'a>,
    pub ip: String,
    pub nameserver: String,
    pub next_hop: String,
}

impl<'a> Ip4Config<'a> {
    pub fn from_path(p: dbus::Path<'a>) -> Option<Self> {
        if p.eq(&dbus::strings::Path::new("/").unwrap()) {
            return None;
        }
        let nameserver = self::Ip4Config::get_nameserver(&p);
        let ip_address = self::Ip4Config::get_ip(&p);
        let next_hop = self::Ip4Config::get_next_hop(&p);

        Some(Ip4Config {
            path: p,
            ip: ip_address.to_string(),
            nameserver: nameserver.to_string(),
            next_hop: next_hop,
        })
    }

    fn get_nameserver<'f>(p: &'a dbus::Path) -> String {
        use crate::nm_ip4_config::OrgFreedesktopNetworkManagerIP4Config;
        let con = Connection::new_system().unwrap();
        let proxy = con.with_proxy("org.freedesktop.NetworkManager", p, Duration::new(5, 0));

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
        let proxy = con.with_proxy("org.freedesktop.NetworkManager", p, Duration::new(5, 0));
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

    /*pub fn get_route_data<'f>(p: &'a dbus::Path) -> Vec<::std::collections::HashMap<String, dbus::arg::Variant<Box<dyn dbus::arg::RefArg + 'static>>>> {
        use crate::nm_ip4_config::OrgFreedesktopNetworkManagerIP4Config;
        let con = Connection::new_system().unwrap();
        let proxy = con.with_proxy("org.freedesktop.NetworkManager", p, Duration::new(5,0));
        let route_data: Vec<DbusOptions> = proxy.route_data().unwrap();
        let mut data: Vec<HashMap<String, _>>;
        for x in &route_data {
            println!("{:?}", x);
            let mut hm: HashMap<String, Box<dyn Any>> = HashMap::new();
            for y in x.keys() {
                if let Some(s) = x.get(y).unwrap().0.as_str() { hm.insert(y.clone(), Box::new(s)); }
                else if let Some(i) = x.get(y).unwrap().0.as_i64() { hm.insert(y.clone(), Box::new(i)); }
                else { hm.insert(y.clone(), Box::<dyn Any>::new(x.get(y).unwrap().0.into())); }
            }
            data.push(hm);
        }
        println!("");
        route_data
    }*/

    fn get_next_hop<'f>(p: &'a dbus::Path) -> String {
        use crate::nm_ip4_config::OrgFreedesktopNetworkManagerIP4Config;
        let con = Connection::new_system().unwrap();
        let proxy = con.with_proxy("org.freedesktop.NetworkManager", p, Duration::new(5, 0));
        let route_data: Vec<DbusOptions> = proxy.route_data().unwrap();

        for data in route_data {
            let dest = data.get("dest");
            if data.contains_key("next-hop")
                && dest.is_some()
                && dest.unwrap().0.as_str().unwrap() == "0.0.0.0"
            {
                let next_hop = data.get("next-hop").unwrap();
                let return_string = next_hop.0.as_str().clone().unwrap();
                return return_string.to_string();
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
    use crate::NetworkManager;

    #[test]
    fn test_get_nameserver() {
        let manager = NetworkManager::new_system();
        let device = manager.get_device_by_ip_iface("wlp2s0").unwrap();
        let wireless_device = crate::device::WirelessDevice::new_from_device(&device);
        let ap = wireless_device
            .get_access_point_by_ssid("UPC22AC955")
            .unwrap();
        let _ = manager.activate_connection(wireless_device.clone(), ap);
        let _ = manager.deactivate_connection(device.clone());
        assert_eq!(
            device.ip4_config.unwrap().nameserver,
            "192.168.0.1".to_string()
        )
    }
}
