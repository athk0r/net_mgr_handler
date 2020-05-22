use dbus::blocking::{Connection};
use crate::nm_networkmanager::OrgFreedesktopNetworkManager;
use std::time::Duration;
use crate::device::{Device, WirelessDevice};
use dbus::Error;
use crate::access_point::AccessPoint;
use std::collections::HashMap;
use dbus::arg::{Variant, RefArg};
use std::process::Command;

type DbusOptions<'a> = HashMap<&'a str, Variant<Box<dyn RefArg>>>;

const PATH_NETWORK_MANAGER: &str = "/org/freedesktop/NetworkManager";

macro_rules! call_on_proxy {
    ($self:ident, $path:ident) => {
        $self.con.with_proxy("org.freedesktop.NetworkManager",
                            $path,
                            Duration::new(5, 0))
    }
}

pub struct NetworkManager {
    con: Connection,
}

impl NetworkManager{
    pub fn new_system() -> Self {
        let connection: dbus::blocking::Connection = Connection::new_system().unwrap();
        NetworkManager{ con: connection }
    }

    pub fn get_all_devices(&self) -> Result<Vec<Device<'static>>, Error> {
        let devices: Vec<dbus::Path> = call_on_proxy!(self, PATH_NETWORK_MANAGER).get_all_devices()?;
        let mut ret: Vec<Device> = Vec::new();
        for device in devices {
            ret.push(Device::new_from_path(device));
        }
        Ok(ret)
    }

    pub fn get_device_by_ip_iface(&self, s: &str) -> Result<Device<'static>, Error> {
        let device_path: dbus::Path = call_on_proxy!(self, PATH_NETWORK_MANAGER).get_device_by_ip_iface(s)?;
        Ok(Device::new_from_path(device_path))
    }

    pub fn add_and_activate_connection(&self/*, d: Device, ap: AccessPoint*/) {
        let mut con: HashMap<&str, DbusOptions> = HashMap::new();
        let y = Box::new("802-11-wireless-security".to_string()) as Box<dyn RefArg>;
        let z: Variant<Box<dyn RefArg>> = Variant(y);
        let mut x: DbusOptions = HashMap::new();
        x.insert("security", z);
        con.insert("802-11-wireless", x);
        let mut zz: DbusOptions = HashMap::new();
        zz.insert("key-mgmt", Variant(Box::new("wpa-psk".to_string()) as Box<dyn RefArg>));
        zz.insert("psk", Variant(Box::new("test1234".to_string()) as Box<dyn RefArg>));
        con.insert("802-11-wireless-security", zz);
        let dev = self::NetworkManager::get_device_by_ip_iface(&self,"wlp2s0").unwrap();
        let dev_path = dev.path.clone();
        let wifi = WirelessDevice::new_from_device(&dev);
        let so = wifi.get_access_point_by_ssid("Compact_bd3e".to_string()).unwrap();
        println!("{:?}", Command::new("sh").arg("id").output());
        let result = call_on_proxy!(self, PATH_NETWORK_MANAGER).add_and_activate_connection(con, dev_path, so.path);
        println!("Connect result: {:?}", result);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_all_devices() {
        let manager = NetworkManager::new_system();
        let devices = manager.get_all_devices();
        //println!("{:?}", devices);
        assert!(devices.unwrap().len()>0);
    }

    #[test]
    fn test_get_device_by_ip_iface() {
        let manager = NetworkManager::new_system();
        let device = manager.get_device_by_ip_iface("wlp2s0");
        //println!("{:?}", device);
        assert!(device.is_ok());
    }

    #[test]
    fn test_add_and_connect() {
        let manager = NetworkManager::new_system();
        manager.add_and_activate_connection();
        assert!(true);
    }
}