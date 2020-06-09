use std::process::Command;

fn main() {
    let current_dns = Command::new("networksetup").args(&["-getdnsservers", "Wi-Fi"]).output().expect("Failed to get current DNS settings");
    let pihole_dns = [49, 57, 50, 46, 49, 54, 56, 46, 56, 54, 46, 49, 54, 48, 10];
    let dns_ip = String::from("192.168.86.160");
    let empty_dns_ip = String::from("Empty");

    let ip = if current_dns.stdout == pihole_dns {
        empty_dns_ip
    }
    else {
        dns_ip
    };

    set_dns(ip)
}

fn set_dns(ip: String) {
    println!("Setting DNS to: {}", &ip);
    Command::new("networksetup").args(&["-setdnsservers", "Wi-Fi", &ip]).output().expect("Failed to set DNS");
}

