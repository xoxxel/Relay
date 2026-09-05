use mdns_sd::{ServiceDaemon, ServiceInfo};
use std::collections::HashMap;

pub struct MdnsBroadcaster {
    daemon: ServiceDaemon,
    fullname: String,
}

impl MdnsBroadcaster {
    pub fn start(port: u16, host_name: &str) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        let daemon = ServiceDaemon::new()?;
        let service_type = "_http._tcp.local.";
        let instance_name = "Relay";
        let host_name_full = format!("{}.", host_name.trim_end_matches('.'));
        let mut properties = HashMap::new();
        properties.insert("app".to_string(), "relay".to_string());
        properties.insert("version".to_string(), "0.1.0".to_string());

        let service_info = ServiceInfo::new(
            service_type,
            instance_name,
            &host_name_full,
            "",
            port,
            properties,
        )?;

        let fullname = service_info.get_fullname().to_string();
        daemon.register(service_info)?;
        println!("mDNS service registered as http://{}:{}", host_name, port);

        Ok(Self { daemon, fullname })
    }

    pub fn stop(&self) {
        let _ = self.daemon.unregister(&self.fullname);
        let _ = self.daemon.shutdown();
        println!("mDNS service stopped");
    }
}

