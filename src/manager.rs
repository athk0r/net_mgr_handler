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

    pub fn add_and_activate_wifi_connection(&self, wd: WirelessDevice, ap: AccessPoint, psk: &str) -> Result<(), Error> {
        let con = get_connection_settings_802_11_wireless(psk);
        let wd_path = wd.device.path.clone();
        let result: Result<(dbus::Path, dbus::Path), dbus::Error>= call_on_proxy!(self, PATH_NETWORK_MANAGER).add_and_activate_connection(con, wd_path, ap.path);
        println!("Connect result: {:?}", result);
        if result.is_ok() {
            return Ok(())
        }
        Err(result.err().unwrap())
    }

    pub fn activate_connection(&self, wd: WirelessDevice, ap: AccessPoint) -> Result<(), Error>{
        let con: dbus::Path = dbus::strings::Path::new("/").unwrap();
        let wd_path = wd.device.path.clone();
        let result: Result<dbus::Path, dbus::Error> = call_on_proxy!(self, PATH_NETWORK_MANAGER).activate_connection(con, wd_path, ap.path);
        if result.is_ok() {
            return Ok(());
        }
        Err(Error::new_failed("Error activating Connection"))
    }

    pub fn deactivate_connection(&self, d: Device) -> Result<(), Error>{
        if d.active_connection.is_some() {
            let result: Result<(), Error> = call_on_proxy!(self, PATH_NETWORK_MANAGER).deactivate_connection(d.active_connection.unwrap().path);
            result
        } else {
            return Err(Error::new_failed("No Active Connection"));
        }
    }

    pub fn connect_wifi(&self, wd: WirelessDevice, ap: AccessPoint, psk: &str) -> Result<(), Error>{
        if self::NetworkManager::find_existing_connection(&self, &ap) {
            return self::NetworkManager::activate_connection(&self, wd, ap);
        } else {
            self::NetworkManager::add_and_activate_wifi_connection(&self, wd, ap, psk);
        }
        return Err(Error::new_failed("Error connecting to wifi"))
    }

    pub fn find_existing_connection(&self, ap: &AccessPoint) -> bool {
        use crate::nm_settings::OrgFreedesktopNetworkManagerSettings;
        let setting_paths = call_on_proxy!(self, PATH_NETWORK_MANAGER_SETTINGS).list_connections().unwrap();
        let mut settings = Vec::new();
        for path in setting_paths {
            settings.push(Setting::from_path(path).unwrap());
        }
        //println!("{:?}", settings);
        for setting in settings {
            let mut ssid: String;
            let mut bssids: Vec<&str>;

            if setting.settings.contains_key("802-11-wireless") {
                let wireless_settings = setting.settings.get_key_value("802-11-wireless").unwrap().1;
                //Get SSID from WirelessSettings
                let ssid_variant = wireless_settings.get_key_value("ssid").unwrap().1;
                //println!("ssid: {:?}", ssid_variant.0);
                let mut boxed_iter = ssid_variant.0.deref().as_iter().unwrap();
                let iter = boxed_iter.deref_mut();
                let mut utf8_vec = Vec::new();
                for item in iter {
                    let utf8_char = u8::try_from(item.as_u64().unwrap()).unwrap();
                    utf8_vec.push(utf8_char);
                }
                //println!("ssidWord: {:?}", String::from_utf8(utf8_vec));
                ssid = String::from_utf8(utf8_vec).unwrap();
                //Get BSSID's from WirelessSettings
                let bssid_variant = wireless_settings.get_key_value("seen-bssids").unwrap().1;
                let mut bssid_iter = bssid_variant.0.deref().as_iter().unwrap();
                bssids = bssid_iter.deref_mut().map(|x| x.as_str().unwrap()).collect();
                // Test if Setting matches AccessPoint
                if ssid.eq(&ap.ssid) && bssids.contains(&ap.hw_address.as_str()) {
                    return true;
                }
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::nm_device::OrgFreedesktopNetworkManagerDeviceWirelessAccessPointRemoved;

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
        assert!(device.is_ok());
    }

    #[test]
    fn test_add_and_activate_wifi_connection() {
        let manager = NetworkManager::new_system();
        let device = manager.get_device_by_ip_iface("wlp2s0").unwrap();
        let wireless_device = WirelessDevice::new_from_device(&device);
        let ap = wireless_device.get_access_point_by_ssid("Compact_bd3e");
        if ap.is_some() {
            let result = manager.add_and_activate_wifi_connection(wireless_device.clone(), ap.unwrap().clone(), "test1234");
            manager.deactivate_connection(device.clone());
            assert!(result.is_ok());
        }
    }

    #[test]
    fn test_deactivate_connection() {
        let manager = NetworkManager::new_system();
        let mut device = manager.get_device_by_ip_iface("wlp2s0").unwrap();
        let wireless_device = WirelessDevice::new_from_device(&device);
        let ap = wireless_device.get_access_point_by_ssid("UPC22AC955").unwrap();
        let result_activate = manager.activate_connection(wireless_device.clone(), ap);
        device.refresh_active_connection();
        let result = manager.deactivate_connection(device);
        eprintln!("{:?}", result_activate);
        eprintln!("{:?}", result);
        assert!(result.is_ok());
    }

    #[test]
    fn test_activate_connection() {
        let manager = NetworkManager::new_system();
        let device = manager.get_device_by_ip_iface("wlp2s0").unwrap();
        manager.deactivate_connection(device.clone());
        let wireless_device = WirelessDevice::new_from_device(&device);
        let ap = wireless_device.get_access_point_by_ssid("UPC22AC955").unwrap();
        let result = manager.activate_connection(wireless_device.clone(), ap);
        eprintln!("{:?}", result);
        assert!(result.is_ok());
    }

    #[test]
    fn test_find_exisiting_connection() {
        let manager = NetworkManager::new_system();
        let device = manager.get_device_by_ip_iface("wlp2s0").unwrap();
        let wireless_device = WirelessDevice::new_from_device(&device);
        let ap = wireless_device.get_access_point_by_ssid("UPC22AC955").unwrap();
        let result = manager.find_existing_connection(&ap);
        assert!(result);
    }

    #[test]
    fn test_connect_wifi_known() {
        // test if count of settings still the same
        let manager = NetworkManager::new_system();
        let device = manager.get_device_by_ip_iface("wlp2s0").unwrap();
        manager.deactivate_connection(device.clone());
        let wireless_device = WirelessDevice::new_from_device(&device);
        let ap = wireless_device.get_access_point_by_ssid("UPC22AC955").unwrap();
        let result = manager.connect_wifi(wireless_device.clone(), ap, "x");
        manager.deactivate_connection(device.clone());
        eprintln!("{:?}", result);
        assert!(result.is_ok());
    }
}