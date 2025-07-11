use clearscreen::clear;
use std::io;

fn main() {
    //clear the console
    clearscreen::clear().expect("failed to clear screen");

    println!("\nThis program wil use Eulers Number and Natural Logrithms");
    

    // x is the Rust calculation or Euler's Number
    let x: f64 = std::f64::consts::E;
    
    find_euler();
    let y: f64 = x.log(x);
    println!("Example: log(e)e = {}. ln e = {}.\n", y, y);

   
    println!("For the calculation y = ln x, we will solve for y. What is your x?:");
    let y1: f64 = get_float();


  
    print_ln(y1);

    
    
    
}

fn print_ln(x: f64) {
    let euler: f64 = std::f64::consts::E;
    let a: f64 = x.clone();
    let b: f64 = a.log(euler);
    println!("log(e){} = {}. ln {} = {}.\n", a, b, a, b);

}

fn find_euler(){
    //find_euler function calculates Eulers Number with Rust math.
    println!("\nEulers Number is (1 + x) raised to the power of 1/x. Or (1 + x) ^ (1/x)");
    println!("Eulers Number calculated by Rust is {}.", std::f64::consts::E);
    // (1 + x) raised to the power of 1/x.  
    let mut x = 1.0;
    let mut y: f64 = 0.0;
   
    for i in 1..54 {
        y += y.clone();
        y = (1.0_f64 + x).powf(1.0_f64/x);
        x = x/2.0;
        
        
    }
    println!("Eulers Number calculated by code is {}.", y);
    println!("Natural Log = log(e)x = ln x.");


}

fn get_float() -> f64 {
    let mut my_input = String::new();
    io::stdin().read_line(&mut my_input).expect("failed to read line.");
    let x: f64 = make_float(my_input.clone());
    x
}

fn make_float(x: String) -> f64 {
    let y = x.trim().parse().expect("Failed to parse");
    return y;
}
