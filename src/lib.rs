#![feature(conservative_impl_trait)]

pub enum Error {
    Invalid,
}

pub type Result = std::result::Result<(), Error>;

/// ```
/// extern crate errr;
///
/// use errr::{runner, run};
///
/// fn main() {
///     let _ = runner(&"test test".to_string(), &run(5));
/// }
/// ```
pub fn runner<T>(subject: &T, to_run: &Fn(&T) -> Result) -> Result {
    if let Err(error) = to_run(subject) {
        Err(error)
    } else {
        Ok(())
    }
}

pub fn run(min: usize) -> impl Fn(&String) -> Result {
    move |s: &String| {
        if s.len() >= min {
            Ok(())
        } else {
            println!("test test");
            Err(Error::Invalid)
        }
    }
}

fn main() {
    let _ = runner(&"test test".to_string(), &run(5));
    println!("TESTING YEAH");
}
