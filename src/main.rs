mod common;
use common::variable::Variable;
use common::function::Function;
use common::function::Square;
use common::function::Exp;

fn numeric_diff(f: &mut dyn Function, x: Variable, eps: f32) -> f32 {
    let x0 = Variable::new(x.data - eps);
    let x1 = Variable::new(x.data + eps);
    let y0 = f.exec(x0);
    let y1 = f.exec(x1);
    (y1.data - y0.data) / (2.0 * eps)
}

fn main() {
    let x = Variable::new(2.0);
    let eps: f32 = 1e-4;
    println!("v = {:?}", numeric_diff(&mut Square::new(), x, eps));
}
