struct Foo {
    a: i32,
    b: String
}


fn print_iter(a: &[i32]) {
    for el in a.iter(){
        print!("{} ", el);
    }
    println!();
}

fn main() {
    let alpha = 5;
    // Value and Reference print the same result probably because of this code:
    // https://github.com/rust-lang/rust/blob/1.18.0/src/libcore/fmt/mod.rs#L1470-L1485
    // which dereferences if it is a reference
    println!("{} {} {:p}", alpha, &alpha, &alpha);

    // pub struct String {
    //     vec: Vec<u8>,
    // }

    // pub struct Vec<T> {
    //     buf: RawVec<T>,
    //     len: usize,
    // }

    // pub struct RawVec<T, A: AllocRef = Global> {
    //     ptr: Unique<T>,
    //     cap: usize,
    //     alloc: A,

    // pub struct Unique<T: ?Sized> {
    //     pointer: *const T,
    //     // NOTE: this marker has no consequences for variance, but is necessary
    //     // for dropck to understand that we logically own a `T`.
    //     //
    //     // For details, see:
    //     // https://github.com/rust-lang/rfcs/blob/master/text/0769-sound-generic-drop.md#phantom-data
    //     _marker: PhantomData<T>,
    // }

    let beta = "leet".to_string();
    // &beta returns pointer to the String struct which is not where the actual String is stored
    println!("{} {} {:p}", beta, &beta, &beta);

    let gamma = [5,10,15];
    print_iter(&gamma);
    println!("{:p}", &&gamma);

    let beta = Foo{a: 100, b: "1701".to_string()};
    println!("{:p} {:?}", &beta, (&beta) as (*const Foo));
}
