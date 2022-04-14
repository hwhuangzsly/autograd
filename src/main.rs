use autograd::bean::constant::Constant;
use autograd::bean::variable::Variable;
use autograd::gradable;
use std::collections::HashMap;
use std::rc::Rc;

fn main() {
	// let mut dict = HashMap::new();
	// // x= 10
 //    dict.insert(String::from("x"), 10.0);

	// let x = Rc::new(Exp::V(Variable{name: String::from("x")}));
	// let a = Rc::new(Exp::C(Constant{value: 2.0}));
	// // x + a
 //    let exp = autograd::add(Rc::clone(&x), Rc::clone(&a));
 //    println!("{:?}", exp.value(&dict));

 //    let exp = autograd::add(exp, Rc::clone(&a));
 //    let exp = autograd::add(exp, Rc::clone(&x));
 //    let exp = autograd::mul(exp, Rc::clone(&x));
 //    // 2*(x+a)*x, x = 20 a = 2
 //    dict.insert(String::from("x"), 20.0);
 //    println!("{:?}", exp.value(&dict));
 //    println!("{:?}", exp.grad("x").value(&dict));
 //    // x = 2
 //    dict.insert(String::from("x"), 2.0);
 //    println!("{:?}", exp.value(&dict));
 //    println!("{:?}", exp.grad("x").value(&dict));
 	let x = 5.0;
 	let mut dict = HashMap::new();
	dict.insert(String::from("x"), x);
	println!("{:?}", function(x).grad("x").value(&dict));
}


fn function(x : f64) -> gradable {
	let vx : gradable = Rc::new(Box::new(Variable{name: String::from("x")}));
	if x < 0.0 {
		autograd::mul(Rc::clone(&vx), Rc::clone(&vx))
	} else {
		autograd::add(Rc::new(Box::new(Constant{value: 10.0})), Rc::clone(&vx))
	}

}