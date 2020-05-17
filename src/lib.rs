extern crate dbus;
mod manager;
mod device;
mod networkmanager;
mod network_IP4Config;
mod network_device;

pub use manager::NetworkManager;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
