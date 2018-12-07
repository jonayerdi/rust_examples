use std::io;
use std::fmt;

type IpV4Addr = [u8; 4];
type IpV6Addr = [u8; 16];

#[derive(Debug)]
enum IpAddr {
    V4(IpV4Addr),
    V6(IpV6Addr),
}

impl fmt::Display for IpAddr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let IpAddr::V4(ipv4) = self {
            write!(f, "{}.{}.{}.{}", ipv4[0], ipv4[1], ipv4[2], ipv4[3])
        } else if let IpAddr::V6(ipv6) = self {
            for i in 0..15 {
                if let Err(e) = write!(f, "{}:", ipv6[i]) {
                    return Err(e);
                }
            }
            write!(f, "{}", ipv6[15])
        } else {
             panic!("An error occurred");
        }
    }
}

fn main() -> io::Result<()> {
    let ipv4 = IpAddr::V4([127,0,0,1]);
    let ipv6 = IpAddr::V6([0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1]);
    println!("{}", ipv4);
    println!("{:?}", ipv4);
    println!("{}", ipv6);
    println!("{:?}", ipv6);
    Ok(())
}
