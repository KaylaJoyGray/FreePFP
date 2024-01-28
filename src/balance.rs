use time;
use crate::currency;

#[derive (Debug)]
pub struct Transaction {
	amount: currency::Amount,
	category: Option<String>,
	date: time::PrimitiveDateTime
}

pub fn running_balance(current: i32, history: Vec<Transaction>) -> i32 {
	history.iter().fold(current, |balance, entry| balance + entry.amount.value)
}

impl Transaction {
	pub fn new(amount: currency::Amount, category: Option<String>, date: time::PrimitiveDateTime) -> Self {
		Transaction {
			amount,
			category,
			date,
		}
	}
}