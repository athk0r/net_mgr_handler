use dbus::blocking::Connection;
use std::time::Duration;
use std::collections::HashMap;
use std::fmt;
use dbus::arg::{Variant, RefArg};
use crate::device_type::DeviceType;
use crate::ip4_config::Ip4Config;
use crate::dhcp4_config::Dhcp4Config;
use crate::active_connection::ActiveConnection;
use crate::access_point::AccessPoint;

type DbusOptions<'a> = HashMap<&'a str, Variant<Box<dyn RefArg>>>;

pub struct Device<'a> {
    pub path: dbus::Path<'a>,
    pub active_connection: Option<ActiveConnection<'a>>,
    pub dhcp4_config: Option<Dhcp4Config<'a>>,
    pub ip4_config: Option<Ip4Config<'a>>,
    pub ip_interface: String,
    pub device_type: DeviceType
}

pub struct WirelessDevice<'a> {
    pub device: &'a Device<'a>,
    pub hw_address: String,
    pub active_access_point: Option<AccessPoint<'a>>
}

impl<'a> Device<'a> {
    pub fn new_from_path(p: dbus::Path<'a>) -> Self {
        let ac = self::Device::get_active_connection(&p);
        let dhcp4_conf = self::Device::get_dhcp4_config(&p);
        let ip4_conf = self::Device::get_ip4_config(&p);
        let ip_iface = self::Device::get_ip_interface(&p);
        let device_type = self::Device::get_device_type(&p);

        Device {
            path: p,
            active_connection: ActiveConnection::from_path(ac),
            dhcp4_config: Dhcp4Config::from_path(dhcp4_conf),
            ip4_config: Ip4Config::from_path(ip4_conf),
            ip_interface: ip_iface,
            device_type: device_type
        }
    }

    fn get_active_connection<'f>(p: &'a dbus::Path) -> dbus::Path<'f> {
        use crate::nm_device::OrgFreedesktopNetworkManagerDevice;
        let con = Connection::new_system().unwrap();
        let proxy = con.with_proxy("org.freedesktop.NetworkManager",
                                   p,
                                   Duration::new(5, 0));

        let active_connection = proxy.active_connection().unwrap();
        active_connection
    }

    fn get_dhcp4_config<'f>(p: &'a dbus::Path) -> dbus::Path<'f> {
        use crate::nm_device::OrgFreedesktopNetworkManagerDevice;
        let con = Connection::new_system().unwrap();
        let proxy = con.with_proxy("org.freedesktop.NetworkManager",
                                   p,
                                   Duration::new(5, 0));

        let dhcp4_config = proxy.dhcp4_config().unwrap();
        dhcp4_config
    }

    fn get_ip4_config<'f>(p: &'a dbus::Path) -> dbus::Path<'f> {
        use crate::nm_device::OrgFreedesktopNetworkManagerDevice;
        let con = Connection::new_system().unwrap();
        let proxy = con.with_proxy("org.freedesktop.NetworkManager",
                                   p,
                                   Duration::new(5, 0));

        let ip4_config = proxy.ip4_config().unwrap();
        ip4_config
    }

    fn get_ip_interface(p: &dbus::Path) -> String {
        use crate::nm_device::OrgFreedesktopNetworkManagerDevice;
        let con = Connection::new_system().unwrap();
        let proxy = con.with_proxy("org.freedesktop.NetworkManager",
                                   p,
                                   Duration::new(5, 0));

        let ip_interface = proxy.ip_interface().unwrap();
        ip_interface
    }

    fn get_device_type(p: &dbus::Path) -> DeviceType {
        use crate::nm_device::OrgFreedesktopNetworkManagerDevice;
        let con = Connection::new_system().unwrap();
        let proxy = con.with_proxy("org.freedesktop.NetworkManager",
                                   p,
                                   Duration::new(5, 0));
        let device_type: DeviceType = DeviceType::from(proxy.device_type().unwrap());
        device_type
    }
}

impl<'a> WirelessDevice<'a> {
    pub fn new_from_device(d: &'a Device) -> Self {
        if d.device_type != DeviceType::WiFi {
            eprintln!("Device is no WiFi Device");
            panic!("Device is no WiFi Device");
        }
        let hw_addr = self::WirelessDevice::get_hw_address(&d.path);

        WirelessDevice {
            device: d,
            hw_address: hw_addr,
            active_access_point: None
        }
    }

    fn get_hw_address(p: &dbus::Path) -> String {
        use crate::nm_device::OrgFreedesktopNetworkManagerDeviceWireless;

        let con = Connection::new_system().unwrap();
        let proxy = con.with_proxy("org.freedesktop.NetworkManager",
                                   p,
                                   Duration::new(5, 0));

        let hw_address = proxy.hw_address().unwrap();
        hw_address
    }

    pub fn scan(&self) -> () {
        use crate::nm_device::OrgFreedesktopNetworkManagerDeviceWireless;

        let con = Connection::new_system().unwrap();
        let proxy = con.with_proxy("org.freedesktop.NetworkManager",
                                   &self.device.path,
                                   Duration::new(5, 0));

        let options: DbusOptions = HashMap::new();
        let old_last_scan = proxy.last_scan().unwrap();
        let _result = proxy.request_scan(options);
        let mut new_last_scan = proxy.last_scan().unwrap();

        while old_last_scan == new_last_scan {
            new_last_scan = proxy.last_scan().unwrap();
        }
    }

    pub fn get_all_access_points(&self) -> Vec<Option<AccessPoint>>{
        use crate::nm_device::OrgFreedesktopNetworkManagerDeviceWireless;

        let con = Connection::new_system().unwrap();
        let proxy = con.with_proxy("org.freedesktop.NetworkManager",
                                   &self.device.path,
                                   Duration::new(5, 0));
        let access_points: Vec<dbus::Path<'static>> = proxy.get_all_access_points().unwrap();
        let mut ret: Vec<Option<AccessPoint>> = Vec::new();
        for ac in access_points {
            ret.push(AccessPoint::from_path(ac));
        }
        ret
    }

    pub fn get_active_access_point(&mut self) {
        use crate::nm_device::OrgFreedesktopNetworkManagerDeviceWireless;

        let con = Connection::new_system().unwrap();
        let proxy = con.with_proxy("org.freedesktop.NetworkManager",
                                   &self.device.path,
                                   Duration::new(5, 0));
        let active_ap = AccessPoint::from_path(proxy.active_access_point().unwrap());
        self.active_access_point = active_ap;
    }

    pub fn get_access_point_by_ssid(&self, s: String) -> Option<AccessPoint>{
        self::WirelessDevice::scan(&self);
        let all_ap = self::WirelessDevice::get_all_access_points(&self);
        for ap in all_ap {
            let tmp_ap = ap.clone();
            if tmp_ap.is_some() && tmp_ap.unwrap().ssid.eq(&s) {
                return ap.clone();
            }
        }
        None
    }
}

impl fmt::Debug for Device<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Device")
            .field("Path", &self.path)
            .field("Active Connection", &self.active_connection)
            .field("DHCP4Config", &self.dhcp4_config)
            .field("IP4Config", &self.ip4_config)
            .field("Interface", &self.ip_interface)
            .field("DeviceType", &self.device_type)
            .finish()
    }
}

impl fmt::Debug for WirelessDevice<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("WirelessDevice")
            .field("Device", &self.device)
            .field("HwAddress", &self.hw_address)
            .field("ActiveAccessPoint", &self.active_access_point)
            .finish()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::NetworkManager;

    #[test]
    fn test_scan() {
        let manager = NetworkManager::new_system();
        let device = manager.get_device_by_ip_iface("wlp2s0").unwrap();
        let wireless_device = WirelessDevice::new_from_device(&device);
        wireless_device.scan();
        assert!(wireless_device.get_all_access_points().len() > 1);
    }

    #[test]
    fn test_get_all_access_points() {
        let manager = NetworkManager::new_system();
        let device = manager.get_device_by_ip_iface("wlp2s0").unwrap();
        let wireless_device = WirelessDevice::new_from_device(&device);
        wireless_device.scan();
        let aps = wireless_device.get_all_access_points();
        println!("AccessPoints: {:#?}", aps);
        assert!(aps.len() > 1);
    }
}