use clearscreen::clear;
fn main() {
    clearscreen::clear().expect("failed to clear screen");
    println!("\nThis program wil use Eulers Constant and Natural Logrithms");
    let x: f64 = std::f64::consts::E;
    println!("\nEulers constant is {}.\n", x); 
}
