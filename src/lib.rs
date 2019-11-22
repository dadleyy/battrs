mod tokens;

#[derive(Debug, PartialEq)]
pub enum PowerSource {
  Battery(u8),
  Other(String),
}

fn parse_life(input: &str) -> Option<u8> {
  input
    .split(';')
    .nth(0)
    .and_then(|v| v.split_whitespace().nth(2))
    .and_then(|v| v.trim_end_matches('%').parse::<u8>().ok())
}

pub fn parse<T>(input: T) -> Option<PowerSource>
where
  T: std::convert::AsRef<str>,
{
  let mut lines = input.as_ref().lines();
  let (first, second) = (lines.next(), lines.next());

  second.and_then(parse_life).and_then(|amount| {
    first.and_then(|line| {
      line.splitn(3, '\'').nth(1).map(|v| match v {
        "Battery Power" => PowerSource::Battery(amount),
        other => PowerSource::Other(String::from(other)),
      })
    })
  })
}

pub fn draw(amount: u8) -> char {
  match amount {
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
  }
}

#[cfg(test)]
mod tests {
  use super::{parse, PowerSource};

  #[test]
  fn none_when_unable_to_parse() {
    let result = parse("whoa");
    assert!(result.is_none());
  }

  #[test]
  fn battery_with_amount_100() {
    let result = parse(
      r#"Now drawing from 'Battery Power'
-InternalBattery-0 (id=4522083)	100%; charged; 0:00 remaining present: true
    "#,
    );
    assert_eq!(result, Some(PowerSource::Battery(100)))
  }

  #[test]
  fn none_invalid_amount() {
    let result = parse(
      r#"Now drawing from 'Battery Power'
-InternalBattery-0 (id=4522083)	abc%; charged; 0:00 remaining present: true
    "#,
    );
    assert!(result.is_none());
  }

  #[test]
  fn battery_with_amount_10() {
    let result = parse(
      r#"Now drawing from 'Battery Power'
-InternalBattery-0 (id=4522083)	10%; charged; 0:00 remaining present: true
    "#,
    );
    assert_eq!(result, Some(PowerSource::Battery(10)))
  }

  #[test]
  fn battery_with_amount_1() {
    let result = parse(
      r#"Now drawing from 'Battery Power'
-InternalBattery-0 (id=4522083)	1%; charged; 0:00 remaining present: true
    "#,
    );
    assert_eq!(result, Some(PowerSource::Battery(1)))
  }

  #[test]
  fn other_with_name() {
    let result = parse(
      r#"Now drawing from 'AC Power'
-InternalBattery-0 (id=4522083)	100%; charged; 0:00 remaining present: true
    "#,
    );
    assert_eq!(result, Some(PowerSource::Other(String::from("AC Power"))))
  }
}
