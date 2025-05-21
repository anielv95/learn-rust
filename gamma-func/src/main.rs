use statrs::distribution::Exp;
use statrs::function::gamma::gamma;
use ndarray::Array2;
use ndarray::Array1;

fn main() {
    println!("Hello, world!");
    let n = Exp::new(0.5).unwrap();
    println!("{}", n);
    let m = gamma(3.0);
    println!("{}",m);

    // 1. Loop over the rows of a 1D array
    let mut a = Array2::<f64>::zeros((2,2));
    let mut b = Array1::<f64>::zeros((20,));
    //for mut row in a.rows_mut() {
     //   row.fill(1.);
    //}
    //
    //a[[0,0]] +=0.1;
    println!("{}\n{}",a,b);
}
