use std::fmt::{Display, Error as FormatError, Formatter};
use std::io::{Error, ErrorKind};
use std::process::Command;

mod tokens;

#[derive(Debug, PartialEq)]
pub enum PowerSource {
  Battery(u8),
  Other(String),
  Unknown,
}

fn parse_life(input: &str) -> Option<u8> {
  input
    .split(';')
    .nth(0)
    .and_then(|v| v.split_whitespace().nth(2))
    .and_then(|v| v.trim_end_matches('%').parse::<u8>().ok())
}

impl<T> std::convert::From<T> for PowerSource
where
  T: std::convert::AsRef<str>,
{
  fn from(input: T) -> Self {
    let mut lines = input.as_ref().lines();
    let (first, second) = (lines.next(), lines.next());

    second
      .and_then(parse_life)
      .and_then(|amount| {
        first.and_then(|line| {
          line.splitn(3, '\'').nth(1).map(|v| match v {
            "Battery Power" => PowerSource::Battery(amount),
            other => PowerSource::Other(String::from(other)),
          })
        })
      })
      .unwrap_or(PowerSource::Unknown)
  }
}

impl Display for PowerSource {
  fn fmt(&self, formatter: &mut Formatter) -> Result<(), FormatError> {
    if let PowerSource::Battery(amount) = self {
      let token = match amount {
        0..=10 => tokens::TEN,
        11..=20 => tokens::TWENTY,
        21..=30 => tokens::THIRTY,
        31..=40 => tokens::FOURTY,
        41..=50 => tokens::FIFTY,
        51..=60 => tokens::SIXTY,
        61..=70 => tokens::SEVENTY,
        71..=80 => tokens::EIGHTY,
        81..=90 => tokens::NINETY,
        _ => tokens::HUNDRED,
      };
      return write!(formatter, "{}", token);
    }
    Ok(())
  }
}

pub fn measure() -> Result<PowerSource, Error> {
  let result = Command::new("pmset").arg("-g").arg("batt").output()?;
  let output = String::from_utf8(result.stdout).map_err(|e| Error::new(ErrorKind::Other, e))?;
  Ok(PowerSource::from(output))
}

#[cfg(test)]
mod tests {
  use super::{tokens, PowerSource};

  #[test]
  fn test_draw_canon() {
    let result = PowerSource::Battery(10);
    let out = format!("{}", result);
    assert_eq!(out, format!("{}", tokens::TEN));
  }

  #[test]
  fn test_draw_debug() {
    let result = PowerSource::Battery(10);
    let out = format!("{:?}", result);
    assert_eq!(out, String::from("Battery(10)"));
  }

  #[test]
  fn none_when_unable_to_parse() {
    let result = PowerSource::from("whoa");
    assert_eq!(result, PowerSource::Unknown);
  }

  #[test]
  fn battery_with_amount_100() {
    let result = PowerSource::from(
      r#"Now drawing from 'Battery Power'
-InternalBattery-0 (id=4522083)	100%; charged; 0:00 remaining present: true
    "#,
    );
    assert_eq!(result, PowerSource::Battery(100));
  }

  #[test]
  fn none_invalid_amount() {
    let result = PowerSource::from(
      r#"Now drawing from 'Battery Power'
-InternalBattery-0 (id=4522083)	abc%; charged; 0:00 remaining present: true
    "#,
    );
    assert_eq!(result, PowerSource::Unknown);
  }

  #[test]
  fn battery_with_amount_10() {
    let result = PowerSource::from(
      r#"Now drawing from 'Battery Power'
-InternalBattery-0 (id=4522083)	10%; charged; 0:00 remaining present: true
    "#,
    );
    assert_eq!(result, PowerSource::Battery(10));
  }

  #[test]
  fn battery_with_amount_1() {
    let result = PowerSource::from(
      r#"Now drawing from 'Battery Power'
-InternalBattery-0 (id=4522083)	1%; charged; 0:00 remaining present: true
    "#,
    );
    assert_eq!(result, PowerSource::Battery(1));
  }

  #[test]
  fn other_with_name() {
    let result = PowerSource::from(
      r#"Now drawing from 'AC Power'
-InternalBattery-0 (id=4522083)	100%; charged; 0:00 remaining present: true
    "#,
    );
    assert_eq!(result, PowerSource::Other(String::from("AC Power")));
  }
}
