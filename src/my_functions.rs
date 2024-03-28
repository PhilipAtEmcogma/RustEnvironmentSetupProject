//making the function as publich using the keyword 'pub', will allow the function to be called else where
pub fn add_five(num: u32) -> u32{
    num+5
}

//the line below indicates to rust that it will be used as unit testing for this module (file)
#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn add_five_test(){
        let x = 100;
        let y = add_five(x);
        println!("y is from test: {}", y);

        // return true or false to indicate did the function passed the unit test
        assert_eq!(y, 105);

    }
}