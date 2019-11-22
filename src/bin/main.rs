use std::io::{Error, ErrorKind};
use std::process::Command;

use battrs::{draw, parse, PowerSource};

fn main() -> Result<(), Error> {
  let result = Command::new("pmset").arg("-g").arg("batt").output()?;
  let output = String::from_utf8(result.stdout).map_err(|e| Error::new(ErrorKind::Other, e))?;
  let source = parse(output);

  if let Some(PowerSource::Battery(amount)) = source {
    println!("source: {}", draw(amount));
  }

  Ok(())
}
