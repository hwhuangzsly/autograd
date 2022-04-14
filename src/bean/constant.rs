use crate::Gradable;
use crate::Exp;
use crate::gradable;
use std::collections::HashMap;
use std::rc::Rc;

pub struct Constant {
	pub value: f64,
}

impl Gradable for Constant {
	fn grad(&self, _name: &str) -> gradable {
		Rc::new(Box::new(Constant{value: 0.0}))
	}
}

impl Exp for Constant {
	fn value(&self, _param_value: &HashMap<String, f64>) -> f64 {
			self.value
		}
}