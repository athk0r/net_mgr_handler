#[derive(Clone, Debug, PartialEq)]
pub enum DeviceType {
    Unknown,
    Ethernet,
    WiFi,
    Unused1,
    Unused2,
    Bt,
    OlpcMesh,
    Wimax,
    Modem,
    Infiniband,
    Bond,
    Vlan,
    Adsl,
    Bridge,
    Generic,
    Team,
    Tun,
    IpTunnel,
    Macvlan,
    Vxlan,
    Veth,
    Macsec,
    Dummy,
}

impl From<u32> for DeviceType {
    fn from(device_type: u32) -> Self {
        match device_type {
            0 => DeviceType::Unknown,
            1 => DeviceType::Ethernet,
            2 => DeviceType::WiFi,
            3 => DeviceType::Unused1,
            4 => DeviceType::Unused2,
            5 => DeviceType::Bt,
            6 => DeviceType::OlpcMesh,
            7 => DeviceType::Wimax,
            8 => DeviceType::Modem,
            9 => DeviceType::Infiniband,
            10 => DeviceType::Bond,
            11 => DeviceType::Vlan,
            12 => DeviceType::Adsl,
            13 => DeviceType::Bridge,
            14 => DeviceType::Generic,
            15 => DeviceType::Team,
            16 => DeviceType::Tun,
            17 => DeviceType::IpTunnel,
            18 => DeviceType::Macvlan,
            19 => DeviceType::Vxlan,
            20 => DeviceType::Veth,
            21 => DeviceType::Macsec,
            22 => DeviceType::Dummy,
            _ => {
                //warn!("Undefined device type: {}", device_type);
                DeviceType::Unknown
            },
        }
    }
}