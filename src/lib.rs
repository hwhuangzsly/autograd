pub mod bean;

use std::collections::HashMap;
use crate::bean::expression::Expression;
use std::rc::Rc;
use std::ops::Add;
use std::cmp::Ordering;

#[derive(Eq, Debug)]
pub enum Operate {
	MINUS,
    ADD,
    SUB,
    MUL,
    DIV,
}

impl Operate {
	pub fn get_priority(&self) -> i32 {
		match self {
			Operate::MINUS => 3,
			Operate::MUL => 2,
			Operate::DIV => 2,
			Operate::ADD => 1,
			Operate::SUB => 1
		}
	}
}

impl Ord for Operate {
    fn cmp(&self, other: &Self) -> Ordering {
        self.get_priority().cmp(&other.get_priority())
    }
}

impl PartialOrd for Operate {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Operate {
    fn eq(&self, other: &Self) -> bool {
        self == other
    }
}

pub type gradable = Rc<Box<dyn Gradable>>;
pub type exp = Rc<Box<dyn Exp>>;

pub fn add(x: gradable, y: gradable) -> gradable {
	Rc::new(Box::new(Expression {
		operate: Operate::ADD,
		params: vec![Rc::clone(&x), Rc::clone(&y)],
	}))
}

pub fn mul(x: gradable, y: gradable) -> gradable {
	Rc::new(Box::new(Expression {
		operate: Operate::MUL,
		params: vec![Rc::clone(&x), Rc::clone(&y)],
	}))
}

pub trait Gradable: Exp {
	fn grad(&self, name: &str) -> gradable;
}

// impl Add for gradable {
// 	type Output = Self;

// 	fn add(self, other: Self) -> Self {
// 		Rc::new(Box::new(Expression {
// 			operate: Operate::ADD,
// 			params: vec![Rc::clone(&self), Rc::clone(&other)],
// 		}))
// 	}
// }

pub trait Exp {
	fn value(&self, param_value: &HashMap<String, f64>) -> f64;
}
// pub enum Exp {
// 	E(Expression),
// 	C(Constant),
// 	V(Variable),
// }

// impl Gradable for Exp {
// 	fn value(&self, param_value: &HashMap<String, f64>) -> f64 {
// 		match self {
// 			Exp::E(expression) => expression.value(param_value),
// 			Exp::C(constant) => constant.value(param_value),
// 			Exp::V(variable) => variable.value(param_value),
// 		}
// 	}

// 	fn grad(&self, name: &str) -> Rc<Exp> {
// 		match self {
// 			Exp::E(expression) => expression.grad(name),
// 			Exp::C(constant) => constant.grad(name),
// 			Exp::V(variable) => variable.grad(name),
// 		}
// 	}
// }