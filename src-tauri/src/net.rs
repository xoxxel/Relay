use std::net::IpAddr;

fn default_route_interface() -> Option<String> {
    let content = std::fs::read_to_string("/proc/net/route").ok()?;
    for line in content.lines().skip(1) {
        let mut parts = line.split_whitespace();
        let iface = parts.next()?;
        let dest = parts.next()?;
        if dest == "00000000" {
            return Some(iface.to_string());
        }
    }
    None
}

fn is_virtual_interface(name: &str) -> bool {
    let n = name.to_lowercase();
    n.contains("tun")
        || n.contains("tap")
        || n.contains("docker")
        || n.contains("virbr")
        || n.contains("veth")
        || n.contains("br-")
        || n.contains("tailscale")
        || n.contains("zerotier")
        || n.contains("wg")
}

fn is_private_ip(ip: &IpAddr) -> bool {
    match ip {
        IpAddr::V4(v4) => v4.is_private(),
        IpAddr::V6(v6) => v6.is_loopback() || v6.is_unique_local(),
    }
}

pub fn detect_lan_ip() -> Option<String> {
    let preferred = default_route_interface();
    let mut fallback: Option<String> = None;

    if let Ok(interfaces) = local_ip_address::list_afinet_netifas() {
        for (name, ip) in interfaces {
            if ip.is_loopback() || !is_private_ip(&ip) {
                continue;
            }
            if is_virtual_interface(&name) {
                continue;
            }

            let is_preferred = preferred.as_deref() == Some(name.as_str());
            if is_preferred {
                return Some(ip.to_string());
            }
            if fallback.is_none() {
                fallback.replace(ip.to_string());
            }
        }
    }

    fallback
}

pub fn format_url(ip: &str, port: u16) -> String {
    format!("http://{}:{}", ip, port)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_lan_ip_is_private_and_reachable() {
        if let Some(ip_str) = detect_lan_ip() {
            let ip: IpAddr = ip_str.parse().expect("detected IP must parse");
            assert!(is_private_ip(&ip), "detected IP should be private, got {}", ip);
            assert!(!ip.is_loopback(), "detected IP should not be loopback");
        }
    }
}