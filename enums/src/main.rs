// "enums encode meaning with data"
#[allow(unused_variables)]

enum IpAddrKind {
    V4,
    V6,
}

fn main() {}

#[allow(dead_code)]
fn simple_enum() {
    fn route(_ip_kind: IpAddrKind) {}

    let _four = IpAddrKind::V4;
    let _six = IpAddrKind::V6;

    route(IpAddrKind::V4);
    route(IpAddrKind::V6);
}

// problem with structs:
#[allow(dead_code)]
fn problem_with_structs() {
    // the below implementation is not very concise:
    struct IpAddr {
        kind: IpAddrKind,
        address: String,
    }

    // two instances of the struct IpAddr are created:
    let _home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let _loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };
}

#[allow(dead_code)]
fn more_concise_data_direct_enum() {
    // rather than an enum inside a struct, put the data directly into each enum variant
    // this definition of IpAddr says both V4 and V6 will have variants that are string vals
    enum IpAddr {
        V4(String),
        V6(String),
    }

    let _home = IpAddr::V4(String::from("127.0.0.1"));
    let _loopback = IpAddr::V6(String::from("::1"));

    // another advantage to using an enum is that we could have different types
    // associated with the data:
    enum IpAddr2 {
        V4(u8, u8, u8, u8),
        V6(String),
    }

    let _home2 = IpAddr2::V4(128, 0, 0, 1);
    let _loopback = IpAddr2::V6(String::from("::1"));
}

// the standar library has a definition for IpAddr too:
#[allow(dead_code)]
struct Ipv4Addr {
    //
}

#[allow(dead_code)]
struct Ipv6Addr {
    //
}

// you can put any kind of data inside an enum variant: strings, numeric types, or structs, for example
#[allow(dead_code)]
enum IpAddr {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}

#[allow(dead_code)]
fn enums_vs_structs() {
    // the difference between a struct and an enum like the one below is:
    // enum doesn’t use the struct keyword and all the variants are grouped together under the Message type.
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    // here is an equivalent number of structs to create the above:
    struct QuitMessage; // unit struct
    struct MoveMessage {
        x: i32,
        y: i32,
    }
    struct WriteMessage(String); // tuple struct
    struct ChangeColorMessage(i32, i32, i32); // tuple struct

    // the impl feature also works with enums:
    impl Message {
        fn call(&self) {
            // method body would be defined here
        }
    }

    let m = Message::Write(String::from("hello"));
    m.call();
}

#[allow(dead_code)]
fn option_enum_no_nulls() {
    // in Rust there is no Null value. Null often leads to problems
    // see explanation here - https://www.infoq.com/presentations/Null-References-The-Billion-Dollar-Mistake-Tony-Hoare/
    // this enum is provided by the standard library:
    // enum Option<T> {
    //     None,
    //     Some(T),
    // }

    let _some_number = Some(5);
    let _some_char = Some('e');

    let _absent_number: Option<i32> = None;

    // when a function or variable specifies its return type as  Option<usize>, it means that the function can potentially return either Some(index) (where index is of type usize) or None.
    fn find_number(numbers: &[i32], target: i32) -> Option<usize> {
        for (index, &number) in numbers.iter().enumerate() {
            if number == target {
                // Some is returned if the number exists
                // wrapping it in Some indicates that the value is not invalid or null
                return Some(index);
            }
        }
        // None is returned if the number doesn't exist
        None
    }
}
