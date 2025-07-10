

use clearscreen::clear;
fn main() {
    clearscreen::clear().expect("failed to clear screen");

    println!("\nThis program wil use Eulers Constant and Natural Logrithms");

    let x: f64 = std::f64::consts::E;
    println!("\nEulers constant is {}.", x); 

    println!("Natural Log = log(e)x = ln x.");

    let y: f64 = x.log(x);
    println!("log(e)e = {}. ln e = {}.", y, y);
  
    print_ln(15.0);
    
    
}

fn print_ln(x: f64) {
    let euler: f64 = std::f64::consts::E;
    let a: f64 = x.clone();
    let b: f64 = a.log(euler);
    println!("log(e){} = {}. ln 10 = {}.\n", a, b, b);

}
