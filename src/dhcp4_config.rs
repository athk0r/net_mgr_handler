use std::fmt;

#[derive(Clone)]
pub struct Dhcp4Config<'a> {
    path: dbus::Path<'a>
}

impl<'a> Dhcp4Config<'a> {
    pub fn from_path(p: dbus::Path<'a>) -> Option<Self> {
        if p.eq(&dbus::strings::Path::new("/").unwrap()){
            return None;
        }

        Some(Dhcp4Config {
            path: p,
        })
    }
}

impl fmt::Debug for Dhcp4Config<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Dhcp4Config")
            .field("Path", &self.path)
            .finish()
    }
}