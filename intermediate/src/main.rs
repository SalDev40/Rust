mod collections;
use collections::sub1;

#[derive(Debug)]
struct User {
    email: String,
    password: String,
    valid: bool,
}

impl User {
    fn new(email: String, password: String, valid: bool) -> User {
        return User {
            email,
            password,
            valid,
        };
    }

    fn getEmail(&self) -> &String {
        return &self.email;
    }

    fn getPassword(&self) -> &String {
        return &self.password;
    }

    fn getIsValid(&self) -> bool {
        return self.valid;
    }
}
#[derive(Debug)]
struct IpDetails {
    address: String,
    kind: String,
}

impl IpDetails {
    fn new(address: String, kind: String) -> IpDetails {
        return IpDetails { address, kind };
    }
    fn getAddress(&self) -> &String {
        return &self.address;
    }
}

enum IpAddr {
    v4(IpDetails),
    v6(IpDetails),
}

impl IpAddr {
    fn new(version: IpDetails) -> IpAddr {
        if version.kind == "v4" {
            return IpAddr::v4(version);
        } else {
            return IpAddr::v6(version);
        }
    }
}

fn plusOne(x: Option<i32>) -> Option<i32> {
    match x {
        Some(i) => return Some(i + 1),
        None => return None,
    }
}

fn optString(x: Option<String>) -> Option<String> {
    match x {
        Some(i) => return Some(i),
        None => return None,
    }
}
fn printIpAddress(ip: &IpAddr) {
    match ip {
        IpAddr::v4(add) => println!("{:?} ", add),
        IpAddr::v6(add) => println!("{:?} ", add),
    }
}
fn main() {
    let john: User = User::new("john@gmail".to_string(), "pass".to_string(), true);
    // println!("{:?}", john);

    let ipAddress: IpAddr = IpAddr::new(IpDetails::new("127.0.0.1".to_string(), "v4".to_string()));


    printIpAddress(&ipAddress);



    // let opt: Option<i32> = Some(13);

    // println!("{:?},", plusOne(opt));

    // let mut optOwn: Option<String> = Some(String::from("hello options"));
    // optOwn = optString(optOwn);
    // // println!("{:?},", optString(optOwn));
    // println!("{:?},", optOwn);

    // let some_u8_value = Some(0u8);
    // if let Some(3) = some_u8_value {
    //     println!("three");
    // }

    sub1::hash();

    let hello: String = String::from("hello");
    let helloRef: &String = &hello;
    let world: &str = &hello[..];
}
