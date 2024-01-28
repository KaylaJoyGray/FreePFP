use std::fmt;
#[derive (Debug)]
pub enum Unit {
	USD,
	GBP
}

impl fmt::Display for Unit {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Unit::USD => write!(f, "$"),
			Unit::GBP => write!(f, "£")
		}
	}
}

pub struct Amount {
	pub symbol: Unit,
	pub value: i32,
}
impl fmt::Debug for Amount {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "Amount {{ symbol: {:?}, value: {} }}", self.symbol, self.value)
	}
}

impl fmt::Display for Amount {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		if self.value > 0 {
			let div = if self.value % 100 != 0 { "." } else { ".0" };
			write!(f, "{}{}{}{}", self.symbol, self.value / 100, div, self.value % 100)
		} else {
			let div = if self.value.abs() % 100 != 0 { "." } else { ".0" };
			write!(f, "-{}{}{}{}", self.symbol, self.value.abs() / 100, div, self.value.abs() % 100)
		}
	}
}

impl Amount {
		pub fn new(symbol: Unit, value: i32) -> Self {
			Amount { symbol, value }
		}
}

#[cfg(test)]
mod tests {
	use super::*;
	#[test]
	fn test_amount_format() {

		let x = Amount::new(Unit::USD, 10000);
		assert_eq!("$100.00", format!("{}",x));

		let x = Amount::new(Unit::USD, -10000);
		assert_eq!("-$100.00", format!("{}",x));

		let x = Amount::new(Unit::USD, 123456);
		assert_eq!("$1234.56", format!("{}",x));

		let x = Amount::new(Unit::USD, -123456);
		assert_eq!("-$1234.56", format!("{}",x));
	}
}