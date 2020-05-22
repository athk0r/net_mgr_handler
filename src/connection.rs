pub struct Connection {
    con_type: ConnectionType,
    security: Security
}

pub enum ConnectionType {
    Wifi,
}

pub enum Security {
    WifiSecurity,
}