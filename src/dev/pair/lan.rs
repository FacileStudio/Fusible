//! Finds the private LAN address the studio shares on.

use std::net::{IpAddr, Ipv4Addr, UdpSocket};

const REFUSED: &str = "no private LAN address found; --share only works on a local network";

/// Whether `ip` is a private IPv4 address (10/8, 172.16/12, 192.168/16).
pub(super) fn is_private(ip: IpAddr) -> bool {
    matches!(ip, IpAddr::V4(v4) if v4.is_private())
}

/// The local address the OS would use to reach the network; no packet is sent.
///
/// Fails when that address is not private, so `--share` never opens a public interface.
pub(in crate::dev) fn lan_ip() -> Result<Ipv4Addr, String> {
    let socket = UdpSocket::bind("0.0.0.0:0").map_err(|_| REFUSED)?;
    socket.connect("192.0.2.1:80").map_err(|_| REFUSED)?;
    match socket.local_addr().map(|addr| addr.ip()) {
        Ok(IpAddr::V4(ip)) if is_private(IpAddr::V4(ip)) => Ok(ip),
        _ => Err(REFUSED.into()),
    }
}
