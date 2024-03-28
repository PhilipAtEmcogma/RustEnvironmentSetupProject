pub fn subtract_ten(num: u32) -> u32{
    num-10
}

//unit testing part
#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn subtract_ten_test(){
        let x = 100;
        let y = subtract_ten(x);
        println!("y is from test: {}", y);

        // return true or false to indicate did the function passed the unit test
        assert_eq!(y, 90);
        
    }
}