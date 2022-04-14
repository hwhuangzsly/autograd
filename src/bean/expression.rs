use crate::Operate;
use crate::Exp;
use crate::Gradable;
use crate::gradable;
use crate::add;
use crate::mul;
use crate::bean::constant::Constant;
use std::collections::HashMap;
use std::rc::Rc;

pub struct Expression {
	pub operate: Operate,
	pub params: Vec<gradable>,
}

impl Gradable for Expression {
    fn grad(&self, name: &str) -> gradable {
        match self.operate {
            Operate::ADD => add(self.params[0].grad(name), self.params[1].grad(name)),
            Operate::MUL => {
                let x = Rc::clone(&self.params[0]);
                let y = Rc::clone(&self.params[1]);
                let z = Rc::clone(&self.params[1]);
                add(mul(x.grad(name), y), mul(x, z.grad(name)))
            }
            _ => Rc::new(Box::new(Constant{value: 0.0}))
        }
    }
}

impl Exp for Expression {
    fn value(&self, param_value: &HashMap<String, f64>) -> f64 {
        match self.operate {
            Operate::MINUS => -&self.params[0].value(param_value),
            Operate::ADD => &self.params[0].value(param_value) + &self.params[1].value(param_value),
            Operate::MUL => &self.params[0].value(param_value) * &self.params[1].value(param_value),
            _ => 0.0,
        }
    }
}
