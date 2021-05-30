use std::io::stdin;
use std::error::Error;

// read:
// https://users.rust-lang.org/t/help-understanding-return-for-box-dyn-error/33748/2

type Result2<T> = std::result::Result<T, Box<dyn Error>>;

fn main() -> Result2<()> {
    let mut a: String = Default::default();
    stdin().read_line(&mut a)?;
    print!("{}", a);
    Ok(())
}
