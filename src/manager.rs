use dbus::blocking::{Connection};
use crate::nm_networkmanager::OrgFreedesktopNetworkManager;
use std::time::Duration;
use crate::device::Device;
use dbus::Error;

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
}