use std::cell::RefCell;
use std::rc::Rc;

mod common;
use common::variable::Variable;
use common::function::Function;

//fn numeric_diff(f: &mut impl Function, x: Variable, eps: f32) -> f32 {
//    let x0 = Variable::new(x.data - eps);
//    let x1 = Variable::new(x.data + eps);
//    let y0 = f.exec(&x0);
//    let y1 = f.exec(&x1);
//    (y1.data - y0.data) / (2.0 * eps)
//}

fn main() {
    let x = Rc::new(RefCell::new(Variable::new(0.5, Function::None)));
    let s1 = Rc::new(RefCell::new(Function::square(x.clone())));
    let e = Rc::new(RefCell::new(Function::exp(s1.clone())));
    let y = Rc::new(RefCell::new(Function::square(e.clone())));

    RefCell::borrow_mut(&y).grad = Some(1.0);
    RefCell::borrow_mut(&y).backward();
    println!("{}", x.borrow().grad.unwrap()); 
}
