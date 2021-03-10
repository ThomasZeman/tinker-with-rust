use std::fmt::{Display, Formatter, Result};

#[derive(Debug)]
struct Alpha {
    a: i32,
    b: i64,
    c: String
}

struct Beta {
    a: i32,
    b: i64,
    c: String
}

// Tuple structs
struct Gamma (i32, i32, i32);

// Use case: Different units for temperature
struct Celsius (f32);

struct Fahrenheit (f32);

struct Noop();

// https://doc.rust-lang.org/std/fmt/struct.Formatter.html
impl Display for Beta {
    // Result is fmt::Result
    // pub type Result = result::Result<(), Error>;
    fn fmt(&self, f: &mut Formatter) -> Result {
        // https://doc.rust-lang.org/std/macro.format_args.html
        // format_args! produces a value of type fmt::Arguments.
        // This value can be passed to the macros within std::fmt for performing useful redirection.
        // format_args!, unlike its derived macros, avoids heap allocations.
        f.write_fmt(format_args!("{},{},{}", self.a, self.b, self.c))
    }
}

fn build_beta() -> Beta {
    Beta {a: 20, b: 30, c: "50".to_string()}
}

fn build_beta2() -> Beta {
    return build_beta()
}

fn structs() {
    let alpha = Alpha{a: 3,b: 2, c: "centauri".to_string()};
    println!("{:?}", alpha); // Alpha { a: 3, b: 2, c: "centauri" }

    let beta = Beta{a: 10, b: 5, c: String::from("ncc1701d")};
    println!("{}", beta); // 10,5,ncc1701d
    println!("{}", build_beta2());
    println!("{}", Beta{..build_beta2()});

    // ".." does not work like the spread operator in JS
    // Must be last and only assigns unassigned fields
    println!("{}", Beta{a: 333, ..build_beta2()});
}

fn tuple_structs() -> Noop {
    // Destructure named tuple struct
    let gamma = Gamma(5, 6, 7);
    let Gamma(a,b,c) = gamma;
    println!("{}, {}, {}", a, b, c);

    // Destructure anonymous tuple
    let tuple1 = (50,60);
    let (x,y) = tuple1;
    println!("{}, {}", x, y);

    // As units
    let temp1 = Celsius(10.0);
    let temp2 = Fahrenheit(32.4);
    println!("{}, {}", temp1.0, temp2.0);

    return Noop();
}

fn main() {
    structs();
    tuple_structs();
}
