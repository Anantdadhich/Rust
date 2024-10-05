//Where structs give you a way of grouping together related fields and data,
// like a Rectangle with its width and height,
// enums give you a way of saying a value is one of a possible set of values
enum Var {
    i4,
    i6,
}
fn main() {
    let t = Var::i4; //we create the instance

    enum IpkindP {
        ip4,
        ip6,
    }

    struct IpKind {
        kind: IpkindP,
        address: String,
    }

    let pc = IpKind {
        kind: IpkindP::ip4,
        address: String::from("2323.234.32"),
    };

    //we define the enum without the need of struct
    //  let nostruct = IpkindP::ip6(String::from("34.3434.3"));
    //   let nostruct1 = IpkindP::ip4(String::from("..34"));

    enum Drgrg {
        v4(u8, u8),
    }
    let tte = Drgrg::v4(23, 34);

    println!("Hello, world!");
}
enum Coin {
    pnny,
    canad,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::pnny => 1,
        Coin::canad => 2,
    }
}
