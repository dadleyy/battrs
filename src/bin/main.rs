use std::io::Error;

use battrs::measure;

fn main() -> Result<(), Error> {
  let measurement = measure()?;
  println!("{}", measurement);
  Ok(())
}
