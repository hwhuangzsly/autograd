extern crate regex;

use regex::Regex;
use autograd::bean::constant::Constant;
use autograd::bean::variable::Variable;
use autograd::gradable;
use parser::parse;
use std::collections::HashMap;
use std::rc::Rc;
use std::io::stdin;

pub mod parser;

fn main() {
 	let mut str_buf = String::new();
    stdin().read_line(&mut str_buf).expect("Failed to read line.");
    let functionx = parse(&str_buf.trim());

    let mut str_buf = String::new();
    stdin().read_line(&mut str_buf).expect("Failed to read line.");
    let target_and_values = str_buf.trim().split("|").collect::<Vec<&str>>();
    let target = target_and_values[0];

    let mut dict = HashMap::new();
    let re = Regex::new(r"(?x)(?P<name>[a-z]+)=(?P<value>.+)").unwrap();
    for value in target_and_values[1].split(",") {
    	re.captures(value).map(|cap| {
        	dict.insert(String::from(cap.name("name").map(|name| name.as_str()).unwrap()), cap.name("value").map(|value| value.as_str().parse().unwrap()).unwrap());
    	});
    }

 // 	let x = 5.0;
 // 	let y = 10.0;
	// dict.insert(String::from("x"), x);
	// dict.insert(String::from("y"), y);
	println!("{:?}", functionx.grad(target).value(&dict));
}


fn function(x : f64) -> gradable {
	let vx : gradable = Rc::new(Box::new(Variable{name: String::from("x")}));
	let vx2 = autograd::mul(Rc::clone(&vx), Rc::clone(&vx));
	if x < 0.0 {
		autograd::mul(Rc::clone(&vx), Rc::clone(&vx))
	} else {
		autograd::add(Rc::new(Box::new(Constant{value: 2.0})), Rc::clone(&vx2))
	}

}