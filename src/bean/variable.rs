use crate::Gradable;
use crate::Exp;
use crate::gradable;
use std::collections::HashMap;
use crate::bean::constant::Constant;
use std::rc::Rc;


pub struct Variable {
	pub name: String,
}

impl Gradable for Variable {
	fn grad(&self, name: &str) -> gradable {
		if self.name == *name {
			Rc::new(Box::new(Constant{value: 1.0}))
		} else {
			Rc::new(Box::new(Constant{value: 0.0}))
		}
	}
}

impl Exp for Variable {
	fn value(&self, param_value: &HashMap<String, f64>) -> f64 {
		if let Some(value) = param_value.get(&self.name) {
			*value
		} else {
			0.0
		}
	}
}