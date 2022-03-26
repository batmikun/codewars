use std::{net::Ipv4Addr, str::FromStr};

fn main() {
    let result = ips_between("10.0.0.0", "10.0.0.50");

    println!("{}", result);
}

fn ips_between(start: &str, end: &str) -> u32 {
    let ip_number_one = u32::from_be_bytes(Ipv4Addr::from_str(start).unwrap().octets());
    let ip_number_two = u32::from_be_bytes(Ipv4Addr::from_str(end).unwrap().octets());

    return ip_number_two - ip_number_one;
}
