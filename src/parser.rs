use autograd::Operate;
use autograd::gradable;
use autograd::bean::variable::Variable;
use autograd::bean::constant::Constant;
use autograd::bean::expression::Expression;
use std::slice::Iter;
use std::rc::Rc;

pub enum Token {
	OperateToken(Operate),
	ValuesToken(gradable)
}

impl Token {
	pub fn get_next_token(string: &str,) -> Option<(Token, &str)> {
		let mut iter = string.chars();
		let next = iter.next();
		let mut is_digit = false;
		let len = string.len();
		if let Some(c) = next {
			match c {
				'+' => return Some((Token::OperateToken(Operate::ADD), &string[1..len])),
				'-' => return Some((Token::OperateToken(Operate::SUB), &string[1..len])),
				'*' => return Some((Token::OperateToken(Operate::MUL), &string[1..len])),
				'/' => return Some((Token::OperateToken(Operate::DIV), &string[1..len])),
				c if c.is_ascii_alphabetic() => is_digit = false,
				_ => is_digit = true
			}
		} else {
			return None;
		}
		let mut end = 1;
        while let Some(c) = iter.next() {
        	match c {
	        	c if c.is_ascii_alphabetic() || c.is_ascii_digit() => {
	        		end += 1;
	        	},
	        	_ => break
        	}
        }
        // println!("tokenString:{}", &string[0..end]);
        if !is_digit {
        	Some((Token::ValuesToken(Rc::new(Box::new(Variable{name: String::from(&string[0..end])}))), &string[end..len]))
        } else {
        	Some((Token::ValuesToken(Rc::new(Box::new(Constant{value: string[0..end].parse().unwrap()}))), &string[end..len]))
        }
    }
}

pub fn parse (string: &str) -> gradable {
	fn match_operate(operates: &mut Vec<Operate>, expressions: &mut Vec<gradable>, operate: Operate) {
		if operates.is_empty() {
			operates.push(operate);
		} else {
			if operate > operates[operates.len() - 1] {
				operates.push(operate);
			} else {
				let right = expressions.pop().unwrap();
				let left = expressions.pop().unwrap();
				let op = operates.pop().unwrap();
				expressions.push(Rc::new(Box::new(Expression {
					operate: op,
					params: vec![Rc::clone(&left), Rc::clone(&right)],
				})));
				match_operate(operates, expressions, operate);
			}
		}
	}
	let mut operates = Vec::new();
	let mut expressions = Vec::new(); 
	let mut left_string = string;
	while let Some((token, left)) = Token::get_next_token(left_string) {
		match token {
			Token::OperateToken(operate) => {
				match_operate(&mut operates, &mut expressions, operate);
			},
			Token::ValuesToken(gradable) => {
				expressions.push(gradable);
			}
		}
		// println!("left:{}", left);
		left_string = left;
	}
	while !operates.is_empty() {
		let right = expressions.pop().unwrap();
		let left = expressions.pop().unwrap();
		let op = operates.pop().unwrap();
		expressions.push(Rc::new(Box::new(Expression {
			operate: op,
			params: vec![Rc::clone(&left), Rc::clone(&right)],
		})));
	}
	expressions.pop().unwrap()
}