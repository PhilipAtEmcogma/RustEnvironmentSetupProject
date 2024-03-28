
fn add_five(num: u32) -> u32{
    num+5
}

fn main(){
    //println!("Hello, World!");
    let x: u32 = 50;
    println!("x is {}", x);
    let y = add_five(x);
    println!("y is {}", y);
}