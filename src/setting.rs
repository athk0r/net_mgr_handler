use std::collections::HashMap;
use dbus::arg::{Variant, RefArg};
use dbus::blocking::Connection;
use std::time::Duration;
use std::fmt;

pub struct Setting<'a> {
    pub path: dbus::Path<'a>,
    pub settings: HashMap<String, HashMap<String, Variant<Box<dyn RefArg>>>>
}

impl<'a> Setting<'a> {
    pub fn from_path(p: dbus::Path<'a>) -> Option<Self> {
        if p.eq(&dbus::strings::Path::new("/").unwrap()){
            return None;
        }
        let settings = self::Setting::get_settings(&p);

        Some(Setting {
            path: p,
            settings: settings
        })
    }
    fn get_settings(p: &'a dbus::Path) -> HashMap<String, HashMap<String, Variant<Box<dyn RefArg>>>> {
        use crate::nm_con_settings::OrgFreedesktopNetworkManagerSettingsConnection;
        let con = Connection::new_system().unwrap();
        let proxy = con.with_proxy("org.freedesktop.NetworkManager",
                                   p,
                                   Duration::new(5, 0));

        proxy.get_settings().unwrap()
    }
}

impl fmt::Debug for Setting<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ConnectionSetting")
            .field("Path", &self.path)
            .field("Settings", &self.settings)
            .finish()
    }
}