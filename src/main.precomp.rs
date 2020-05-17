#![feature(prelude_import)]
#![no_std]
#[prelude_import]
use ::std::prelude::v1::*;
#[macro_use]
extern crate std;
#[macro_use]
use dbus::blocking::{Connection};
use crate::networkmanager::OrgFreedesktopNetworkManager;
use std::time::Duration;
use crate::device::Device;
use dbus::Error;

const PATH_NETWORK_MANAGER: &str = "/org/freedesktop/NetworkManager";

macro_rules! call_on_proxy {
    ($ self : ident, $ path : ident, $ F : ident) =>
    {
        $ self . con .
        with_proxy("org.freedesktop.NetworkManager", $ path, Duration ::
                   new(5, 0)) . $ F()
    }
}

pub struct NetworkManager {
    con: Connection,
}

impl NetworkManager {
    pub fn new_system() -> Self {
        let connection: dbus::blocking::Connection =
            Connection::new_system().unwrap();
        NetworkManager{con: connection,}
    }

    pub fn get_all_devices(self) -> Result<Vec<Device<'static>>, Error> {
        let devices: Vec<dbus::Path> =
            self.con.with_proxy("org.freedesktop.NetworkManager",
                                PATH_NETWORK_MANAGER,
                                Duration::new(5, 0)).get_all_devices()?;
        let mut ret: Vec<Device> = Vec::new();
        for device in devices { ret.push(Device::new_from_path(device)); }

        /*pub fn get_device_by_ip_iface(self, s: String) -> Result<Device<'static>, Error> {
            let device_path: dbus::Path = call_on_proxy!(self, PATH_NETWORK_MANAGER, get_device_by_ip_iface)?;
            Ok(Device::new_from_path(device_path))
        }*/


        {
            ::std::io::_print(::core::fmt::Arguments::new_v1(&["", "\n"],
                                                             &match (&ret,) {
                                                                  (arg0,) =>
                                                                  [::core::fmt::ArgumentV1::new(arg0,
                                                                                                ::core::fmt::Debug::fmt)],
                                                              }));
        };
        Ok(ret)
    }
}
