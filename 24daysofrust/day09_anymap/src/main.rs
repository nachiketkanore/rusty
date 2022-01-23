use anymap::AnyMap;
use std::net::Ipv4Addr;

#[derive(Debug)]
enum HostAddress {
    DomainName(String),
    Ip(Ipv4Addr),
}

#[derive(Debug)]
struct Port(u32);

#[derive(Debug)]
struct ConnectionLimit(u32);

fn main() {
    let mut config = AnyMap::new();
    config.insert(HostAddress::DomainName("github.com".to_string()));
    config.insert(HostAddress::Ip(Ipv4Addr::new(127, 0, 0, 1)));
    // enum value gets overridden
    config.insert(Port(666));
    config.insert(ConnectionLimit(21));

    println!("{:?}", config.get::<HostAddress>());
    println!("{:?}", config.get::<Port>());
    println!("{:?}", config.get::<ConnectionLimit>());

    assert!(config.get::<String>().is_none());
    assert!(config.get::<u32>().is_none());
}
