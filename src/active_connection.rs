use std::fmt;

pub struct ActiveConnection<'a> {
    path: dbus::Path<'a>
}

impl<'a> ActiveConnection<'a> {
    pub fn from_path(p: dbus::Path<'a>) -> Option<ActiveConnection> {
        if p.eq(&dbus::strings::Path::new("/").unwrap()){
            return None;
        }

        Some(ActiveConnection {
            path: p,
        })
    }
}

impl fmt::Debug for ActiveConnection<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ActiveConnection")
            .field("Path", &self.path)
            .finish()
    }
}