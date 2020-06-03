use std::time::Duration;
use dbus::blocking::Connection;
use std::fmt;

#[derive(Clone)]
pub struct AccessPoint<'a> {
    pub path: dbus::Path<'a>,
    pub ssid: String,
    pub hw_address: String,
    pub wpa_flags: u32
}

impl<'a> AccessPoint<'a> {
    pub fn from_path(p: dbus::Path<'a>) -> Option<Self> {
        if p.eq(&dbus::strings::Path::new("/").unwrap()){
            return None;
        }
        let ssid = self::AccessPoint::get_ssid(&p);
        let hw_addr = self::AccessPoint::get_hw_address(&p);
        let wpa_flags = self::AccessPoint::get_wpa_flags(&p);

        Some(AccessPoint {
            path: p,
            ssid: ssid,
            hw_address: hw_addr,
            wpa_flags: wpa_flags
        })
    }

    fn get_ssid(p: &'a dbus::Path) -> String{
        use crate::nm_access_point::OrgFreedesktopNetworkManagerAccessPoint;
        let con = Connection::new_system().unwrap();
        let proxy = con.with_proxy("org.freedesktop.NetworkManager",
                                   p,
                                   Duration::new(5, 0));

        let ssid = proxy.ssid().unwrap();
        String::from_utf8(ssid).unwrap()
    }

    fn get_hw_address(p: &'a dbus::Path) -> String {
        use crate::nm_access_point::OrgFreedesktopNetworkManagerAccessPoint;
        let con = Connection::new_system().unwrap();
        let proxy = con.with_proxy("org.freedesktop.NetworkManager",
                                   p,
                                   Duration::new(5, 0));

        let hw_addr = proxy.hw_address().unwrap();
        hw_addr
    }

    fn get_wpa_flags(p: &'a dbus::Path) -> u32 {
        use crate::nm_access_point::OrgFreedesktopNetworkManagerAccessPoint;
        let con = Connection::new_system().unwrap();
        let proxy = con.with_proxy("org.freedesktop.NetworkManager",
                                   p,
                                   Duration::new(5, 0));

        let wpa_flags = proxy.wpa_flags().unwrap();
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