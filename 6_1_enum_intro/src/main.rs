use std::fmt;

#[derive(Debug)]
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

impl fmt::Display for IpAddr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &IpAddr::V4(a, b, c, d) => write!(f, "{}.{}.{}.{}", a, b, c, d),
            &IpAddr::V6(ref a) => write!(f, "{}", a)
        }
    }
}

fn main() {
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));

    println!("v4: {ip}", ip=home);
    println!("v6: {ip}", ip=loopback);
}
