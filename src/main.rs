use gethostname::gethostname;
use std::process::Command;
use std::str;
use serde::{Serialize, Deserialize};
use serde_json::{Result};

#[derive(Serialize, Deserialize, Debug)]
struct IpAddrInterface {
    address: String,
    broadcast: String,
    flags: Vec<String>,
    link_type: String,
    operstate: String,
    addr_info: Vec<AddrInfo>,
}

#[derive(Serialize, Deserialize, Debug)]
struct AddrInfo {
    broadcast: Option<String>,
    dynamic: Option<bool>,
    family: String,
    label: Option<String>,
    local: String,
    noprefixroute: Option<bool>,
    preferred_life_time: i64,
    prefixlen: i64,
    scope: String,
    valid_life_time: i64,
}

fn get_ip_info() -> Result<Vec<IpAddrInterface>> {
    let output = Command::new("ip").args(&["-j", "address"]).output().expect("Failed to execute process");
    let result = str::from_utf8(&output.stdout).expect("Failed to convert to string");
    let data: Vec<IpAddrInterface> = serde_json::from_str(result).expect("Failed to parse");
    return Ok(data)
}

fn main() {
    let hostname = gethostname();
    let hostname = hostname.to_str().unwrap_or("");
    println!("Hostname: {}", hostname);

    let interfaces = get_ip_info().expect("Failed to find interfaces");

    for interface in interfaces {
        println!("Link type: {} [{}]", interface.link_type, interface.operstate);
        for addr_info in interface.addr_info {
            println!("> {}", addr_info.local);
        }
    }
}
