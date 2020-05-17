use dbus::blocking::{Connection, Proxy};

pub struct NetworkManager {
    con: Connection,
    proxy: Proxy,
}

impl NetworkManager {
    pub fn new_system() -> Self {
        NetworkManager {
            con: Connection::new_system()?,
            proxy: c.with_proxy("org.freedesktop.NetworkManager", "/org/freedesktop/NetworkManager",
                                Duration::new(5, 0)),
        }
    }

    pub fn get_all_devices() -> Vec<Device> {
        let devices = proxy.get_all_devices()?;
        let mut ret: Vec<Device>;
        for d in devices {
            ret.push(Device::new_from_path(d));
        }
    }
}

mod tests {
    use super::*;

    fn test_get_all_devices() {
        let manager = NetworkManager::new_system();
        let devices = manager.get_all_devices();
        println!("{:?}", devices);
    }
}