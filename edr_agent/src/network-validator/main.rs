use std::process::Command;
use std::io::{self, Write};
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::net::IpAddr;
use std::str::FromStr;

#[derive(Debug)]
struct GeoIpEntry {
    network: String,
    country: String,
    region: String,
    city: String,
}

fn parse_ip_connection(line: &str) -> Option<String> {
    let parts: Vec<&str> = line.split_whitespace().collect();
    if parts.len() > 8 {
        if let Some(connection) = parts.get(8) {
            if connection.contains("->") {
                return Some(connection.to_string());
            }
        }
    }
    None
}

fn get_dst_ip(connection: &str) -> Option<String> {
    let parts: Vec<&str> = connection.split("->").collect();
    if parts.len() == 2 {
        let dst = parts[1];
        let ip_port: Vec<&str> = dst.split(':').collect();
        if ip_port.len() == 2 {
            return Some(ip_port[0].to_string());
        }
    }
    None
}

fn load_geoip_data(file_path: &str) -> io::Result<Vec<GeoIpEntry>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let mut entries = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let parts: Vec<&str> = line.split(',').collect();
        if parts.len() >= 4 {
            entries.push(GeoIpEntry {
                network: parts[0].to_string(),
                country: parts[1].to_string(),
                region: parts[2].to_string(),
                city: parts[3].to_string(),
            });
        }
    }
    Ok(entries)
}

fn ip_in_network(ip: &str, network: &str) -> bool {
    let ip = match IpAddr::from_str(ip) {
        Ok(ip) => ip,
        Err(_) => return false,
    };

    let (network_ip, mask) = match network.split_once('/') {
        Some((ip, mask)) => (ip, mask),
        None => return false,
    };

    let network_ip = match IpAddr::from_str(network_ip) {
        Ok(ip) => ip,
        Err(_) => return false,
    };

    let mask = match mask.parse::<u32>() {
        Ok(m) => m,
        Err(_) => return false,
    };

    match (ip, network_ip) {
        (IpAddr::V4(ip), IpAddr::V4(net)) => {
            let ip_int = u32::from_be_bytes(ip.octets());
            let net_int = u32::from_be_bytes(net.octets());
            let mask_int = !0u32 << (32 - mask);
            (ip_int & mask_int) == (net_int & mask_int)
        }
        (IpAddr::V6(_), IpAddr::V6(_)) => false, // IPv6 not implemented
        _ => false,
    }
}

fn find_geo_location(ip: &str, geoip_data: &[GeoIpEntry]) -> Option<String> {
    for entry in geoip_data {
        if ip_in_network(ip, &entry.network) {
            return Some(format!("{}/{}/{}", entry.country, entry.region, entry.city));
        }
    }
    None
}

fn main() -> io::Result<()> {
    // Load GeoIP data (assuming the CSV file is named "geoip.csv")
    let geoip_data = load_geoip_data("egress-ip-ranges.csv")?;

    // Create the command
    let output = Command::new("sudo")
        .arg("lsof")
        .arg("-i")
        .arg("-n")
        .arg("-P")
        .output()?;

    // Check if the command was successful
    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        let lines: Vec<&str> = stdout.lines().collect();

        // Print header
        println!("{:<20} {:<60} {:<30}", "Process", "Connection", "Geo Location");

        // Process each line
        for line in lines {
            if let Some(connection) = parse_ip_connection(line) {
                if let Some(dst_ip) = get_dst_ip(&connection) {
                    let geo = find_geo_location(&dst_ip, &geoip_data)
                        .unwrap_or_else(|| "Unknown".to_string());
                    println!("{:<20} {:<60} {:<30}", 
                        line.split_whitespace().next().unwrap_or(""),
                        connection,
                        geo);
                }
            }
        }
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        eprintln!("Command failed with error:\n{}", stderr);
    }

    Ok(())
}