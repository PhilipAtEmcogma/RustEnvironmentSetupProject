//mod means module, acts like importing a library
mod my_functions;
mod other_funcs;

//importing a specific function from the imported library
use crate::my_functions::add_five;
use crate::other_funcs::minus_funcs::subtract_ten;
//use crate::my_functions::* //importing every function in the library

//use crate::my_functions::{add_five, subtract_ten};
//functions can be imported as a bulk like above, or individually like below
/*
use create::my_functions::add_five;
use create::my_functions::subtrate_10;
*/

fn main() {
    //println!("Hello, World!");
    let x: u32 = 50;
    println!("x is {}", x);
    let y = add_five(x);
    println!("y is {}", y);
    let z = subtract_ten(y);
    println!("z is {}", z);
}
