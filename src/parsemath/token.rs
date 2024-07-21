#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub enum Token {
	Add,
	Subtract,
	Multiply,
	Divide,
	Caret,
	LeftParen,
	RightParen,
	Num(f64),
	EOF
}

#[derive(Debug, PartialEq, PartialOrd)]
pub enum OperPrec {
	DefaultZero,
	AddSub,
	MulDiv,
	Power,
	Negative,
}

impl Token{
	pub fn get_oper_prec(&self) -> OperPrec {
		use self::OperPrec::*;
		use self::Token::*;
		match *self {
			Add | Subtract => AddSub,
			Multiply | Divide => MulDiv,
			Caret => Power,
			_ => DefaultZero,
		}
	}
}