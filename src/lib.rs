extern crate dbus;
mod manager;
mod nm_networkmanager;
mod device;
mod nm_device;
mod device_type;
mod ip4_config;
mod nm_ip4_config;
mod dhcp4_config;
mod active_connection;
mod nm_active_connection;
mod access_point;
mod nm_access_point;
mod connection;
mod setting;
mod nm_settings;
mod nm_con_settings;

pub use manager::NetworkManager;
pub use device::Device;
pub use device::WirelessDevice;
pub use access_point::AccessPoint;
pub use ip4_config::Ip4Config;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
