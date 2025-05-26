use statrs::distribution::Exp;
use statrs::function::gamma::gamma;
use ndarray::Array;

fn main() {
    println!("Hello, world!");
    let n = Exp::new(0.5).unwrap();
    println!("{}", n);
    let m = gamma(3.0);
    println!("{}",m);

    let x_array = Array::linspace(1.,2.,101);
    println!("\n{}",x_array);
    let mut y = Array::range(1.,2.01,0.01);
    //let y = gamma(x_array);
    println!("\n{}",y);
    let mut index= 0;
    while index<101 {
        y[index] = gamma(x_array[index]);
        index = index+1;
    }
    println!("\n{}",y);
}
