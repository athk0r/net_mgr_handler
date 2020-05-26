use dbus::blocking::{Connection};
use crate::nm_networkmanager::OrgFreedesktopNetworkManager;
use std::time::Duration;
use crate::device::{Device, WirelessDevice};
use dbus::Error;
use crate::access_point::AccessPoint;
use std::collections::HashMap;
use dbus::arg::{Variant, RefArg};
use std::process::Command;
use crate::setting::Setting;
use std::convert::{Infallible, TryInto};
use std::ops::{Deref, DerefMut};
use std::borrow::Borrow;
use std::convert::TryFrom;

type DbusOptions<'a> = HashMap<&'a str, Variant<Box<dyn RefArg>>>;

const PATH_NETWORK_MANAGER: &str = "/org/freedesktop/NetworkManager";
const PATH_NETWORK_MANAGER_SETTINGS: &str = "/org/freedesktop/NetworkManager/Settings";

macro_rules! call_on_proxy {
    ($self:ident, $path:ident) => {
        $self.con.with_proxy("org.freedesktop.NetworkManager",
                            $path,
                            Duration::new(5, 0))
    }
}

fn string_into_dbus_option(s: &str) -> Variant<Box<dyn RefArg>> {
    let b = Box::new(s.to_string()) as Box<dyn RefArg>;
    let v: Variant<Box<dyn RefArg>> = Variant(b);
    v
}

fn get_connection_settings_802_11_wireless(psk: &str) -> HashMap<&'static str, HashMap<&'static str, Variant<Box<dyn RefArg>>>> {
    let mut ret: HashMap<&str, DbusOptions> = HashMap::new();
    let mut wireless_options: DbusOptions = HashMap::new();
    let mut security_options: DbusOptions = HashMap::new();

    wireless_options.insert("security", string_into_dbus_option("802-11-wireless-security"));

    security_options.insert("key-mgmt", string_into_dbus_option("wpa-psk"));
    security_options.insert("psk", string_into_dbus_option(psk));
    ret.insert("802-11-wireless", wireless_options);
    ret.insert("802-11-wireless-security", security_options);
    ret
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

    pub fn add_and_activate_wifi_connection(&self, wd: WirelessDevice, ap: AccessPoint, psk: &str) {
        let con = get_connection_settings_802_11_wireless(psk);
        //let dev = self::NetworkManager::get_device_by_ip_iface(&self,"wlp2s0").unwrap();
        //let dev_path = dev.path.clone();
        //let wifi = WirelessDevice::new_from_device(&dev);
        //let so = wifi.get_access_point_by_ssid("Compact_bd3e".to_string()).unwrap();
        let wd_path = wd.device.path.clone();
        let result = call_on_proxy!(self, PATH_NETWORK_MANAGER).add_and_activate_connection(con, wd_path, ap.path);
        println!("Connect result: {:?}", result);
    }

    pub fn activate_connection(&self, wd: WirelessDevice, ap: AccessPoint) {
        let con: dbus::Path = dbus::strings::Path::new("/").unwrap();
        let wd_path = wd.device.path.clone();
        let result = call_on_proxy!(self, PATH_NETWORK_MANAGER).activate_connection(con, wd_path, ap.path);
    }

    pub fn deactivate_connection(&self, d: Device) -> Result<(), Error>{
        if d.active_connection.is_some() {
            let result = call_on_proxy!(self, PATH_NETWORK_MANAGER).deactivate_connection(d.active_connection.unwrap().path);
            return Ok(());
        } else {
            return Err(Error::new_failed("No Active Connection"));
        }
    }

    pub fn connect_wifi(&self, wd: WirelessDevice, ap: AccessPoint, psk: &str) {

    }

    pub fn find_existing_connection(&self, ap: &AccessPoint) -> bool {
        use crate::nm_settings::OrgFreedesktopNetworkManagerSettings;
        let result = call_on_proxy!(self, PATH_NETWORK_MANAGER_SETTINGS).list_connections().unwrap();
        let mut settings = Vec::new();
        for p in result {
            settings.push(Setting::from_path(p).unwrap());
        }
        println!("{:?}", settings);
        for setting in settings {
            if setting.settings.contains_key("802-11-wireless") {
                let wireless_settings = setting.settings.get_key_value("802-11-wireless").unwrap().1;
                let ssid_u8 = wireless_settings.get_key_value("ssid").unwrap().1;
                println!("ssid: {:?}", ssid_u8.0);
                let mut x = ssid_u8.0.deref().as_iter().unwrap();
                let y = x.deref_mut();
                let mut utf8_vec = Vec::new();
                for z in y {
                    println!("{:?}", z.as_u64().unwrap());
                    let m = u8::try_from(z.as_u64().unwrap()).unwrap();
                    println!("m: {:?}", m);
                    utf8_vec.push(m);
                }
                println!("utf8: {:?}", utf8_vec);
                println!("ssid: {:?}", String::from_utf8(utf8_vec));
                //println!("{:?}", x);
                //let x = ssid_u8.0.deref().as_iter().unwrap().deref_mut().map(|x| x.as_any().downcast_ref::<u8>().unwrap());
                //let y = x.as_str();
                //println!("ssid: {:?}",x);
                //let ssid = String::from_utf8(ssid_u8.unwrap()).unwrap();
                //println!("ssid: {:?}", x);
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_all_devices() {
        let manager = NetworkManager::new_system();
        let devices = manager.get_all_devices();
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
    fn test_add_and_activate_wifi_connection() {
        let manager = NetworkManager::new_system();
        let device = manager.get_device_by_ip_iface("wlp2s0").unwrap();
        let wireless_device = WirelessDevice::new_from_device(&device);
        let ap = wireless_device.get_access_point_by_ssid("Compact_bd3e");
        if ap.is_some() {
            manager.add_and_activate_wifi_connection(wireless_device.clone(), ap.unwrap().clone(), "test1234");
            assert!(true);
        } else {
            eprintln!("WARNING: AP not found");
            assert!(false);
        }
    }

    #[test]
    fn test_deactivate_connection() {
        let manager = NetworkManager::new_system();
        let device = manager.get_device_by_ip_iface("wlp2s0").unwrap();
        let result = manager.deactivate_connection(device);
        assert!(true);
    }

    #[test]
    fn test_activate_connection() {
        let manager = NetworkManager::new_system();
        let device = manager.get_device_by_ip_iface("wlp2s0").unwrap();
        let wireless_device = WirelessDevice::new_from_device(&device);
        let ap = wireless_device.get_access_point_by_ssid("UPC22AC955").unwrap();
        let result = manager.activate_connection(wireless_device.clone(), ap);
        assert!(true);
    }

    #[test]
    fn test_find_exisiting_connection() {
        let manager = NetworkManager::new_system();
        let device = manager.get_device_by_ip_iface("wlp2s0").unwrap();
        let wireless_device = WirelessDevice::new_from_device(&device);
        let ap = wireless_device.get_access_point_by_ssid("UPC22AC955").unwrap();
        manager.find_existing_connection(&ap);
        assert!(false);
    }
}