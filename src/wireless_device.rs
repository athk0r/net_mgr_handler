use dbus::blocking::Connection;
use std::time::Duration;
use std::collections::HashMap;
use std::fmt;
use dbus::arg::{Variant, RefArg};
use crate::device_type::DeviceType;
use crate::network_device::OrgFreedesktopNetworkManagerDeviceWireless;

type DbusOptions<'a> = HashMap<&'a str, Variant<Box<dyn RefArg>>>;

pub struct WirelessDevice<'a> {
    path: dbus::Path<'a>,
    active_connection: Option<dbus::Path<'a>>,
    dhcp4_config: dbus::Path<'a>,
    ip4_config: dbus::Path<'a>,
    ip_interface: String,
    hw_address: String,
    active_access_point: Option<dbus::Path<'a>>
}

impl<'a> WirelessDevice<'a> {
    pub fn new_from_path(p: dbus::Path<'a>) -> Self {
        let ac = self::WirelessDevice::get_active_connection(&p);
        let dhcp4_conf = self::WirelessDevice::get_dhcp4_config(&p);
        let ip4_conf = self::WirelessDevice::get_ip4_config(&p);
        let ip_iface = self::WirelessDevice::get_ip_interface(&p);
        let hw_addr = self::WirelessDevice::get_hw_address(&p);

        WirelessDevice {
            path: p,
            active_connection: ac,
            dhcp4_config: dhcp4_conf,
            ip4_config: ip4_conf,
            ip_interface: ip_iface,
            hw_address: hw_addr,
            active_access_point: None
        }
    }

    fn get_active_connection<'f>(p: &'a dbus::Path) -> Option<dbus::Path<'f>> {
        use crate::network_device::OrgFreedesktopNetworkManagerDevice;
        let con = Connection::new_system().unwrap();
        let proxy = con.with_proxy("org.freedesktop.NetworkManager",
                                   p,
                                   Duration::new(5, 0));

        let active_connection = proxy.active_connection().unwrap();
        Some(active_connection)
    }

    fn get_dhcp4_config<'f>(p: &'a dbus::Path) -> dbus::Path<'f> {
        use crate::network_device::OrgFreedesktopNetworkManagerDevice;
        let con = Connection::new_system().unwrap();
        let proxy = con.with_proxy("org.freedesktop.NetworkManager",
                                   p,
                                   Duration::new(5, 0));

        let dhcp4_config = proxy.dhcp4_config().unwrap();
        dhcp4_config
    }

    fn get_ip4_config<'f>(p: &'a dbus::Path) -> dbus::Path<'f> {
        use crate::network_device::OrgFreedesktopNetworkManagerDevice;
        let con = Connection::new_system().unwrap();
        let proxy = con.with_proxy("org.freedesktop.NetworkManager",
                                   p,
                                   Duration::new(5, 0));

        let ip4_config = proxy.ip4_config().unwrap();
        ip4_config
    }

    fn get_ip_interface(p: &dbus::Path) -> String {
        use crate::network_device::OrgFreedesktopNetworkManagerDevice;
        let con = Connection::new_system().unwrap();
        let proxy = con.with_proxy("org.freedesktop.NetworkManager",
                                   p,
                                   Duration::new(5, 0));

        let ip_interface = proxy.ip_interface().unwrap();
        ip_interface
    }

    fn get_hw_address(p: &dbus::Path) -> String {
        use crate::network_device::OrgFreedesktopNetworkManagerDeviceWireless;

        let con = Connection::new_system().unwrap();
        let proxy = con.with_proxy("org.freedesktop.NetworkManager",
                                   p,
                                   Duration::new(5, 0));

        let hw_address = proxy.hw_address().unwrap();
        hw_address
    }

    pub fn scan(&self) -> () {
        use crate::network_device::OrgFreedesktopNetworkManagerDeviceWireless;

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
        use crate::network_device::OrgFreedesktopNetworkManagerDeviceWireless;

        let con = Connection::new_system().unwrap();
        let proxy = con.with_proxy("org.freedesktop.NetworkManager",
                                   &self.path,
                                   Duration::new(5, 0));
        let access_points: Vec<dbus::Path<'static>> = proxy.get_all_access_points().unwrap();
        access_points
    }
}

impl fmt::Debug for WirelessDevice<'_> {
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