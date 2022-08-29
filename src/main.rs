mod common;
use common::variable::Variable;
use common::function::Function;
use common::function::Square;
use common::function::Exp;

//fn numeric_diff(f: &mut impl Function, x: Variable, eps: f32) -> f32 {
//    let x0 = Variable::new(x.data - eps);
//    let x1 = Variable::new(x.data + eps);
//    let y0 = f.exec(&x0);
//    let y1 = f.exec(&x1);
//    (y1.data - y0.data) / (2.0 * eps)
//}

fn main() {
    let mut x = Variable::new(0.5);
    let mut a = Square::new(&x);
    let mut ax = a.exec();
    let mut b = Exp::new(&ax);
    let mut bx = b.exec();
    let mut c = Square::new(&bx);

    let mut y = c.exec();

    y.grad = Some(1.0);
    bx.grad = Some(c.backward(y.grad.unwrap()));
    ax.grad = Some(b.backward(bx.grad.unwrap()));
    x.grad = Some(a.backward(ax.grad.unwrap()));

    println!("{}", x.grad.unwrap());
    //println!("v = {:?}", numeric_diff(&mut Square::new(), x, eps));
}
