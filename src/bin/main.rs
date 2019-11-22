use std::io::Error;

use battrs::{draw, measure};

fn main() -> Result<(), Error> {
  let measurement = measure()?;

  if let Some(token) = draw(measurement) {
    println!("{}", token);
  }

  Ok(())
}
