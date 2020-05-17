struct Device {
    path: dbus::Path,
}

impl Device {
    pub fn new_from_path(p: dbus::Path) -> Self {
        Device {
            path: p
        }
    }

}