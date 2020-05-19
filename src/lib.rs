extern crate dbus;
mod manager;
mod device;
mod nm_networkmanager;
mod nm_ip4_config;
mod nm_device;
mod device_type;
mod ip4_config;
mod dhcp4_config;
mod active_connection;

pub use manager::NetworkManager;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
