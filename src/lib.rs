pub mod bean;

use std::collections::HashMap;
use crate::bean::expression::Expression;
use std::rc::Rc;

pub enum Operate {
	MINUS,
    ADD,
    SUB,
    MUL,
    DIV,
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