#![feature(prelude_import)]
#[prelude_import]
use std::prelude::v1::*;
#[macro_use]
extern crate std;
extern crate dbus;
mod manager {


    use dbus::blocking::{Connection};
    use crate::networkmanager::OrgFreedesktopNetworkManager;
    use std::time::Duration;
    use crate::device::Device;
    use dbus::Error;
    const PATH_NETWORK_MANAGER: &str = "/org/freedesktop/NetworkManager";
    macro_rules! call_on_proxy {
        ($ self : ident, $ path : ident) =>
        {
            $ self . con .
            with_proxy("org.freedesktop.NetworkManager", $ path, Duration ::
                       new(5, 0))
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
            {
                ::std::io::_print(::core::fmt::Arguments::new_v1(&["", "\n"],
                                                                 &match (&ret,)
                                                                      {
                                                                      (arg0,)
                                                                      =>
                                                                      [::core::fmt::ArgumentV1::new(arg0,
                                                                                                    ::core::fmt::Debug::fmt)],
                                                                  }));
            };
            Ok(ret)
        }
        pub fn get_device_by_ip_iface(self, s: &str)
         -> Result<Device<'static>, Error> {
            let device_path: dbus::Path =
                self.con.with_proxy("org.freedesktop.NetworkManager",
                                    PATH_NETWORK_MANAGER,
                                    Duration::new(5,
                                                  0)).get_device_by_ip_iface(s)?;
            Ok(Device::new_from_path(device_path))
        }
    }
}
mod device {
    use dbus::blocking::Connection;
    use crate::network_device::OrgFreedesktopNetworkManagerDeviceWireless;
    use std::time::Duration;
    use std::collections::HashMap;
    use std::fmt;
    use dbus::arg::{Variant, RefArg};
    macro_rules! proxy {
        ($ self : ident) =>
        {
            let con = Connection :: new_system() . unwrap() ; con .
            with_proxy("org.freedesktop.NetworkManager", $ self . path,
                       Duration :: new(5, 0))
        }
    }
    type dbus_options<'a> = HashMap<&'a str, Variant<Box<RefArg>>>;
    pub struct Device<'a> {
        path: dbus::Path<'a>,
    }
    impl <'a> Device<'a> {
        pub fn new_from_path(p: dbus::Path<'a>) -> Self { Device{path: p,} }
        pub fn scan(self) {
            let proxy = let con = Connection::new_system().unwrap();
            let options: dbus_options = HashMap::new();
            let old_last_scan = proxy.last_scan().unwrap();
            proxy.request_scan(options);
            let mut new_last_scan = proxy.last_scan().unwrap();
            while old_last_scan == new_last_scan {
                new_last_scan = proxy.last_scan().unwrap();
            }
        }
        pub fn get_all_access_points(self) -> Vec<dbus::Path<'static>> {
            let proxy = let con = Connection::new_system().unwrap();
            let access_points: Vec<dbus::Path<'static>> =
                proxy.get_all_access_points().unwrap();
            access_points
        }
    }
    impl fmt::Debug for Device<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("Device").field("Path", &self.path).finish()
        }
    }
    mod tests {
        use super::*;
        use crate::NetworkManager;
        fn test_scan() {
            let manager = NetworkManager::new_system();
            let device = manager.get_device_by_ip_iface("wlp2s0").unwrap();
            device.scan();
            if !(device.get_all_access_points().len() > 1) {
                {
                    ::std::rt::begin_panic("assertion failed: device.get_all_access_points().len() > 1")
                }
            };
        }
    }
}
mod networkmanager {
    use dbus as dbus;
    use dbus::arg;
    use dbus::blocking;
    pub trait OrgFreedesktopDBusProperties {
        fn get<R0: for<'b> arg::Get<'b> +
               'static>(&self, interface_name: &str, property_name: &str)
        -> Result<R0, dbus::Error>;
        fn get_all(&self, interface_name: &str)
        ->
            Result<::std::collections::HashMap<String,
                                               arg::Variant<Box<dyn arg::RefArg +
                                                                'static>>>,
                   dbus::Error>;
        fn set<I2: arg::Arg +
               arg::Append>(&self, interface_name: &str, property_name: &str,
                            value: I2)
        -> Result<(), dbus::Error>;
    }
    impl <'a, C: ::std::ops::Deref<Target = blocking::Connection>>
     OrgFreedesktopDBusProperties for blocking::Proxy<'a, C> {
        fn get<R0: for<'b> arg::Get<'b> +
               'static>(&self, interface_name: &str, property_name: &str)
         -> Result<R0, dbus::Error> {
            self.method_call("org.freedesktop.DBus.Properties", "Get",
                             (interface_name,
                              property_name)).and_then(|r:
                                                            (arg::Variant<R0>,)|
                                                           Ok((r.0).0))
        }
        fn get_all(&self, interface_name: &str)
         ->
             Result<::std::collections::HashMap<String,
                                                arg::Variant<Box<dyn arg::RefArg +
                                                                 'static>>>,
                    dbus::Error> {
            self.method_call("org.freedesktop.DBus.Properties", "GetAll",
                             (interface_name,)).and_then(|r:
                                                              (::std::collections::HashMap<String,
                                                                                           arg::Variant<Box<dyn arg::RefArg +
                                                                                                            'static>>>,)|
                                                             Ok(r.0))
        }
        fn set<I2: arg::Arg +
               arg::Append>(&self, interface_name: &str, property_name: &str,
                            value: I2) -> Result<(), dbus::Error> {
            self.method_call("org.freedesktop.DBus.Properties", "Set",
                             (interface_name, property_name,
                              arg::Variant(value)))
        }
    }
    pub struct OrgFreedesktopDBusPropertiesPropertiesChanged {
        pub interface_name: String,
        pub changed_properties: ::std::collections::HashMap<String,
                                                            arg::Variant<Box<dyn arg::RefArg +
                                                                             'static>>>,
        pub invalidated_properties: Vec<String>,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::fmt::Debug for OrgFreedesktopDBusPropertiesPropertiesChanged
     {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match *self {
                OrgFreedesktopDBusPropertiesPropertiesChanged {
                interface_name: ref __self_0_0,
                changed_properties: ref __self_0_1,
                invalidated_properties: ref __self_0_2 } => {
                    let mut debug_trait_builder =
                        f.debug_struct("OrgFreedesktopDBusPropertiesPropertiesChanged");
                    let _ =
                        debug_trait_builder.field("interface_name",
                                                  &&(*__self_0_0));
                    let _ =
                        debug_trait_builder.field("changed_properties",
                                                  &&(*__self_0_1));
                    let _ =
                        debug_trait_builder.field("invalidated_properties",
                                                  &&(*__self_0_2));
                    debug_trait_builder.finish()
                }
            }
        }
    }
    impl arg::AppendAll for OrgFreedesktopDBusPropertiesPropertiesChanged {
        fn append(&self, i: &mut arg::IterAppend) {
            arg::RefArg::append(&self.interface_name, i);
            arg::RefArg::append(&self.changed_properties, i);
            arg::RefArg::append(&self.invalidated_properties, i);
        }
    }
    impl arg::ReadAll for OrgFreedesktopDBusPropertiesPropertiesChanged {
        fn read(i: &mut arg::Iter) -> Result<Self, arg::TypeMismatchError> {
            Ok(OrgFreedesktopDBusPropertiesPropertiesChanged{interface_name:
                                                                 i.read()?,
                                                             changed_properties:
                                                                 i.read()?,
                                                             invalidated_properties:
                                                                 i.read()?,})
        }
    }
    impl dbus::message::SignalArgs for
     OrgFreedesktopDBusPropertiesPropertiesChanged {
        const NAME: &'static str = "PropertiesChanged";
        const INTERFACE: &'static str = "org.freedesktop.DBus.Properties";
    }
    pub trait OrgFreedesktopDBusIntrospectable {
        fn introspect(&self)
        -> Result<String, dbus::Error>;
    }
    impl <'a, C: ::std::ops::Deref<Target = blocking::Connection>>
     OrgFreedesktopDBusIntrospectable for blocking::Proxy<'a, C> {
        fn introspect(&self) -> Result<String, dbus::Error> {
            self.method_call("org.freedesktop.DBus.Introspectable",
                             "Introspect",
                             ()).and_then(|r: (String,)| Ok(r.0))
        }
    }
    pub trait OrgFreedesktopDBusPeer {
        fn ping(&self)
        -> Result<(), dbus::Error>;
        fn get_machine_id(&self)
        -> Result<String, dbus::Error>;
    }
    impl <'a, C: ::std::ops::Deref<Target = blocking::Connection>>
     OrgFreedesktopDBusPeer for blocking::Proxy<'a, C> {
        fn ping(&self) -> Result<(), dbus::Error> {
            self.method_call("org.freedesktop.DBus.Peer", "Ping", ())
        }
        fn get_machine_id(&self) -> Result<String, dbus::Error> {
            self.method_call("org.freedesktop.DBus.Peer", "GetMachineId",
                             ()).and_then(|r: (String,)| Ok(r.0))
        }
    }
    pub trait OrgFreedesktopNetworkManager {
        fn reload(&self, flags: u32)
        -> Result<(), dbus::Error>;
        fn get_devices(&self)
        -> Result<Vec<dbus::Path<'static>>, dbus::Error>;
        fn get_all_devices(&self)
        -> Result<Vec<dbus::Path<'static>>, dbus::Error>;
        fn get_device_by_ip_iface(&self, iface: &str)
        -> Result<dbus::Path<'static>, dbus::Error>;
        fn activate_connection(&self, connection: dbus::Path,
                               device: dbus::Path,
                               specific_object: dbus::Path)
        -> Result<dbus::Path<'static>, dbus::Error>;
        fn add_and_activate_connection(&self,
                                       connection:
                                           ::std::collections::HashMap<&str,
                                                                       ::std::collections::HashMap<&str,
                                                                                                   arg::Variant<Box<dyn arg::RefArg>>>>,
                                       device: dbus::Path,
                                       specific_object: dbus::Path)
        -> Result<(dbus::Path<'static>, dbus::Path<'static>), dbus::Error>;
        fn add_and_activate_connection2(&self,
                                        connection:
                                            ::std::collections::HashMap<&str,
                                                                        ::std::collections::HashMap<&str,
                                                                                                    arg::Variant<Box<dyn arg::RefArg>>>>,
                                        device: dbus::Path,
                                        specific_object: dbus::Path,
                                        options:
                                            ::std::collections::HashMap<&str,
                                                                        arg::Variant<Box<dyn arg::RefArg>>>)
        ->
            Result<(dbus::Path<'static>, dbus::Path<'static>,
                    ::std::collections::HashMap<String,
                                                arg::Variant<Box<dyn arg::RefArg +
                                                                 'static>>>),
                   dbus::Error>;
        fn deactivate_connection(&self, active_connection: dbus::Path)
        -> Result<(), dbus::Error>;
        fn sleep(&self, sleep: bool)
        -> Result<(), dbus::Error>;
        fn enable(&self, enable: bool)
        -> Result<(), dbus::Error>;
        fn get_permissions(&self)
        -> Result<::std::collections::HashMap<String, String>, dbus::Error>;
        fn set_logging(&self, level: &str, domains: &str)
        -> Result<(), dbus::Error>;
        fn get_logging(&self)
        -> Result<(String, String), dbus::Error>;
        fn check_connectivity(&self)
        -> Result<u32, dbus::Error>;
        fn state(&self)
        -> Result<u32, dbus::Error>;
        fn checkpoint_create(&self, devices: Vec<dbus::Path>,
                             rollback_timeout: u32, flags: u32)
        -> Result<dbus::Path<'static>, dbus::Error>;
        fn checkpoint_destroy(&self, checkpoint: dbus::Path)
        -> Result<(), dbus::Error>;
        fn checkpoint_rollback(&self, checkpoint: dbus::Path)
        -> Result<::std::collections::HashMap<String, u32>, dbus::Error>;
        fn checkpoint_adjust_rollback_timeout(&self, checkpoint: dbus::Path,
                                              add_timeout: u32)
        -> Result<(), dbus::Error>;
        fn devices(&self)
        -> Result<Vec<dbus::Path<'static>>, dbus::Error>;
        fn all_devices(&self)
        -> Result<Vec<dbus::Path<'static>>, dbus::Error>;
        fn checkpoints(&self)
        -> Result<Vec<dbus::Path<'static>>, dbus::Error>;
        fn networking_enabled(&self)
        -> Result<bool, dbus::Error>;
        fn wireless_enabled(&self)
        -> Result<bool, dbus::Error>;
        fn set_wireless_enabled(&self, value: bool)
        -> Result<(), dbus::Error>;
        fn wireless_hardware_enabled(&self)
        -> Result<bool, dbus::Error>;
        fn wwan_enabled(&self)
        -> Result<bool, dbus::Error>;
        fn set_wwan_enabled(&self, value: bool)
        -> Result<(), dbus::Error>;
        fn wwan_hardware_enabled(&self)
        -> Result<bool, dbus::Error>;
        fn wimax_enabled(&self)
        -> Result<bool, dbus::Error>;
        fn set_wimax_enabled(&self, value: bool)
        -> Result<(), dbus::Error>;
        fn wimax_hardware_enabled(&self)
        -> Result<bool, dbus::Error>;
        fn active_connections(&self)
        -> Result<Vec<dbus::Path<'static>>, dbus::Error>;
        fn primary_connection(&self)
        -> Result<dbus::Path<'static>, dbus::Error>;
        fn primary_connection_type(&self)
        -> Result<String, dbus::Error>;
        fn metered(&self)
        -> Result<u32, dbus::Error>;
        fn activating_connection(&self)
        -> Result<dbus::Path<'static>, dbus::Error>;
        fn startup(&self)
        -> Result<bool, dbus::Error>;
        fn version(&self)
        -> Result<String, dbus::Error>;
        fn capabilities(&self)
        -> Result<u32, dbus::Error>;
        fn state_(&self)
        -> Result<u32, dbus::Error>;
        fn connectivity(&self)
        -> Result<u32, dbus::Error>;
        fn connectivity_check_available(&self)
        -> Result<bool, dbus::Error>;
        fn connectivity_check_enabled(&self)
        -> Result<bool, dbus::Error>;
        fn set_connectivity_check_enabled(&self, value: bool)
        -> Result<(), dbus::Error>;
        fn connectivity_check_uri(&self)
        -> Result<String, dbus::Error>;
        fn global_dns_configuration(&self)
        ->
            Result<::std::collections::HashMap<String,
                                               arg::Variant<Box<dyn arg::RefArg +
                                                                'static>>>,
                   dbus::Error>;
        fn set_global_dns_configuration(&self,
                                        value:
                                            ::std::collections::HashMap<String,
                                                                        arg::Variant<Box<dyn arg::RefArg +
                                                                                         'static>>>)
        -> Result<(), dbus::Error>;
    }
    impl <'a, C: ::std::ops::Deref<Target = blocking::Connection>>
     OrgFreedesktopNetworkManager for blocking::Proxy<'a, C> {
        fn reload(&self, flags: u32) -> Result<(), dbus::Error> {
            self.method_call("org.freedesktop.NetworkManager", "Reload",
                             (flags,))
        }
        fn get_devices(&self)
         -> Result<Vec<dbus::Path<'static>>, dbus::Error> {
            self.method_call("org.freedesktop.NetworkManager", "GetDevices",
                             ()).and_then(|r: (Vec<dbus::Path<'static>>,)|
                                              Ok(r.0))
        }
        fn get_all_devices(&self)
         -> Result<Vec<dbus::Path<'static>>, dbus::Error> {
            self.method_call("org.freedesktop.NetworkManager",
                             "GetAllDevices",
                             ()).and_then(|r: (Vec<dbus::Path<'static>>,)|
                                              Ok(r.0))
        }
        fn get_device_by_ip_iface(&self, iface: &str)
         -> Result<dbus::Path<'static>, dbus::Error> {
            self.method_call("org.freedesktop.NetworkManager",
                             "GetDeviceByIpIface",
                             (iface,)).and_then(|r: (dbus::Path<'static>,)|
                                                    Ok(r.0))
        }
        fn activate_connection(&self, connection: dbus::Path,
                               device: dbus::Path,
                               specific_object: dbus::Path)
         -> Result<dbus::Path<'static>, dbus::Error> {
            self.method_call("org.freedesktop.NetworkManager",
                             "ActivateConnection",
                             (connection, device,
                              specific_object)).and_then(|r:
                                                              (dbus::Path<'static>,)|
                                                             Ok(r.0))
        }
        fn add_and_activate_connection(&self,
                                       connection:
                                           ::std::collections::HashMap<&str,
                                                                       ::std::collections::HashMap<&str,
                                                                                                   arg::Variant<Box<dyn arg::RefArg>>>>,
                                       device: dbus::Path,
                                       specific_object: dbus::Path)
         -> Result<(dbus::Path<'static>, dbus::Path<'static>), dbus::Error> {
            self.method_call("org.freedesktop.NetworkManager",
                             "AddAndActivateConnection",
                             (connection, device, specific_object))
        }
        fn add_and_activate_connection2(&self,
                                        connection:
                                            ::std::collections::HashMap<&str,
                                                                        ::std::collections::HashMap<&str,
                                                                                                    arg::Variant<Box<dyn arg::RefArg>>>>,
                                        device: dbus::Path,
                                        specific_object: dbus::Path,
                                        options:
                                            ::std::collections::HashMap<&str,
                                                                        arg::Variant<Box<dyn arg::RefArg>>>)
         ->
             Result<(dbus::Path<'static>, dbus::Path<'static>,
                     ::std::collections::HashMap<String,
                                                 arg::Variant<Box<dyn arg::RefArg +
                                                                  'static>>>),
                    dbus::Error> {
            self.method_call("org.freedesktop.NetworkManager",
                             "AddAndActivateConnection2",
                             (connection, device, specific_object, options))
        }
        fn deactivate_connection(&self, active_connection: dbus::Path)
         -> Result<(), dbus::Error> {
            self.method_call("org.freedesktop.NetworkManager",
                             "DeactivateConnection", (active_connection,))
        }
        fn sleep(&self, sleep: bool) -> Result<(), dbus::Error> {
            self.method_call("org.freedesktop.NetworkManager", "Sleep",
                             (sleep,))
        }
        fn enable(&self, enable: bool) -> Result<(), dbus::Error> {
            self.method_call("org.freedesktop.NetworkManager", "Enable",
                             (enable,))
        }
        fn get_permissions(&self)
         -> Result<::std::collections::HashMap<String, String>, dbus::Error> {
            self.method_call("org.freedesktop.NetworkManager",
                             "GetPermissions",
                             ()).and_then(|r:
                                               (::std::collections::HashMap<String,
                                                                            String>,)|
                                              Ok(r.0))
        }
        fn set_logging(&self, level: &str, domains: &str)
         -> Result<(), dbus::Error> {
            self.method_call("org.freedesktop.NetworkManager", "SetLogging",
                             (level, domains))
        }
        fn get_logging(&self) -> Result<(String, String), dbus::Error> {
            self.method_call("org.freedesktop.NetworkManager", "GetLogging",
                             ())
        }
        fn check_connectivity(&self) -> Result<u32, dbus::Error> {
            self.method_call("org.freedesktop.NetworkManager",
                             "CheckConnectivity",
                             ()).and_then(|r: (u32,)| Ok(r.0))
        }
        fn state(&self) -> Result<u32, dbus::Error> {
            self.method_call("org.freedesktop.NetworkManager", "state",
                             ()).and_then(|r: (u32,)| Ok(r.0))
        }
        fn checkpoint_create(&self, devices: Vec<dbus::Path>,
                             rollback_timeout: u32, flags: u32)
         -> Result<dbus::Path<'static>, dbus::Error> {
            self.method_call("org.freedesktop.NetworkManager",
                             "CheckpointCreate",
                             (devices, rollback_timeout,
                              flags)).and_then(|r: (dbus::Path<'static>,)|
                                                   Ok(r.0))
        }
        fn checkpoint_destroy(&self, checkpoint: dbus::Path)
         -> Result<(), dbus::Error> {
            self.method_call("org.freedesktop.NetworkManager",
                             "CheckpointDestroy", (checkpoint,))
        }
        fn checkpoint_rollback(&self, checkpoint: dbus::Path)
         -> Result<::std::collections::HashMap<String, u32>, dbus::Error> {
            self.method_call("org.freedesktop.NetworkManager",
                             "CheckpointRollback",
                             (checkpoint,)).and_then(|r:
                                                          (::std::collections::HashMap<String,
                                                                                       u32>,)|
                                                         Ok(r.0))
        }
        fn checkpoint_adjust_rollback_timeout(&self, checkpoint: dbus::Path,
                                              add_timeout: u32)
         -> Result<(), dbus::Error> {
            self.method_call("org.freedesktop.NetworkManager",
                             "CheckpointAdjustRollbackTimeout",
                             (checkpoint, add_timeout))
        }
        fn devices(&self) -> Result<Vec<dbus::Path<'static>>, dbus::Error> {
            <Self as
                blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self,
                                                                          "org.freedesktop.NetworkManager",
                                                                          "Devices")
        }
        fn all_devices(&self)
         -> Result<Vec<dbus::Path<'static>>, dbus::Error> {
            <Self as
                blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self,
                                                                          "org.freedesktop.NetworkManager",
                                                                          "AllDevices")
        }
        fn checkpoints(&self)
         -> Result<Vec<dbus::Path<'static>>, dbus::Error> {
            <Self as
                blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self,
                                                                          "org.freedesktop.NetworkManager",
                                                                          "Checkpoints")
        }
        fn networking_enabled(&self) -> Result<bool, dbus::Error> {
            <Self as
                blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self,
                                                                          "org.freedesktop.NetworkManager",
                                                                          "NetworkingEnabled")
        }
        fn wireless_enabled(&self) -> Result<bool, dbus::Error> {
            <Self as
                blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self,
                                                                          "org.freedesktop.NetworkManager",
                                                                          "WirelessEnabled")
        }
        fn wireless_hardware_enabled(&self) -> Result<bool, dbus::Error> {
            <Self as
                blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self,
                                                                          "org.freedesktop.NetworkManager",
                                                                          "WirelessHardwareEnabled")
        }
        fn wwan_enabled(&self) -> Result<bool, dbus::Error> {
            <Self as
                blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self,
                                                                          "org.freedesktop.NetworkManager",
                                                                          "WwanEnabled")
        }
        fn wwan_hardware_enabled(&self) -> Result<bool, dbus::Error> {
            <Self as
                blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self,
                                                                          "org.freedesktop.NetworkManager",
                                                                          "WwanHardwareEnabled")
        }
        fn wimax_enabled(&self) -> Result<bool, dbus::Error> {
            <Self as
                blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self,
                                                                          "org.freedesktop.NetworkManager",
                                                                          "WimaxEnabled")
        }
        fn wimax_hardware_enabled(&self) -> Result<bool, dbus::Error> {
            <Self as
                blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self,
                                                                          "org.freedesktop.NetworkManager",
                                                                          "WimaxHardwareEnabled")
        }
        fn active_connections(&self)
         -> Result<Vec<dbus::Path<'static>>, dbus::Error> {
            <Self as
                blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self,
                                                                          "org.freedesktop.NetworkManager",
                                                                          "ActiveConnections")
        }
        fn primary_connection(&self)
         -> Result<dbus::Path<'static>, dbus::Error> {
            <Self as
                blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self,
                                                                          "org.freedesktop.NetworkManager",
                                                                          "PrimaryConnection")
        }
        fn primary_connection_type(&self) -> Result<String, dbus::Error> {
            <Self as
                blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self,
                                                                          "org.freedesktop.NetworkManager",
                                                                          "PrimaryConnectionType")
        }
        fn metered(&self) -> Result<u32, dbus::Error> {
            <Self as
                blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self,
                                                                          "org.freedesktop.NetworkManager",
                                                                          "Metered")
        }
        fn activating_connection(&self)
         -> Result<dbus::Path<'static>, dbus::Error> {
            <Self as
                blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self,
                                                                          "org.freedesktop.NetworkManager",
                                                                          "ActivatingConnection")
        }
        fn startup(&self) -> Result<bool, dbus::Error> {
            <Self as
                blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self,
                                                                          "org.freedesktop.NetworkManager",
                                                                          "Startup")
        }
        fn version(&self) -> Result<String, dbus::Error> {
            <Self as
                blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self,
                                                                          "org.freedesktop.NetworkManager",
                                                                          "Version")
        }
        fn capabilities(&self) -> Result<u32, dbus::Error> {
            <Self as
                blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self,
                                                                          "org.freedesktop.NetworkManager",
                                                                          "Capabilities")
        }
        fn state_(&self) -> Result<u32, dbus::Error> {
            <Self as
                blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self,
                                                                          "org.freedesktop.NetworkManager",
                                                                          "State")
        }
        fn connectivity(&self) -> Result<u32, dbus::Error> {
            <Self as
                blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self,
                                                                          "org.freedesktop.NetworkManager",
                                                                          "Connectivity")
        }
        fn connectivity_check_available(&self) -> Result<bool, dbus::Error> {
            <Self as
                blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self,
                                                                          "org.freedesktop.NetworkManager",
                                                                          "ConnectivityCheckAvailable")
        }
        fn connectivity_check_enabled(&self) -> Result<bool, dbus::Error> {
            <Self as
                blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self,
                                                                          "org.freedesktop.NetworkManager",
                                                                          "ConnectivityCheckEnabled")
        }
        fn connectivity_check_uri(&self) -> Result<String, dbus::Error> {
            <Self as
                blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self,
                                                                          "org.freedesktop.NetworkManager",
                                                                          "ConnectivityCheckUri")
        }
        fn global_dns_configuration(&self)
         ->
             Result<::std::collections::HashMap<String,
                                                arg::Variant<Box<dyn arg::RefArg +
                                                                 'static>>>,
                    dbus::Error> {
            <Self as
                blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self,
                                                                          "org.freedesktop.NetworkManager",
                                                                          "GlobalDnsConfiguration")
        }
        fn set_wireless_enabled(&self, value: bool)
         -> Result<(), dbus::Error> {
            <Self as
                blocking::stdintf::org_freedesktop_dbus::Properties>::set(&self,
                                                                          "org.freedesktop.NetworkManager",
                                                                          "WirelessEnabled",
                                                                          value)
        }
        fn set_wwan_enabled(&self, value: bool) -> Result<(), dbus::Error> {
            <Self as
                blocking::stdintf::org_freedesktop_dbus::Properties>::set(&self,
                                                                          "org.freedesktop.NetworkManager",
                                                                          "WwanEnabled",
                                                                          value)
        }
        fn set_wimax_enabled(&self, value: bool) -> Result<(), dbus::Error> {
            <Self as
                blocking::stdintf::org_freedesktop_dbus::Properties>::set(&self,
                                                                          "org.freedesktop.NetworkManager",
                                                                          "WimaxEnabled",
                                                                          value)
        }
        fn set_connectivity_check_enabled(&self, value: bool)
         -> Result<(), dbus::Error> {
            <Self as
                blocking::stdintf::org_freedesktop_dbus::Properties>::set(&self,
                                                                          "org.freedesktop.NetworkManager",
                                                                          "ConnectivityCheckEnabled",
                                                                          value)
        }
        fn set_global_dns_configuration(&self,
                                        value:
                                            ::std::collections::HashMap<String,
                                                                        arg::Variant<Box<dyn arg::RefArg +
                                                                                         'static>>>)
         -> Result<(), dbus::Error> {
            <Self as
                blocking::stdintf::org_freedesktop_dbus::Properties>::set(&self,
                                                                          "org.freedesktop.NetworkManager",
                                                                          "GlobalDnsConfiguration",
                                                                          value)
        }
    }
    pub struct OrgFreedesktopNetworkManagerPropertiesChanged {
        pub properties: ::std::collections::HashMap<String,
                                                    arg::Variant<Box<dyn arg::RefArg +
                                                                     'static>>>,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::fmt::Debug for OrgFreedesktopNetworkManagerPropertiesChanged
     {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match *self {
                OrgFreedesktopNetworkManagerPropertiesChanged {
                properties: ref __self_0_0 } => {
                    let mut debug_trait_builder =
                        f.debug_struct("OrgFreedesktopNetworkManagerPropertiesChanged");
                    let _ =
                        debug_trait_builder.field("properties",
                                                  &&(*__self_0_0));
                    debug_trait_builder.finish()
                }
            }
        }
    }
    impl arg::AppendAll for OrgFreedesktopNetworkManagerPropertiesChanged {
        fn append(&self, i: &mut arg::IterAppend) {
            arg::RefArg::append(&self.properties, i);
        }
    }
    impl arg::ReadAll for OrgFreedesktopNetworkManagerPropertiesChanged {
        fn read(i: &mut arg::Iter) -> Result<Self, arg::TypeMismatchError> {
            Ok(OrgFreedesktopNetworkManagerPropertiesChanged{properties:
                                                                 i.read()?,})
        }
    }
    impl dbus::message::SignalArgs for
     OrgFreedesktopNetworkManagerPropertiesChanged {
        const NAME: &'static str = "PropertiesChanged";
        const INTERFACE: &'static str = "org.freedesktop.NetworkManager";
    }
    pub struct OrgFreedesktopNetworkManagerCheckPermissions {
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::fmt::Debug for OrgFreedesktopNetworkManagerCheckPermissions {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match *self {
                OrgFreedesktopNetworkManagerCheckPermissions {  } => {
                    let mut debug_trait_builder =
                        f.debug_struct("OrgFreedesktopNetworkManagerCheckPermissions");
                    debug_trait_builder.finish()
                }
            }
        }
    }
    impl arg::AppendAll for OrgFreedesktopNetworkManagerCheckPermissions {
        fn append(&self, _: &mut arg::IterAppend) { }
    }
    impl arg::ReadAll for OrgFreedesktopNetworkManagerCheckPermissions {
        fn read(_: &mut arg::Iter) -> Result<Self, arg::TypeMismatchError> {
            Ok(OrgFreedesktopNetworkManagerCheckPermissions{})
        }
    }
    impl dbus::message::SignalArgs for
     OrgFreedesktopNetworkManagerCheckPermissions {
        const NAME: &'static str = "CheckPermissions";
        const INTERFACE: &'static str = "org.freedesktop.NetworkManager";
    }
    pub struct OrgFreedesktopNetworkManagerStateChanged {
        pub state: u32,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::fmt::Debug for OrgFreedesktopNetworkManagerStateChanged {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match *self {
                OrgFreedesktopNetworkManagerStateChanged {
                state: ref __self_0_0 } => {
                    let mut debug_trait_builder =
                        f.debug_struct("OrgFreedesktopNetworkManagerStateChanged");
                    let _ =
                        debug_trait_builder.field("state", &&(*__self_0_0));
                    debug_trait_builder.finish()
                }
            }
        }
    }
    impl arg::AppendAll for OrgFreedesktopNetworkManagerStateChanged {
        fn append(&self, i: &mut arg::IterAppend) {
            arg::RefArg::append(&self.state, i);
        }
    }
    impl arg::ReadAll for OrgFreedesktopNetworkManagerStateChanged {
        fn read(i: &mut arg::Iter) -> Result<Self, arg::TypeMismatchError> {
            Ok(OrgFreedesktopNetworkManagerStateChanged{state: i.read()?,})
        }
    }
    impl dbus::message::SignalArgs for
     OrgFreedesktopNetworkManagerStateChanged {
        const NAME: &'static str = "StateChanged";
        const INTERFACE: &'static str = "org.freedesktop.NetworkManager";
    }
    pub struct OrgFreedesktopNetworkManagerDeviceAdded {
        pub device_path: dbus::Path<'static>,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::fmt::Debug for OrgFreedesktopNetworkManagerDeviceAdded {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match *self {
                OrgFreedesktopNetworkManagerDeviceAdded {
                device_path: ref __self_0_0 } => {
                    let mut debug_trait_builder =
                        f.debug_struct("OrgFreedesktopNetworkManagerDeviceAdded");
                    let _ =
                        debug_trait_builder.field("device_path",
                                                  &&(*__self_0_0));
                    debug_trait_builder.finish()
                }
            }
        }
    }
    impl arg::AppendAll for OrgFreedesktopNetworkManagerDeviceAdded {
        fn append(&self, i: &mut arg::IterAppend) {
            arg::RefArg::append(&self.device_path, i);
        }
    }
    impl arg::ReadAll for OrgFreedesktopNetworkManagerDeviceAdded {
        fn read(i: &mut arg::Iter) -> Result<Self, arg::TypeMismatchError> {
            Ok(OrgFreedesktopNetworkManagerDeviceAdded{device_path:
                                                           i.read()?,})
        }
    }
    impl dbus::message::SignalArgs for OrgFreedesktopNetworkManagerDeviceAdded
     {
        const NAME: &'static str = "DeviceAdded";
        const INTERFACE: &'static str = "org.freedesktop.NetworkManager";
    }
    pub struct OrgFreedesktopNetworkManagerDeviceRemoved {
        pub device_path: dbus::Path<'static>,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::fmt::Debug for OrgFreedesktopNetworkManagerDeviceRemoved {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match *self {
                OrgFreedesktopNetworkManagerDeviceRemoved {
                device_path: ref __self_0_0 } => {
                    let mut debug_trait_builder =
                        f.debug_struct("OrgFreedesktopNetworkManagerDeviceRemoved");
                    let _ =
                        debug_trait_builder.field("device_path",
                                                  &&(*__self_0_0));
                    debug_trait_builder.finish()
                }
            }
        }
    }
    impl arg::AppendAll for OrgFreedesktopNetworkManagerDeviceRemoved {
        fn append(&self, i: &mut arg::IterAppend) {
            arg::RefArg::append(&self.device_path, i);
        }
    }
    impl arg::ReadAll for OrgFreedesktopNetworkManagerDeviceRemoved {
        fn read(i: &mut arg::Iter) -> Result<Self, arg::TypeMismatchError> {
            Ok(OrgFreedesktopNetworkManagerDeviceRemoved{device_path:
                                                             i.read()?,})
        }
    }
    impl dbus::message::SignalArgs for
     OrgFreedesktopNetworkManagerDeviceRemoved {
        const NAME: &'static str = "DeviceRemoved";
        const INTERFACE: &'static str = "org.freedesktop.NetworkManager";
    }
}
mod network_IP4Config {
    use dbus as dbus;
    use dbus::arg;
    use dbus::blocking;
    pub trait OrgFreedesktopDBusProperties {
        fn get<R0: for<'b> arg::Get<'b> +
               'static>(&self, interface_name: &str, property_name: &str)
        -> Result<R0, dbus::Error>;
        fn get_all(&self, interface_name: &str)
        ->
            Result<::std::collections::HashMap<String,
                                               arg::Variant<Box<dyn arg::RefArg +
                                                                'static>>>,
                   dbus::Error>;
        fn set<I2: arg::Arg +
               arg::Append>(&self, interface_name: &str, property_name: &str,
                            value: I2)
        -> Result<(), dbus::Error>;
    }
    impl <'a, C: ::std::ops::Deref<Target = blocking::Connection>>
     OrgFreedesktopDBusProperties for blocking::Proxy<'a, C> {
        fn get<R0: for<'b> arg::Get<'b> +
               'static>(&self, interface_name: &str, property_name: &str)
         -> Result<R0, dbus::Error> {
            self.method_call("org.freedesktop.DBus.Properties", "Get",
                             (interface_name,
                              property_name)).and_then(|r:
                                                            (arg::Variant<R0>,)|
                                                           Ok((r.0).0))
        }
        fn get_all(&self, interface_name: &str)
         ->
             Result<::std::collections::HashMap<String,
                                                arg::Variant<Box<dyn arg::RefArg +
                                                                 'static>>>,
                    dbus::Error> {
            self.method_call("org.freedesktop.DBus.Properties", "GetAll",
                             (interface_name,)).and_then(|r:
                                                              (::std::collections::HashMap<String,
                                                                                           arg::Variant<Box<dyn arg::RefArg +
                                                                                                            'static>>>,)|
                                                             Ok(r.0))
        }
        fn set<I2: arg::Arg +
               arg::Append>(&self, interface_name: &str, property_name: &str,
                            value: I2) -> Result<(), dbus::Error> {
            self.method_call("org.freedesktop.DBus.Properties", "Set",
                             (interface_name, property_name,
                              arg::Variant(value)))
        }
    }
    pub struct OrgFreedesktopDBusPropertiesPropertiesChanged {
        pub interface_name: String,
        pub changed_properties: ::std::collections::HashMap<String,
                                                            arg::Variant<Box<dyn arg::RefArg +
                                                                             'static>>>,
        pub invalidated_properties: Vec<String>,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::fmt::Debug for OrgFreedesktopDBusPropertiesPropertiesChanged
     {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match *self {
                OrgFreedesktopDBusPropertiesPropertiesChanged {
                interface_name: ref __self_0_0,
                changed_properties: ref __self_0_1,
                invalidated_properties: ref __self_0_2 } => {
                    let mut debug_trait_builder =
                        f.debug_struct("OrgFreedesktopDBusPropertiesPropertiesChanged");
                    let _ =
                        debug_trait_builder.field("interface_name",
                                                  &&(*__self_0_0));
                    let _ =
                        debug_trait_builder.field("changed_properties",
                                                  &&(*__self_0_1));
                    let _ =
                        debug_trait_builder.field("invalidated_properties",
                                                  &&(*__self_0_2));
                    debug_trait_builder.finish()
                }
            }
        }
    }
    impl arg::AppendAll for OrgFreedesktopDBusPropertiesPropertiesChanged {
        fn append(&self, i: &mut arg::IterAppend) {
            arg::RefArg::append(&self.interface_name, i);
            arg::RefArg::append(&self.changed_properties, i);
            arg::RefArg::append(&self.invalidated_properties, i);
        }
    }
    impl arg::ReadAll for OrgFreedesktopDBusPropertiesPropertiesChanged {
        fn read(i: &mut arg::Iter) -> Result<Self, arg::TypeMismatchError> {
            Ok(OrgFreedesktopDBusPropertiesPropertiesChanged{interface_name:
                                                                 i.read()?,
                                                             changed_properties:
                                                                 i.read()?,
                                                             invalidated_properties:
                                                                 i.read()?,})
        }
    }
    impl dbus::message::SignalArgs for
     OrgFreedesktopDBusPropertiesPropertiesChanged {
        const NAME: &'static str = "PropertiesChanged";
        const INTERFACE: &'static str = "org.freedesktop.DBus.Properties";
    }
    pub trait OrgFreedesktopDBusIntrospectable {
        fn introspect(&self)
        -> Result<String, dbus::Error>;
    }
    impl <'a, C: ::std::ops::Deref<Target = blocking::Connection>>
     OrgFreedesktopDBusIntrospectable for blocking::Proxy<'a, C> {
        fn introspect(&self) -> Result<String, dbus::Error> {
            self.method_call("org.freedesktop.DBus.Introspectable",
                             "Introspect",
                             ()).and_then(|r: (String,)| Ok(r.0))
        }
    }
    pub trait OrgFreedesktopDBusPeer {
        fn ping(&self)
        -> Result<(), dbus::Error>;
        fn get_machine_id(&self)
        -> Result<String, dbus::Error>;
    }
    impl <'a, C: ::std::ops::Deref<Target = blocking::Connection>>
     OrgFreedesktopDBusPeer for blocking::Proxy<'a, C> {
        fn ping(&self) -> Result<(), dbus::Error> {
            self.method_call("org.freedesktop.DBus.Peer", "Ping", ())
        }
        fn get_machine_id(&self) -> Result<String, dbus::Error> {
            self.method_call("org.freedesktop.DBus.Peer", "GetMachineId",
                             ()).and_then(|r: (String,)| Ok(r.0))
        }
    }
    pub trait OrgFreedesktopNetworkManagerIP4Config {
        fn addresses(&self)
        -> Result<Vec<Vec<u32>>, dbus::Error>;
        fn address_data(&self)
        ->
            Result<Vec<::std::collections::HashMap<String,
                                                   arg::Variant<Box<dyn arg::RefArg +
                                                                    'static>>>>,
                   dbus::Error>;
        fn gateway(&self)
        -> Result<String, dbus::Error>;
        fn routes(&self)
        -> Result<Vec<Vec<u32>>, dbus::Error>;
        fn route_data(&self)
        ->
            Result<Vec<::std::collections::HashMap<String,
                                                   arg::Variant<Box<dyn arg::RefArg +
                                                                    'static>>>>,
                   dbus::Error>;
        fn nameserver_data(&self)
        ->
            Result<Vec<::std::collections::HashMap<String,
                                                   arg::Variant<Box<dyn arg::RefArg +
                                                                    'static>>>>,
                   dbus::Error>;
        fn nameservers(&self)
        -> Result<Vec<u32>, dbus::Error>;
        fn domains(&self)
        -> Result<Vec<String>, dbus::Error>;
        fn searches(&self)
        -> Result<Vec<String>, dbus::Error>;
        fn dns_options(&self)
        -> Result<Vec<String>, dbus::Error>;
        fn dns_priority(&self)
        -> Result<i32, dbus::Error>;
        fn wins_server_data(&self)
        -> Result<Vec<String>, dbus::Error>;
        fn wins_servers(&self)
        -> Result<Vec<u32>, dbus::Error>;
    }
    impl <'a, C: ::std::ops::Deref<Target = blocking::Connection>>
     OrgFreedesktopNetworkManagerIP4Config for blocking::Proxy<'a, C> {
        fn addresses(&self) -> Result<Vec<Vec<u32>>, dbus::Error> {
            <Self as
                blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self,
                                                                          "org.freedesktop.NetworkManager.IP4Config",
                                                                          "Addresses")
        }
        fn address_data(&self)
         ->
             Result<Vec<::std::collections::HashMap<String,
                                                    arg::Variant<Box<dyn arg::RefArg +
                                                                     'static>>>>,
                    dbus::Error> {
            <Self as
                blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self,
                                                                          "org.freedesktop.NetworkManager.IP4Config",
                                                                          "AddressData")
        }
        fn gateway(&self) -> Result<String, dbus::Error> {
            <Self as
                blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self,
                                                                          "org.freedesktop.NetworkManager.IP4Config",
                                                                          "Gateway")
        }
        fn routes(&self) -> Result<Vec<Vec<u32>>, dbus::Error> {
            <Self as
                blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self,
                                                                          "org.freedesktop.NetworkManager.IP4Config",
                                                                          "Routes")
        }
        fn route_data(&self)
         ->
             Result<Vec<::std::collections::HashMap<String,
                                                    arg::Variant<Box<dyn arg::RefArg +
                                                                     'static>>>>,
                    dbus::Error> {
            <Self as
                blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self,
                                                                          "org.freedesktop.NetworkManager.IP4Config",
                                                                          "RouteData")
        }
        fn nameserver_data(&self)
         ->
             Result<Vec<::std::collections::HashMap<String,
                                                    arg::Variant<Box<dyn arg::RefArg +
                                                                     'static>>>>,
                    dbus::Error> {
            <Self as
                blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self,
                                                                          "org.freedesktop.NetworkManager.IP4Config",
                                                                          "NameserverData")
        }
        fn nameservers(&self) -> Result<Vec<u32>, dbus::Error> {
            <Self as
                blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self,
                                                                          "org.freedesktop.NetworkManager.IP4Config",
                                                                          "Nameservers")
        }
        fn domains(&self) -> Result<Vec<String>, dbus::Error> {
            <Self as
                blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self,
                                                                          "org.freedesktop.NetworkManager.IP4Config",
                                                                          "Domains")
        }
        fn searches(&self) -> Result<Vec<String>, dbus::Error> {
            <Self as
                blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self,
                                                                          "org.freedesktop.NetworkManager.IP4Config",
                                                                          "Searches")
        }
        fn dns_options(&self) -> Result<Vec<String>, dbus::Error> {
            <Self as
                blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self,
                                                                          "org.freedesktop.NetworkManager.IP4Config",
                                                                          "DnsOptions")
        }
        fn dns_priority(&self) -> Result<i32, dbus::Error> {
            <Self as
                blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self,
                                                                          "org.freedesktop.NetworkManager.IP4Config",
                                                                          "DnsPriority")
        }
        fn wins_server_data(&self) -> Result<Vec<String>, dbus::Error> {
            <Self as
                blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self,
                                                                          "org.freedesktop.NetworkManager.IP4Config",
                                                                          "WinsServerData")
        }
        fn wins_servers(&self) -> Result<Vec<u32>, dbus::Error> {
            <Self as
                blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self,
                                                                          "org.freedesktop.NetworkManager.IP4Config",
                                                                          "WinsServers")
        }
    }
    pub struct OrgFreedesktopNetworkManagerIP4ConfigPropertiesChanged {
        pub properties: ::std::collections::HashMap<String,
                                                    arg::Variant<Box<dyn arg::RefArg +
                                                                     'static>>>,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::fmt::Debug for
     OrgFreedesktopNetworkManagerIP4ConfigPropertiesChanged {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match *self {
                OrgFreedesktopNetworkManagerIP4ConfigPropertiesChanged {
                properties: ref __self_0_0 } => {
                    let mut debug_trait_builder =
                        f.debug_struct("OrgFreedesktopNetworkManagerIP4ConfigPropertiesChanged");
                    let _ =
                        debug_trait_builder.field("properties",
                                                  &&(*__self_0_0));
                    debug_trait_builder.finish()
                }
            }
        }
    }
    impl arg::AppendAll for
     OrgFreedesktopNetworkManagerIP4ConfigPropertiesChanged {
        fn append(&self, i: &mut arg::IterAppend) {
            arg::RefArg::append(&self.properties, i);
        }
    }
    impl arg::ReadAll for
     OrgFreedesktopNetworkManagerIP4ConfigPropertiesChanged {
        fn read(i: &mut arg::Iter) -> Result<Self, arg::TypeMismatchError> {
            Ok(OrgFreedesktopNetworkManagerIP4ConfigPropertiesChanged{properties:
                                                                          i.read()?,})
        }
    }
    impl dbus::message::SignalArgs for
     OrgFreedesktopNetworkManagerIP4ConfigPropertiesChanged {
        const NAME: &'static str = "PropertiesChanged";
        const INTERFACE: &'static str =
            "org.freedesktop.NetworkManager.IP4Config";
    }
}
mod network_device {
    use dbus as dbus;
    use dbus::arg;
    use dbus::blocking;
    pub trait OrgFreedesktopDBusProperties {
        fn get<R0: for<'b> arg::Get<'b> +
               'static>(&self, interface_name: &str, property_name: &str)
        -> Result<R0, dbus::Error>;
        fn get_all(&self, interface_name: &str)
        ->
            Result<::std::collections::HashMap<String,
                                               arg::Variant<Box<dyn arg::RefArg +
                                                                'static>>>,
                   dbus::Error>;
        fn set<I2: arg::Arg +
               arg::Append>(&self, interface_name: &str, property_name: &str,
                            value: I2)
        -> Result<(), dbus::Error>;
    }
    impl <'a, C: ::std::ops::Deref<Target = blocking::Connection>>
     OrgFreedesktopDBusProperties for blocking::Proxy<'a, C> {
        fn get<R0: for<'b> arg::Get<'b> +
               'static>(&self, interface_name: &str, property_name: &str)
         -> Result<R0, dbus::Error> {
            self.method_call("org.freedesktop.DBus.Properties", "Get",
                             (interface_name,
                              property_name)).and_then(|r:
                                                            (arg::Variant<R0>,)|
                                                           Ok((r.0).0))
        }
        fn get_all(&self, interface_name: &str)
         ->
             Result<::std::collections::HashMap<String,
                                                arg::Variant<Box<dyn arg::RefArg +
                                                                 'static>>>,
                    dbus::Error> {
            self.method_call("org.freedesktop.DBus.Properties", "GetAll",
                             (interface_name,)).and_then(|r:
                                                              (::std::collections::HashMap<String,
                                                                                           arg::Variant<Box<dyn arg::RefArg +
                                                                                                            'static>>>,)|
                                                             Ok(r.0))
        }
        fn set<I2: arg::Arg +
               arg::Append>(&self, interface_name: &str, property_name: &str,
                            value: I2) -> Result<(), dbus::Error> {
            self.method_call("org.freedesktop.DBus.Properties", "Set",
                             (interface_name, property_name,
                              arg::Variant(value)))
        }
    }
    pub struct OrgFreedesktopDBusPropertiesPropertiesChanged {
        pub interface_name: String,
        pub changed_properties: ::std::collections::HashMap<String,
                                                            arg::Variant<Box<dyn arg::RefArg +
                                                                             'static>>>,
        pub invalidated_properties: Vec<String>,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::fmt::Debug for OrgFreedesktopDBusPropertiesPropertiesChanged
     {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match *self {
                OrgFreedesktopDBusPropertiesPropertiesChanged {
                interface_name: ref __self_0_0,
                changed_properties: ref __self_0_1,
                invalidated_properties: ref __self_0_2 } => {
                    let mut debug_trait_builder =
                        f.debug_struct("OrgFreedesktopDBusPropertiesPropertiesChanged");
                    let _ =
                        debug_trait_builder.field("interface_name",
                                                  &&(*__self_0_0));
                    let _ =
                        debug_trait_builder.field("changed_properties",
                                                  &&(*__self_0_1));
                    let _ =
                        debug_trait_builder.field("invalidated_properties",
                                                  &&(*__self_0_2));
                    debug_trait_builder.finish()
                }
            }
        }
    }
    impl arg::AppendAll for OrgFreedesktopDBusPropertiesPropertiesChanged {
        fn append(&self, i: &mut arg::IterAppend) {
            arg::RefArg::append(&self.interface_name, i);
            arg::RefArg::append(&self.changed_properties, i);
            arg::RefArg::append(&self.invalidated_properties, i);
        }
    }
    impl arg::ReadAll for OrgFreedesktopDBusPropertiesPropertiesChanged {
        fn read(i: &mut arg::Iter) -> Result<Self, arg::TypeMismatchError> {
            Ok(OrgFreedesktopDBusPropertiesPropertiesChanged{interface_name:
                                                                 i.read()?,
                                                             changed_properties:
                                                                 i.read()?,
                                                             invalidated_properties:
                                                                 i.read()?,})
        }
    }
    impl dbus::message::SignalArgs for
     OrgFreedesktopDBusPropertiesPropertiesChanged {
        const NAME: &'static str = "PropertiesChanged";
        const INTERFACE: &'static str = "org.freedesktop.DBus.Properties";
    }
    pub trait OrgFreedesktopDBusIntrospectable {
        fn introspect(&self)
        -> Result<String, dbus::Error>;
    }
    impl <'a, C: ::std::ops::Deref<Target = blocking::Connection>>
     OrgFreedesktopDBusIntrospectable for blocking::Proxy<'a, C> {
        fn introspect(&self) -> Result<String, dbus::Error> {
            self.method_call("org.freedesktop.DBus.Introspectable",
                             "Introspect",
                             ()).and_then(|r: (String,)| Ok(r.0))
        }
    }
    pub trait OrgFreedesktopDBusPeer {
        fn ping(&self)
        -> Result<(), dbus::Error>;
        fn get_machine_id(&self)
        -> Result<String, dbus::Error>;
    }
    impl <'a, C: ::std::ops::Deref<Target = blocking::Connection>>
     OrgFreedesktopDBusPeer for blocking::Proxy<'a, C> {
        fn ping(&self) -> Result<(), dbus::Error> {
            self.method_call("org.freedesktop.DBus.Peer", "Ping", ())
        }
        fn get_machine_id(&self) -> Result<String, dbus::Error> {
            self.method_call("org.freedesktop.DBus.Peer", "GetMachineId",
                             ()).and_then(|r: (String,)| Ok(r.0))
        }
    }
    pub trait OrgFreedesktopNetworkManagerDeviceStatistics {
        fn refresh_rate_ms(&self)
        -> Result<u32, dbus::Error>;
        fn set_refresh_rate_ms(&self, value: u32)
        -> Result<(), dbus::Error>;
        fn tx_bytes(&self)
        -> Result<u64, dbus::Error>;
        fn rx_bytes(&self)
        -> Result<u64, dbus::Error>;
    }
    impl <'a, C: ::std::ops::Deref<Target = blocking::Connection>>
     OrgFreedesktopNetworkManagerDeviceStatistics for blocking::Proxy<'a, C> {
        fn refresh_rate_ms(&self) -> Result<u32, dbus::Error> {
            <Self as
                blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self,
                                                                          "org.freedesktop.NetworkManager.Device.Statistics",
                                                                          "RefreshRateMs")
        }
        fn tx_bytes(&self) -> Result<u64, dbus::Error> {
            <Self as
                blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self,
                                                                          "org.freedesktop.NetworkManager.Device.Statistics",
                                                                          "TxBytes")
        }
        fn rx_bytes(&self) -> Result<u64, dbus::Error> {
            <Self as
                blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self,
                                                                          "org.freedesktop.NetworkManager.Device.Statistics",
                                                                          "RxBytes")
        }
        fn set_refresh_rate_ms(&self, value: u32) -> Result<(), dbus::Error> {
            <Self as
                blocking::stdintf::org_freedesktop_dbus::Properties>::set(&self,
                                                                          "org.freedesktop.NetworkManager.Device.Statistics",
                                                                          "RefreshRateMs",
                                                                          value)
        }
    }
    pub struct OrgFreedesktopNetworkManagerDeviceStatisticsPropertiesChanged {
        pub properties: ::std::collections::HashMap<String,
                                                    arg::Variant<Box<dyn arg::RefArg +
                                                                     'static>>>,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::fmt::Debug for
     OrgFreedesktopNetworkManagerDeviceStatisticsPropertiesChanged {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match *self {
                OrgFreedesktopNetworkManagerDeviceStatisticsPropertiesChanged {
                properties: ref __self_0_0 } => {
                    let mut debug_trait_builder =
                        f.debug_struct("OrgFreedesktopNetworkManagerDeviceStatisticsPropertiesChanged");
                    let _ =
                        debug_trait_builder.field("properties",
                                                  &&(*__self_0_0));
                    debug_trait_builder.finish()
                }
            }
        }
    }
    impl arg::AppendAll for
     OrgFreedesktopNetworkManagerDeviceStatisticsPropertiesChanged {
        fn append(&self, i: &mut arg::IterAppend) {
            arg::RefArg::append(&self.properties, i);
        }
    }
    impl arg::ReadAll for
     OrgFreedesktopNetworkManagerDeviceStatisticsPropertiesChanged {
        fn read(i: &mut arg::Iter) -> Result<Self, arg::TypeMismatchError> {
            Ok(OrgFreedesktopNetworkManagerDeviceStatisticsPropertiesChanged{properties:
                                                                                 i.read()?,})
        }
    }
    impl dbus::message::SignalArgs for
     OrgFreedesktopNetworkManagerDeviceStatisticsPropertiesChanged {
        const NAME: &'static str = "PropertiesChanged";
        const INTERFACE: &'static str =
            "org.freedesktop.NetworkManager.Device.Statistics";
    }
    pub trait OrgFreedesktopNetworkManagerDeviceWireless {
        fn get_access_points(&self)
        -> Result<Vec<dbus::Path<'static>>, dbus::Error>;
        fn get_all_access_points(&self)
        -> Result<Vec<dbus::Path<'static>>, dbus::Error>;
        fn request_scan(&self,
                        options:
                            ::std::collections::HashMap<&str,
                                                        arg::Variant<Box<dyn arg::RefArg>>>)
        -> Result<(), dbus::Error>;
        fn hw_address(&self)
        -> Result<String, dbus::Error>;
        fn perm_hw_address(&self)
        -> Result<String, dbus::Error>;
        fn mode(&self)
        -> Result<u32, dbus::Error>;
        fn bitrate(&self)
        -> Result<u32, dbus::Error>;
        fn access_points(&self)
        -> Result<Vec<dbus::Path<'static>>, dbus::Error>;
        fn active_access_point(&self)
        -> Result<dbus::Path<'static>, dbus::Error>;
        fn wireless_capabilities(&self)
        -> Result<u32, dbus::Error>;
        fn last_scan(&self)
        -> Result<i64, dbus::Error>;
    }
    impl <'a, C: ::std::ops::Deref<Target = blocking::Connection>>
     OrgFreedesktopNetworkManagerDeviceWireless for blocking::Proxy<'a, C> {
        fn get_access_points(&self)
         -> Result<Vec<dbus::Path<'static>>, dbus::Error> {
            self.method_call("org.freedesktop.NetworkManager.Device.Wireless",
                             "GetAccessPoints",
                             ()).and_then(|r: (Vec<dbus::Path<'static>>,)|
                                              Ok(r.0))
        }
        fn get_all_access_points(&self)
         -> Result<Vec<dbus::Path<'static>>, dbus::Error> {
            self.method_call("org.freedesktop.NetworkManager.Device.Wireless",
                             "GetAllAccessPoints",
                             ()).and_then(|r: (Vec<dbus::Path<'static>>,)|
                                              Ok(r.0))
        }
        fn request_scan(&self,
                        options:
                            ::std::collections::HashMap<&str,
                                                        arg::Variant<Box<dyn arg::RefArg>>>)
         -> Result<(), dbus::Error> {
            self.method_call("org.freedesktop.NetworkManager.Device.Wireless",
                             "RequestScan", (options,))
        }
        fn hw_address(&self) -> Result<String, dbus::Error> {
            <Self as
                blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self,
                                                                          "org.freedesktop.NetworkManager.Device.Wireless",
                                                                          "HwAddress")
        }
        fn perm_hw_address(&self) -> Result<String, dbus::Error> {
            <Self as
                blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self,
                                                                          "org.freedesktop.NetworkManager.Device.Wireless",
                                                                          "PermHwAddress")
        }
        fn mode(&self) -> Result<u32, dbus::Error> {
            <Self as
                blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self,
                                                                          "org.freedesktop.NetworkManager.Device.Wireless",
                                                                          "Mode")
        }
        fn bitrate(&self) -> Result<u32, dbus::Error> {
            <Self as
                blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self,
                                                                          "org.freedesktop.NetworkManager.Device.Wireless",
                                                                          "Bitrate")
        }
        fn access_points(&self)
         -> Result<Vec<dbus::Path<'static>>, dbus::Error> {
            <Self as
                blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self,
                                                                          "org.freedesktop.NetworkManager.Device.Wireless",
                                                                          "AccessPoints")
        }
        fn active_access_point(&self)
         -> Result<dbus::Path<'static>, dbus::Error> {
            <Self as
                blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self,
                                                                          "org.freedesktop.NetworkManager.Device.Wireless",
                                                                          "ActiveAccessPoint")
        }
        fn wireless_capabilities(&self) -> Result<u32, dbus::Error> {
            <Self as
                blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self,
                                                                          "org.freedesktop.NetworkManager.Device.Wireless",
                                                                          "WirelessCapabilities")
        }
        fn last_scan(&self) -> Result<i64, dbus::Error> {
            <Self as
                blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self,
                                                                          "org.freedesktop.NetworkManager.Device.Wireless",
                                                                          "LastScan")
        }
    }
    pub struct OrgFreedesktopNetworkManagerDeviceWirelessPropertiesChanged {
        pub properties: ::std::collections::HashMap<String,
                                                    arg::Variant<Box<dyn arg::RefArg +
                                                                     'static>>>,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::fmt::Debug for
     OrgFreedesktopNetworkManagerDeviceWirelessPropertiesChanged {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match *self {
                OrgFreedesktopNetworkManagerDeviceWirelessPropertiesChanged {
                properties: ref __self_0_0 } => {
                    let mut debug_trait_builder =
                        f.debug_struct("OrgFreedesktopNetworkManagerDeviceWirelessPropertiesChanged");
                    let _ =
                        debug_trait_builder.field("properties",
                                                  &&(*__self_0_0));
                    debug_trait_builder.finish()
                }
            }
        }
    }
    impl arg::AppendAll for
     OrgFreedesktopNetworkManagerDeviceWirelessPropertiesChanged {
        fn append(&self, i: &mut arg::IterAppend) {
            arg::RefArg::append(&self.properties, i);
        }
    }
    impl arg::ReadAll for
     OrgFreedesktopNetworkManagerDeviceWirelessPropertiesChanged {
        fn read(i: &mut arg::Iter) -> Result<Self, arg::TypeMismatchError> {
            Ok(OrgFreedesktopNetworkManagerDeviceWirelessPropertiesChanged{properties:
                                                                               i.read()?,})
        }
    }
    impl dbus::message::SignalArgs for
     OrgFreedesktopNetworkManagerDeviceWirelessPropertiesChanged {
        const NAME: &'static str = "PropertiesChanged";
        const INTERFACE: &'static str =
            "org.freedesktop.NetworkManager.Device.Wireless";
    }
    pub struct OrgFreedesktopNetworkManagerDeviceWirelessAccessPointAdded {
        pub access_point: dbus::Path<'static>,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::fmt::Debug for
     OrgFreedesktopNetworkManagerDeviceWirelessAccessPointAdded {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match *self {
                OrgFreedesktopNetworkManagerDeviceWirelessAccessPointAdded {
                access_point: ref __self_0_0 } => {
                    let mut debug_trait_builder =
                        f.debug_struct("OrgFreedesktopNetworkManagerDeviceWirelessAccessPointAdded");
                    let _ =
                        debug_trait_builder.field("access_point",
                                                  &&(*__self_0_0));
                    debug_trait_builder.finish()
                }
            }
        }
    }
    impl arg::AppendAll for
     OrgFreedesktopNetworkManagerDeviceWirelessAccessPointAdded {
        fn append(&self, i: &mut arg::IterAppend) {
            arg::RefArg::append(&self.access_point, i);
        }
    }
    impl arg::ReadAll for
     OrgFreedesktopNetworkManagerDeviceWirelessAccessPointAdded {
        fn read(i: &mut arg::Iter) -> Result<Self, arg::TypeMismatchError> {
            Ok(OrgFreedesktopNetworkManagerDeviceWirelessAccessPointAdded{access_point:
                                                                              i.read()?,})
        }
    }
    impl dbus::message::SignalArgs for
     OrgFreedesktopNetworkManagerDeviceWirelessAccessPointAdded {
        const NAME: &'static str = "AccessPointAdded";
        const INTERFACE: &'static str =
            "org.freedesktop.NetworkManager.Device.Wireless";
    }
    pub struct OrgFreedesktopNetworkManagerDeviceWirelessAccessPointRemoved {
        pub access_point: dbus::Path<'static>,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::fmt::Debug for
     OrgFreedesktopNetworkManagerDeviceWirelessAccessPointRemoved {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match *self {
                OrgFreedesktopNetworkManagerDeviceWirelessAccessPointRemoved {
                access_point: ref __self_0_0 } => {
                    let mut debug_trait_builder =
                        f.debug_struct("OrgFreedesktopNetworkManagerDeviceWirelessAccessPointRemoved");
                    let _ =
                        debug_trait_builder.field("access_point",
                                                  &&(*__self_0_0));
                    debug_trait_builder.finish()
                }
            }
        }
    }
    impl arg::AppendAll for
     OrgFreedesktopNetworkManagerDeviceWirelessAccessPointRemoved {
        fn append(&self, i: &mut arg::IterAppend) {
            arg::RefArg::append(&self.access_point, i);
        }
    }
    impl arg::ReadAll for
     OrgFreedesktopNetworkManagerDeviceWirelessAccessPointRemoved {
        fn read(i: &mut arg::Iter) -> Result<Self, arg::TypeMismatchError> {
            Ok(OrgFreedesktopNetworkManagerDeviceWirelessAccessPointRemoved{access_point:
                                                                                i.read()?,})
        }
    }
    impl dbus::message::SignalArgs for
     OrgFreedesktopNetworkManagerDeviceWirelessAccessPointRemoved {
        const NAME: &'static str = "AccessPointRemoved";
        const INTERFACE: &'static str =
            "org.freedesktop.NetworkManager.Device.Wireless";
    }
    pub trait OrgFreedesktopNetworkManagerDevice {
        fn reapply(&self,
                   connection:
                       ::std::collections::HashMap<&str,
                                                   ::std::collections::HashMap<&str,
                                                                               arg::Variant<Box<dyn arg::RefArg>>>>,
                   version_id: u64, flags: u32)
        -> Result<(), dbus::Error>;
        fn get_applied_connection(&self, flags: u32)
        ->
            Result<(::std::collections::HashMap<String,
                                                ::std::collections::HashMap<String,
                                                                            arg::Variant<Box<dyn arg::RefArg +
                                                                                             'static>>>>,
                    u64), dbus::Error>;
        fn disconnect(&self)
        -> Result<(), dbus::Error>;
        fn delete(&self)
        -> Result<(), dbus::Error>;
        fn udi(&self)
        -> Result<String, dbus::Error>;
        fn interface(&self)
        -> Result<String, dbus::Error>;
        fn ip_interface(&self)
        -> Result<String, dbus::Error>;
        fn driver(&self)
        -> Result<String, dbus::Error>;
        fn driver_version(&self)
        -> Result<String, dbus::Error>;
        fn firmware_version(&self)
        -> Result<String, dbus::Error>;
        fn capabilities(&self)
        -> Result<u32, dbus::Error>;
        fn ip4_address(&self)
        -> Result<u32, dbus::Error>;
        fn state(&self)
        -> Result<u32, dbus::Error>;
        fn state_reason(&self)
        -> Result<(u32, u32), dbus::Error>;
        fn active_connection(&self)
        -> Result<dbus::Path<'static>, dbus::Error>;
        fn ip4_config(&self)
        -> Result<dbus::Path<'static>, dbus::Error>;
        fn dhcp4_config(&self)
        -> Result<dbus::Path<'static>, dbus::Error>;
        fn ip6_config(&self)
        -> Result<dbus::Path<'static>, dbus::Error>;
        fn dhcp6_config(&self)
        -> Result<dbus::Path<'static>, dbus::Error>;
        fn managed(&self)
        -> Result<bool, dbus::Error>;
        fn set_managed(&self, value: bool)
        -> Result<(), dbus::Error>;
        fn autoconnect(&self)
        -> Result<bool, dbus::Error>;
        fn set_autoconnect(&self, value: bool)
        -> Result<(), dbus::Error>;
        fn firmware_missing(&self)
        -> Result<bool, dbus::Error>;
        fn nm_plugin_missing(&self)
        -> Result<bool, dbus::Error>;
        fn device_type(&self)
        -> Result<u32, dbus::Error>;
        fn available_connections(&self)
        -> Result<Vec<dbus::Path<'static>>, dbus::Error>;
        fn physical_port_id(&self)
        -> Result<String, dbus::Error>;
        fn mtu(&self)
        -> Result<u32, dbus::Error>;
        fn metered(&self)
        -> Result<u32, dbus::Error>;
        fn lldp_neighbors(&self)
        ->
            Result<Vec<::std::collections::HashMap<String,
                                                   arg::Variant<Box<dyn arg::RefArg +
                                                                    'static>>>>,
                   dbus::Error>;
        fn real(&self)
        -> Result<bool, dbus::Error>;
        fn ip4_connectivity(&self)
        -> Result<u32, dbus::Error>;
        fn ip6_connectivity(&self)
        -> Result<u32, dbus::Error>;
        fn interface_flags(&self)
        -> Result<u32, dbus::Error>;
    }
    impl <'a, C: ::std::ops::Deref<Target = blocking::Connection>>
     OrgFreedesktopNetworkManagerDevice for blocking::Proxy<'a, C> {
        fn reapply(&self,
                   connection:
                       ::std::collections::HashMap<&str,
                                                   ::std::collections::HashMap<&str,
                                                                               arg::Variant<Box<dyn arg::RefArg>>>>,
                   version_id: u64, flags: u32) -> Result<(), dbus::Error> {
            self.method_call("org.freedesktop.NetworkManager.Device",
                             "Reapply", (connection, version_id, flags))
        }
        fn get_applied_connection(&self, flags: u32)
         ->
             Result<(::std::collections::HashMap<String,
                                                 ::std::collections::HashMap<String,
                                                                             arg::Variant<Box<dyn arg::RefArg +
                                                                                              'static>>>>,
                     u64), dbus::Error> {
            self.method_call("org.freedesktop.NetworkManager.Device",
                             "GetAppliedConnection", (flags,))
        }
        fn disconnect(&self) -> Result<(), dbus::Error> {
            self.method_call("org.freedesktop.NetworkManager.Device",
                             "Disconnect", ())
        }
        fn delete(&self) -> Result<(), dbus::Error> {
            self.method_call("org.freedesktop.NetworkManager.Device",
                             "Delete", ())
        }
        fn udi(&self) -> Result<String, dbus::Error> {
            <Self as
                blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self,
                                                                          "org.freedesktop.NetworkManager.Device",
                                                                          "Udi")
        }
        fn interface(&self) -> Result<String, dbus::Error> {
            <Self as
                blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self,
                                                                          "org.freedesktop.NetworkManager.Device",
                                                                          "Interface")
        }
        fn ip_interface(&self) -> Result<String, dbus::Error> {
            <Self as
                blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self,
                                                                          "org.freedesktop.NetworkManager.Device",
                                                                          "IpInterface")
        }
        fn driver(&self) -> Result<String, dbus::Error> {
            <Self as
                blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self,
                                                                          "org.freedesktop.NetworkManager.Device",
                                                                          "Driver")
        }
        fn driver_version(&self) -> Result<String, dbus::Error> {
            <Self as
                blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self,
                                                                          "org.freedesktop.NetworkManager.Device",
                                                                          "DriverVersion")
        }
        fn firmware_version(&self) -> Result<String, dbus::Error> {
            <Self as
                blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self,
                                                                          "org.freedesktop.NetworkManager.Device",
                                                                          "FirmwareVersion")
        }
        fn capabilities(&self) -> Result<u32, dbus::Error> {
            <Self as
                blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self,
                                                                          "org.freedesktop.NetworkManager.Device",
                                                                          "Capabilities")
        }
        fn ip4_address(&self) -> Result<u32, dbus::Error> {
            <Self as
                blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self,
                                                                          "org.freedesktop.NetworkManager.Device",
                                                                          "Ip4Address")
        }
        fn state(&self) -> Result<u32, dbus::Error> {
            <Self as
                blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self,
                                                                          "org.freedesktop.NetworkManager.Device",
                                                                          "State")
        }
        fn state_reason(&self) -> Result<(u32, u32), dbus::Error> {
            <Self as
                blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self,
                                                                          "org.freedesktop.NetworkManager.Device",
                                                                          "StateReason")
        }
        fn active_connection(&self)
         -> Result<dbus::Path<'static>, dbus::Error> {
            <Self as
                blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self,
                                                                          "org.freedesktop.NetworkManager.Device",
                                                                          "ActiveConnection")
        }
        fn ip4_config(&self) -> Result<dbus::Path<'static>, dbus::Error> {
            <Self as
                blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self,
                                                                          "org.freedesktop.NetworkManager.Device",
                                                                          "Ip4Config")
        }
        fn dhcp4_config(&self) -> Result<dbus::Path<'static>, dbus::Error> {
            <Self as
                blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self,
                                                                          "org.freedesktop.NetworkManager.Device",
                                                                          "Dhcp4Config")
        }
        fn ip6_config(&self) -> Result<dbus::Path<'static>, dbus::Error> {
            <Self as
                blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self,
                                                                          "org.freedesktop.NetworkManager.Device",
                                                                          "Ip6Config")
        }
        fn dhcp6_config(&self) -> Result<dbus::Path<'static>, dbus::Error> {
            <Self as
                blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self,
                                                                          "org.freedesktop.NetworkManager.Device",
                                                                          "Dhcp6Config")
        }
        fn managed(&self) -> Result<bool, dbus::Error> {
            <Self as
                blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self,
                                                                          "org.freedesktop.NetworkManager.Device",
                                                                          "Managed")
        }
        fn autoconnect(&self) -> Result<bool, dbus::Error> {
            <Self as
                blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self,
                                                                          "org.freedesktop.NetworkManager.Device",
                                                                          "Autoconnect")
        }
        fn firmware_missing(&self) -> Result<bool, dbus::Error> {
            <Self as
                blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self,
                                                                          "org.freedesktop.NetworkManager.Device",
                                                                          "FirmwareMissing")
        }
        fn nm_plugin_missing(&self) -> Result<bool, dbus::Error> {
            <Self as
                blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self,
                                                                          "org.freedesktop.NetworkManager.Device",
                                                                          "NmPluginMissing")
        }
        fn device_type(&self) -> Result<u32, dbus::Error> {
            <Self as
                blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self,
                                                                          "org.freedesktop.NetworkManager.Device",
                                                                          "DeviceType")
        }
        fn available_connections(&self)
         -> Result<Vec<dbus::Path<'static>>, dbus::Error> {
            <Self as
                blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self,
                                                                          "org.freedesktop.NetworkManager.Device",
                                                                          "AvailableConnections")
        }
        fn physical_port_id(&self) -> Result<String, dbus::Error> {
            <Self as
                blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self,
                                                                          "org.freedesktop.NetworkManager.Device",
                                                                          "PhysicalPortId")
        }
        fn mtu(&self) -> Result<u32, dbus::Error> {
            <Self as
                blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self,
                                                                          "org.freedesktop.NetworkManager.Device",
                                                                          "Mtu")
        }
        fn metered(&self) -> Result<u32, dbus::Error> {
            <Self as
                blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self,
                                                                          "org.freedesktop.NetworkManager.Device",
                                                                          "Metered")
        }
        fn lldp_neighbors(&self)
         ->
             Result<Vec<::std::collections::HashMap<String,
                                                    arg::Variant<Box<dyn arg::RefArg +
                                                                     'static>>>>,
                    dbus::Error> {
            <Self as
                blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self,
                                                                          "org.freedesktop.NetworkManager.Device",
                                                                          "LldpNeighbors")
        }
        fn real(&self) -> Result<bool, dbus::Error> {
            <Self as
                blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self,
                                                                          "org.freedesktop.NetworkManager.Device",
                                                                          "Real")
        }
        fn ip4_connectivity(&self) -> Result<u32, dbus::Error> {
            <Self as
                blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self,
                                                                          "org.freedesktop.NetworkManager.Device",
                                                                          "Ip4Connectivity")
        }
        fn ip6_connectivity(&self) -> Result<u32, dbus::Error> {
            <Self as
                blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self,
                                                                          "org.freedesktop.NetworkManager.Device",
                                                                          "Ip6Connectivity")
        }
        fn interface_flags(&self) -> Result<u32, dbus::Error> {
            <Self as
                blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self,
                                                                          "org.freedesktop.NetworkManager.Device",
                                                                          "InterfaceFlags")
        }
        fn set_managed(&self, value: bool) -> Result<(), dbus::Error> {
            <Self as
                blocking::stdintf::org_freedesktop_dbus::Properties>::set(&self,
                                                                          "org.freedesktop.NetworkManager.Device",
                                                                          "Managed",
                                                                          value)
        }
        fn set_autoconnect(&self, value: bool) -> Result<(), dbus::Error> {
            <Self as
                blocking::stdintf::org_freedesktop_dbus::Properties>::set(&self,
                                                                          "org.freedesktop.NetworkManager.Device",
                                                                          "Autoconnect",
                                                                          value)
        }
    }
    pub struct OrgFreedesktopNetworkManagerDeviceStateChanged {
        pub new_state: u32,
        pub old_state: u32,
        pub reason: u32,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::fmt::Debug for OrgFreedesktopNetworkManagerDeviceStateChanged
     {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match *self {
                OrgFreedesktopNetworkManagerDeviceStateChanged {
                new_state: ref __self_0_0,
                old_state: ref __self_0_1,
                reason: ref __self_0_2 } => {
                    let mut debug_trait_builder =
                        f.debug_struct("OrgFreedesktopNetworkManagerDeviceStateChanged");
                    let _ =
                        debug_trait_builder.field("new_state",
                                                  &&(*__self_0_0));
                    let _ =
                        debug_trait_builder.field("old_state",
                                                  &&(*__self_0_1));
                    let _ =
                        debug_trait_builder.field("reason", &&(*__self_0_2));
                    debug_trait_builder.finish()
                }
            }
        }
    }
    impl arg::AppendAll for OrgFreedesktopNetworkManagerDeviceStateChanged {
        fn append(&self, i: &mut arg::IterAppend) {
            arg::RefArg::append(&self.new_state, i);
            arg::RefArg::append(&self.old_state, i);
            arg::RefArg::append(&self.reason, i);
        }
    }
    impl arg::ReadAll for OrgFreedesktopNetworkManagerDeviceStateChanged {
        fn read(i: &mut arg::Iter) -> Result<Self, arg::TypeMismatchError> {
            Ok(OrgFreedesktopNetworkManagerDeviceStateChanged{new_state:
                                                                  i.read()?,
                                                              old_state:
                                                                  i.read()?,
                                                              reason:
                                                                  i.read()?,})
        }
    }
    impl dbus::message::SignalArgs for
     OrgFreedesktopNetworkManagerDeviceStateChanged {
        const NAME: &'static str = "StateChanged";
        const INTERFACE: &'static str =
            "org.freedesktop.NetworkManager.Device";
    }
}
pub use manager::NetworkManager;
