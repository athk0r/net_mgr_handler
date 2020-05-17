use dbus::blocking::Connection;
use crate::network_device::OrgFreedesktopNetworkManagerDeviceWireless;
use std::time::Duration;
use std::collections::HashMap;
use std::fmt;
use dbus::arg::{Variant, RefArg};

type DbusOptions<'a> = HashMap<&'a str, Variant<Box<dyn RefArg>>>;

pub struct Device<'a> {
    path: dbus::Path<'a>,
}

impl<'a> Device<'a> {
    pub fn new_from_path(p: dbus::Path<'a>) -> Self {
        Device { path: p }
    }

    pub fn scan(&self) -> () {
        let con = Connection::new_system().unwrap();
        let proxy = con.with_proxy("org.freedesktop.NetworkManager",
                                   &self.path,
                                   Duration::new(5, 0));

        let options: DbusOptions = HashMap::new();
        let old_last_scan = proxy.last_scan().unwrap();
        let _result = proxy.request_scan(options);
        let mut new_last_scan = proxy.last_scan().unwrap();

        while old_last_scan == new_last_scan {
            new_last_scan = proxy.last_scan().unwrap();
        }
    }

    pub fn get_all_access_points(&self) -> Vec<dbus::Path<'static>>{
        let con = Connection::new_system().unwrap();
        let proxy = con.with_proxy("org.freedesktop.NetworkManager",
                                   &self.path,
                                   Duration::new(5, 0));
        let access_points: Vec<dbus::Path<'static>> = proxy.get_all_access_points().unwrap();
        access_points
    }
}

impl fmt::Debug for Device<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Device")
            .field("Path", &self.path)
            .finish()
    }
}

mod tests {
    use super::*;
    use crate::NetworkManager;

    #[test]
    fn test_scan() {
        let manager = NetworkManager::new_system();
        let device = manager.get_device_by_ip_iface("wlp2s0").unwrap();
        device.scan();
        assert!(device.get_all_access_points().len() > 1);
    }

    #[test]
    fn test_get_all_access_points() {
        let manager = NetworkManager::new_system();
        let device = manager.get_device_by_ip_iface("wlp2s0").unwrap();
        device.scan();
        let aps = device.get_all_access_points();
        println!("AccessPoints: {:#?}", aps);
        assert!(aps.len() > 1);
    }
}