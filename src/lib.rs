extern crate dbus;
mod manager;
mod wireless_device;
mod networkmanager;
mod network_ip4_config;
mod network_device;
mod device_type;

pub use manager::NetworkManager;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
