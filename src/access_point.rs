use dbus::blocking::Connection;
use std::fmt;
use std::time::Duration;

#[derive(Clone, PartialEq)]
pub struct AccessPoint<'a> {
    pub path: dbus::Path<'a>,
    pub ssid: String,
    pub hw_address: String,
    pub wpa_flags: u32,
}

impl<'a> AccessPoint<'a> {
    pub fn from_path(p: dbus::Path<'a>) -> Option<Self> {
        if p.eq(&dbus::strings::Path::new("/").unwrap()) {
            return None;
        }
        let ssid = self::AccessPoint::get_ssid(&p);
        let hw_addr = self::AccessPoint::get_hw_address(&p);
        let wpa_flags = self::AccessPoint::get_wpa_flags(&p);

        Some(AccessPoint {
            path: p,
            ssid: ssid,
            hw_address: hw_addr,
            wpa_flags: wpa_flags,
        })
    }

    fn get_ssid(p: &'a dbus::Path) -> String {
        use crate::nm_access_point::OrgFreedesktopNetworkManagerAccessPoint;
        let con = Connection::new_system().unwrap();
        let proxy = con.with_proxy("org.freedesktop.NetworkManager", p, Duration::new(5, 0));

        let ssid = proxy.ssid().expect("can't get SSID from DBus");
        String::from_utf8(ssid).expect("can't create String from given SSID")
    }

    fn get_hw_address(p: &'a dbus::Path) -> String {
        use crate::nm_access_point::OrgFreedesktopNetworkManagerAccessPoint;
        let con = Connection::new_system().unwrap();
        let proxy = con.with_proxy("org.freedesktop.NetworkManager", p, Duration::new(5, 0));

        let hw_addr = proxy.hw_address().expect("can't get HwAddr from DBus");
        hw_addr
    }

    fn get_wpa_flags(p: &'a dbus::Path) -> u32 {
        use crate::nm_access_point::OrgFreedesktopNetworkManagerAccessPoint;
        let con = Connection::new_system().unwrap();
        let proxy = con.with_proxy("org.freedesktop.NetworkManager", p, Duration::new(5, 0));

        let wpa_flags = proxy.wpa_flags().expect("can't get WPAFlags from DBus");
        wpa_flags
    }
}

impl fmt::Debug for AccessPoint<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AccessPoint")
            .field("Path", &self.path)
            .field("SSID", &self.ssid)
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_eq() {
        let net_manager = NetworkManager::new_system();
        let device = net_manager.get_device_by_ip_iface("wlp2s0").unwrap();

        let wifi_device = WirelessDevice::new_from_device(&device);
        wifi_device.scan();

        let access_points = wifi_device.get_all_access_points();

        assert!(
            !(access_points
                .first()
                .unwrap()
                .eq(access_points.last().unwrap()))
        )
    }
}
