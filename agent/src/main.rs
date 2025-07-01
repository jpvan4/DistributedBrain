// src/main.rs
use reqwest::Client;
use sysinfo::{System, SystemExt, CpuExt, Networks};
use std::{time::Duration};
use tokio::{time::sleep};
use serde::Serialize;
use log::{error, info};

#[derive(Serialize)]
struct SystemInfo {
    ip: String,
    hostname: String,
    cpu_cores: usize,
    total_memory: u64, // in bytes
    os: String,
    os_version: String,
}

async fn send_to_server(info: &SystemInfo) -> Result<(), reqwest::Error> {
    let client = Client::new();
    let response = client.post("https://yourcentralbrain.example.com/register")
        .json(info)
        .send()
        .await?;
        
    if !response.status().is_success() {
        error!("Server responded with status: {}", response.status());
    } else {
        info!("Data sent successfully.");
    }
    
    Ok(())
}

async fn gather_info() -> SystemInfo {
    let mut sys = System::new_all();
    sys.refresh_all();
    SystemInfo {
        ip: sys.networks().values()
            .filter_map(|iface| iface.ip_addr().to_string().parse::<std::net::IpAddr>().ok())
            .next()
            .map_or("unknown".to_string(), |ip| ip.to_string()),
        // Note: This is a simplified way to get the first IP address.
        hostname: sys.host_name().unwrap_or_default(),
        cpu_cores: sys.cpus().len(),
        total_memory: sys.total_memory(), // in bytes
        os: sys.name().unwrap_or("unknown".into()),
        os_version: sys.os_version().unwrap_or("unknown".into()),
    }
}

#[tokio::main]
async fn main() {
    env_logger::init(); // Initialize the logger
    loop {
        let info = gather_info().await;
        if let Err(e) = send_to_server(&info).await {
            eprintln!("Failed to send data: {}", e);
        }
        sleep(Duration::from_secs(60)).await; // heartbeat every 60s
    }
}
