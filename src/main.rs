

use clearscreen::clear;
fn main() {
    clearscreen::clear().expect("failed to clear screen");

    println!("\nThis program wil use Eulers Constant and Natural Logrithms");

    let x: f64 = std::f64::consts::E;
    find_euler();

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

fn find_euler(){
    println!("\nEulers Constant is (1 + x) raised to the power of 1/x.");
    println!("Eulers constant calculated by Rust is {}.", std::f64::consts::E);
    // (1 + x) raised to the power of 1/x.  
    let mut x = 1.0;
    let mut y: f64 = 0.0;
   
    for i in 1..54 {
        y += y.clone();
        y = (1.0_f64 + x).powf(1.0_f64/x);
        x = x/2.0;
        
        
    }
    println!("Eulers constant calculated by code is {}.", y);


}
