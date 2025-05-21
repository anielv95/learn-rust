use statrs::distribution::Exp;
use statrs::function::gamma::gamma;

fn main() {
    println!("Hello, world!");
    let n = Exp::new(0.5).unwrap();
    println!("{}", n);
    let m = gamma(70.0);
    println!("{}",m);
}
