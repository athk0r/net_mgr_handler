// This code was autogenerated with `dbus-codegen-rust -s -g -m None -d org.freedesktop.NetworkManager -p /org/freedesktop/NetworkManager/Devices/3`, see https://github.com/diwic/dbus-rs
use dbus as dbus;
use dbus::arg;
use dbus::blocking;

pub trait OrgFreedesktopDBusProperties {
    fn get<R0: for<'b> arg::Get<'b> + 'static>(&self, interface_name: &str, property_name: &str) -> Result<R0, dbus::Error>;
    fn get_all(&self, interface_name: &str) -> Result<::std::collections::HashMap<String, arg::Variant<Box<dyn arg::RefArg + 'static>>>, dbus::Error>;
    fn set<I2: arg::Arg + arg::Append>(&self, interface_name: &str, property_name: &str, value: I2) -> Result<(), dbus::Error>;
}

impl<'a, C: ::std::ops::Deref<Target=blocking::Connection>> OrgFreedesktopDBusProperties for blocking::Proxy<'a, C> {

    fn get<R0: for<'b> arg::Get<'b> + 'static>(&self, interface_name: &str, property_name: &str) -> Result<R0, dbus::Error> {
        self.method_call("org.freedesktop.DBus.Properties", "Get", (interface_name, property_name, ))
            .and_then(|r: (arg::Variant<R0>, )| Ok((r.0).0, ))
    }

    fn get_all(&self, interface_name: &str) -> Result<::std::collections::HashMap<String, arg::Variant<Box<dyn arg::RefArg + 'static>>>, dbus::Error> {
        self.method_call("org.freedesktop.DBus.Properties", "GetAll", (interface_name, ))
            .and_then(|r: (::std::collections::HashMap<String, arg::Variant<Box<dyn arg::RefArg + 'static>>>, )| Ok(r.0, ))
    }

    fn set<I2: arg::Arg + arg::Append>(&self, interface_name: &str, property_name: &str, value: I2) -> Result<(), dbus::Error> {
        self.method_call("org.freedesktop.DBus.Properties", "Set", (interface_name, property_name, arg::Variant(value), ))
    }
}

#[derive(Debug)]
pub struct OrgFreedesktopDBusPropertiesPropertiesChanged {
    pub interface_name: String,
    pub changed_properties: ::std::collections::HashMap<String, arg::Variant<Box<dyn arg::RefArg + 'static>>>,
    pub invalidated_properties: Vec<String>,
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
        Ok(OrgFreedesktopDBusPropertiesPropertiesChanged {
            interface_name: i.read()?,
            changed_properties: i.read()?,
            invalidated_properties: i.read()?,
        })
    }
}

impl dbus::message::SignalArgs for OrgFreedesktopDBusPropertiesPropertiesChanged {
    const NAME: &'static str = "PropertiesChanged";
    const INTERFACE: &'static str = "org.freedesktop.DBus.Properties";
}

pub trait OrgFreedesktopDBusIntrospectable {
    fn introspect(&self) -> Result<String, dbus::Error>;
}

impl<'a, C: ::std::ops::Deref<Target=blocking::Connection>> OrgFreedesktopDBusIntrospectable for blocking::Proxy<'a, C> {

    fn introspect(&self) -> Result<String, dbus::Error> {
        self.method_call("org.freedesktop.DBus.Introspectable", "Introspect", ())
            .and_then(|r: (String, )| Ok(r.0, ))
    }
}

pub trait OrgFreedesktopDBusPeer {
    fn ping(&self) -> Result<(), dbus::Error>;
    fn get_machine_id(&self) -> Result<String, dbus::Error>;
}

impl<'a, C: ::std::ops::Deref<Target=blocking::Connection>> OrgFreedesktopDBusPeer for blocking::Proxy<'a, C> {

    fn ping(&self) -> Result<(), dbus::Error> {
        self.method_call("org.freedesktop.DBus.Peer", "Ping", ())
    }

    fn get_machine_id(&self) -> Result<String, dbus::Error> {
        self.method_call("org.freedesktop.DBus.Peer", "GetMachineId", ())
            .and_then(|r: (String, )| Ok(r.0, ))
    }
}

pub trait OrgFreedesktopNetworkManagerDeviceStatistics {
    fn refresh_rate_ms(&self) -> Result<u32, dbus::Error>;
    fn set_refresh_rate_ms(&self, value: u32) -> Result<(), dbus::Error>;
    fn tx_bytes(&self) -> Result<u64, dbus::Error>;
    fn rx_bytes(&self) -> Result<u64, dbus::Error>;
}

impl<'a, C: ::std::ops::Deref<Target=blocking::Connection>> OrgFreedesktopNetworkManagerDeviceStatistics for blocking::Proxy<'a, C> {

    fn refresh_rate_ms(&self) -> Result<u32, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.NetworkManager.Device.Statistics", "RefreshRateMs")
    }

    fn tx_bytes(&self) -> Result<u64, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.NetworkManager.Device.Statistics", "TxBytes")
    }

    fn rx_bytes(&self) -> Result<u64, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.NetworkManager.Device.Statistics", "RxBytes")
    }

    fn set_refresh_rate_ms(&self, value: u32) -> Result<(), dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::set(&self, "org.freedesktop.NetworkManager.Device.Statistics", "RefreshRateMs", value)
    }
}

#[derive(Debug)]
pub struct OrgFreedesktopNetworkManagerDeviceStatisticsPropertiesChanged {
    pub properties: ::std::collections::HashMap<String, arg::Variant<Box<dyn arg::RefArg + 'static>>>,
}

impl arg::AppendAll for OrgFreedesktopNetworkManagerDeviceStatisticsPropertiesChanged {
    fn append(&self, i: &mut arg::IterAppend) {
        arg::RefArg::append(&self.properties, i);
    }
}

impl arg::ReadAll for OrgFreedesktopNetworkManagerDeviceStatisticsPropertiesChanged {
    fn read(i: &mut arg::Iter) -> Result<Self, arg::TypeMismatchError> {
        Ok(OrgFreedesktopNetworkManagerDeviceStatisticsPropertiesChanged {
            properties: i.read()?,
        })
    }
}

impl dbus::message::SignalArgs for OrgFreedesktopNetworkManagerDeviceStatisticsPropertiesChanged {
    const NAME: &'static str = "PropertiesChanged";
    const INTERFACE: &'static str = "org.freedesktop.NetworkManager.Device.Statistics";
}

pub trait OrgFreedesktopNetworkManagerDeviceWireless {
    fn get_access_points(&self) -> Result<Vec<dbus::Path<'static>>, dbus::Error>;
    fn get_all_access_points(&self) -> Result<Vec<dbus::Path<'static>>, dbus::Error>;
    fn request_scan(&self, options: ::std::collections::HashMap<&str, arg::Variant<Box<dyn arg::RefArg>>>) -> Result<(), dbus::Error>;
    fn hw_address(&self) -> Result<String, dbus::Error>;
    fn perm_hw_address(&self) -> Result<String, dbus::Error>;
    fn mode(&self) -> Result<u32, dbus::Error>;
    fn bitrate(&self) -> Result<u32, dbus::Error>;
    fn access_points(&self) -> Result<Vec<dbus::Path<'static>>, dbus::Error>;
    fn active_access_point(&self) -> Result<dbus::Path<'static>, dbus::Error>;
    fn wireless_capabilities(&self) -> Result<u32, dbus::Error>;
    fn last_scan(&self) -> Result<i64, dbus::Error>;
}

impl<'a, C: ::std::ops::Deref<Target=blocking::Connection>> OrgFreedesktopNetworkManagerDeviceWireless for blocking::Proxy<'a, C> {

    fn get_access_points(&self) -> Result<Vec<dbus::Path<'static>>, dbus::Error> {
        self.method_call("org.freedesktop.NetworkManager.Device.Wireless", "GetAccessPoints", ())
            .and_then(|r: (Vec<dbus::Path<'static>>, )| Ok(r.0, ))
    }

    fn get_all_access_points(&self) -> Result<Vec<dbus::Path<'static>>, dbus::Error> {
        self.method_call("org.freedesktop.NetworkManager.Device.Wireless", "GetAllAccessPoints", ())
            .and_then(|r: (Vec<dbus::Path<'static>>, )| Ok(r.0, ))
    }

    fn request_scan(&self, options: ::std::collections::HashMap<&str, arg::Variant<Box<dyn arg::RefArg>>>) -> Result<(), dbus::Error> {
        self.method_call("org.freedesktop.NetworkManager.Device.Wireless", "RequestScan", (options, ))
    }

    fn hw_address(&self) -> Result<String, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.NetworkManager.Device.Wireless", "HwAddress")
    }

    fn perm_hw_address(&self) -> Result<String, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.NetworkManager.Device.Wireless", "PermHwAddress")
    }

    fn mode(&self) -> Result<u32, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.NetworkManager.Device.Wireless", "Mode")
    }

    fn bitrate(&self) -> Result<u32, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.NetworkManager.Device.Wireless", "Bitrate")
    }

    fn access_points(&self) -> Result<Vec<dbus::Path<'static>>, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.NetworkManager.Device.Wireless", "AccessPoints")
    }

    fn active_access_point(&self) -> Result<dbus::Path<'static>, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.NetworkManager.Device.Wireless", "ActiveAccessPoint")
    }

    fn wireless_capabilities(&self) -> Result<u32, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.NetworkManager.Device.Wireless", "WirelessCapabilities")
    }

    fn last_scan(&self) -> Result<i64, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.NetworkManager.Device.Wireless", "LastScan")
    }
}

#[derive(Debug)]
pub struct OrgFreedesktopNetworkManagerDeviceWirelessPropertiesChanged {
    pub properties: ::std::collections::HashMap<String, arg::Variant<Box<dyn arg::RefArg + 'static>>>,
}

impl arg::AppendAll for OrgFreedesktopNetworkManagerDeviceWirelessPropertiesChanged {
    fn append(&self, i: &mut arg::IterAppend) {
        arg::RefArg::append(&self.properties, i);
    }
}

impl arg::ReadAll for OrgFreedesktopNetworkManagerDeviceWirelessPropertiesChanged {
    fn read(i: &mut arg::Iter) -> Result<Self, arg::TypeMismatchError> {
        Ok(OrgFreedesktopNetworkManagerDeviceWirelessPropertiesChanged {
            properties: i.read()?,
        })
    }
}

impl dbus::message::SignalArgs for OrgFreedesktopNetworkManagerDeviceWirelessPropertiesChanged {
    const NAME: &'static str = "PropertiesChanged";
    const INTERFACE: &'static str = "org.freedesktop.NetworkManager.Device.Wireless";
}

#[derive(Debug)]
pub struct OrgFreedesktopNetworkManagerDeviceWirelessAccessPointAdded {
    pub access_point: dbus::Path<'static>,
}

impl arg::AppendAll for OrgFreedesktopNetworkManagerDeviceWirelessAccessPointAdded {
    fn append(&self, i: &mut arg::IterAppend) {
        arg::RefArg::append(&self.access_point, i);
    }
}

impl arg::ReadAll for OrgFreedesktopNetworkManagerDeviceWirelessAccessPointAdded {
    fn read(i: &mut arg::Iter) -> Result<Self, arg::TypeMismatchError> {
        Ok(OrgFreedesktopNetworkManagerDeviceWirelessAccessPointAdded {
            access_point: i.read()?,
        })
    }
}

impl dbus::message::SignalArgs for OrgFreedesktopNetworkManagerDeviceWirelessAccessPointAdded {
    const NAME: &'static str = "AccessPointAdded";
    const INTERFACE: &'static str = "org.freedesktop.NetworkManager.Device.Wireless";
}

#[derive(Debug)]
pub struct OrgFreedesktopNetworkManagerDeviceWirelessAccessPointRemoved {
    pub access_point: dbus::Path<'static>,
}

impl arg::AppendAll for OrgFreedesktopNetworkManagerDeviceWirelessAccessPointRemoved {
    fn append(&self, i: &mut arg::IterAppend) {
        arg::RefArg::append(&self.access_point, i);
    }
}

impl arg::ReadAll for OrgFreedesktopNetworkManagerDeviceWirelessAccessPointRemoved {
    fn read(i: &mut arg::Iter) -> Result<Self, arg::TypeMismatchError> {
        Ok(OrgFreedesktopNetworkManagerDeviceWirelessAccessPointRemoved {
            access_point: i.read()?,
        })
    }
}

impl dbus::message::SignalArgs for OrgFreedesktopNetworkManagerDeviceWirelessAccessPointRemoved {
    const NAME: &'static str = "AccessPointRemoved";
    const INTERFACE: &'static str = "org.freedesktop.NetworkManager.Device.Wireless";
}

pub trait OrgFreedesktopNetworkManagerDevice {
    fn reapply(&self, connection: ::std::collections::HashMap<&str, ::std::collections::HashMap<&str, arg::Variant<Box<dyn arg::RefArg>>>>, version_id: u64, flags: u32) -> Result<(), dbus::Error>;
    fn get_applied_connection(&self, flags: u32) -> Result<(::std::collections::HashMap<String, ::std::collections::HashMap<String, arg::Variant<Box<dyn arg::RefArg + 'static>>>>, u64), dbus::Error>;
    fn disconnect(&self) -> Result<(), dbus::Error>;
    fn delete(&self) -> Result<(), dbus::Error>;
    fn udi(&self) -> Result<String, dbus::Error>;
    fn interface(&self) -> Result<String, dbus::Error>;
    fn ip_interface(&self) -> Result<String, dbus::Error>;
    fn driver(&self) -> Result<String, dbus::Error>;
    fn driver_version(&self) -> Result<String, dbus::Error>;
    fn firmware_version(&self) -> Result<String, dbus::Error>;
    fn capabilities(&self) -> Result<u32, dbus::Error>;
    fn ip4_address(&self) -> Result<u32, dbus::Error>;
    fn state(&self) -> Result<u32, dbus::Error>;
    fn state_reason(&self) -> Result<(u32, u32), dbus::Error>;
    fn active_connection(&self) -> Result<dbus::Path<'static>, dbus::Error>;
    fn ip4_config(&self) -> Result<dbus::Path<'static>, dbus::Error>;
    fn dhcp4_config(&self) -> Result<dbus::Path<'static>, dbus::Error>;
    fn ip6_config(&self) -> Result<dbus::Path<'static>, dbus::Error>;
    fn dhcp6_config(&self) -> Result<dbus::Path<'static>, dbus::Error>;
    fn managed(&self) -> Result<bool, dbus::Error>;
    fn set_managed(&self, value: bool) -> Result<(), dbus::Error>;
    fn autoconnect(&self) -> Result<bool, dbus::Error>;
    fn set_autoconnect(&self, value: bool) -> Result<(), dbus::Error>;
    fn firmware_missing(&self) -> Result<bool, dbus::Error>;
    fn nm_plugin_missing(&self) -> Result<bool, dbus::Error>;
    fn device_type(&self) -> Result<u32, dbus::Error>;
    fn available_connections(&self) -> Result<Vec<dbus::Path<'static>>, dbus::Error>;
    fn physical_port_id(&self) -> Result<String, dbus::Error>;
    fn mtu(&self) -> Result<u32, dbus::Error>;
    fn metered(&self) -> Result<u32, dbus::Error>;
    fn lldp_neighbors(&self) -> Result<Vec<::std::collections::HashMap<String, arg::Variant<Box<dyn arg::RefArg + 'static>>>>, dbus::Error>;
    fn real(&self) -> Result<bool, dbus::Error>;
    fn ip4_connectivity(&self) -> Result<u32, dbus::Error>;
    fn ip6_connectivity(&self) -> Result<u32, dbus::Error>;
    fn interface_flags(&self) -> Result<u32, dbus::Error>;
}

impl<'a, C: ::std::ops::Deref<Target=blocking::Connection>> OrgFreedesktopNetworkManagerDevice for blocking::Proxy<'a, C> {

    fn reapply(&self, connection: ::std::collections::HashMap<&str, ::std::collections::HashMap<&str, arg::Variant<Box<dyn arg::RefArg>>>>, version_id: u64, flags: u32) -> Result<(), dbus::Error> {
        self.method_call("org.freedesktop.NetworkManager.Device", "Reapply", (connection, version_id, flags, ))
    }

    fn get_applied_connection(&self, flags: u32) -> Result<(::std::collections::HashMap<String, ::std::collections::HashMap<String, arg::Variant<Box<dyn arg::RefArg + 'static>>>>, u64), dbus::Error> {
        self.method_call("org.freedesktop.NetworkManager.Device", "GetAppliedConnection", (flags, ))
    }

    fn disconnect(&self) -> Result<(), dbus::Error> {
        self.method_call("org.freedesktop.NetworkManager.Device", "Disconnect", ())
    }

    fn delete(&self) -> Result<(), dbus::Error> {
        self.method_call("org.freedesktop.NetworkManager.Device", "Delete", ())
    }

    fn udi(&self) -> Result<String, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.NetworkManager.Device", "Udi")
    }

    fn interface(&self) -> Result<String, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.NetworkManager.Device", "Interface")
    }

    fn ip_interface(&self) -> Result<String, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.NetworkManager.Device", "IpInterface")
    }

    fn driver(&self) -> Result<String, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.NetworkManager.Device", "Driver")
    }

    fn driver_version(&self) -> Result<String, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.NetworkManager.Device", "DriverVersion")
    }

    fn firmware_version(&self) -> Result<String, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.NetworkManager.Device", "FirmwareVersion")
    }

    fn capabilities(&self) -> Result<u32, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.NetworkManager.Device", "Capabilities")
    }

    fn ip4_address(&self) -> Result<u32, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.NetworkManager.Device", "Ip4Address")
    }

    fn state(&self) -> Result<u32, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.NetworkManager.Device", "State")
    }

    fn state_reason(&self) -> Result<(u32, u32), dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.NetworkManager.Device", "StateReason")
    }

    fn active_connection(&self) -> Result<dbus::Path<'static>, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.NetworkManager.Device", "ActiveConnection")
    }

    fn ip4_config(&self) -> Result<dbus::Path<'static>, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.NetworkManager.Device", "Ip4Config")
    }

    fn dhcp4_config(&self) -> Result<dbus::Path<'static>, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.NetworkManager.Device", "Dhcp4Config")
    }

    fn ip6_config(&self) -> Result<dbus::Path<'static>, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.NetworkManager.Device", "Ip6Config")
    }

    fn dhcp6_config(&self) -> Result<dbus::Path<'static>, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.NetworkManager.Device", "Dhcp6Config")
    }

    fn managed(&self) -> Result<bool, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.NetworkManager.Device", "Managed")
    }

    fn autoconnect(&self) -> Result<bool, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.NetworkManager.Device", "Autoconnect")
    }

    fn firmware_missing(&self) -> Result<bool, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.NetworkManager.Device", "FirmwareMissing")
    }

    fn nm_plugin_missing(&self) -> Result<bool, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.NetworkManager.Device", "NmPluginMissing")
    }

    fn device_type(&self) -> Result<u32, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.NetworkManager.Device", "DeviceType")
    }

    fn available_connections(&self) -> Result<Vec<dbus::Path<'static>>, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.NetworkManager.Device", "AvailableConnections")
    }

    fn physical_port_id(&self) -> Result<String, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.NetworkManager.Device", "PhysicalPortId")
    }

    fn mtu(&self) -> Result<u32, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.NetworkManager.Device", "Mtu")
    }

    fn metered(&self) -> Result<u32, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.NetworkManager.Device", "Metered")
    }

    fn lldp_neighbors(&self) -> Result<Vec<::std::collections::HashMap<String, arg::Variant<Box<dyn arg::RefArg + 'static>>>>, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.NetworkManager.Device", "LldpNeighbors")
    }

    fn real(&self) -> Result<bool, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.NetworkManager.Device", "Real")
    }

    fn ip4_connectivity(&self) -> Result<u32, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.NetworkManager.Device", "Ip4Connectivity")
    }

    fn ip6_connectivity(&self) -> Result<u32, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.NetworkManager.Device", "Ip6Connectivity")
    }

    fn interface_flags(&self) -> Result<u32, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.NetworkManager.Device", "InterfaceFlags")
    }

    fn set_managed(&self, value: bool) -> Result<(), dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::set(&self, "org.freedesktop.NetworkManager.Device", "Managed", value)
    }

    fn set_autoconnect(&self, value: bool) -> Result<(), dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::set(&self, "org.freedesktop.NetworkManager.Device", "Autoconnect", value)
    }
}

#[derive(Debug)]
pub struct OrgFreedesktopNetworkManagerDeviceStateChanged {
    pub new_state: u32,
    pub old_state: u32,
    pub reason: u32,
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
        Ok(OrgFreedesktopNetworkManagerDeviceStateChanged {
            new_state: i.read()?,
            old_state: i.read()?,
            reason: i.read()?,
        })
    }
}

impl dbus::message::SignalArgs for OrgFreedesktopNetworkManagerDeviceStateChanged {
    const NAME: &'static str = "StateChanged";
    const INTERFACE: &'static str = "org.freedesktop.NetworkManager.Device";
}