/* use /// to create documentation (like below), 
    then use 'cargo doc' to build the documentation, which cargo will generate the documentations and save if into doc folder
    finally use 'cargo doc --open' to open the documentation for this rust project.
    very useful for documentating the project.
*/
///Function: subtract_ten
/// 
/// # Arguments (num:u32)
///
/// # Example
/// ```
/// let x = 15;
/// let y= subtract_ten(x);
/// ```

pub fn subtract_ten(num: u32) -> u32 {
    num - 10 //no need ; for the return part
}

//unit testing part
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn subtract_ten_test() {
        let x = 100;
        let y = subtract_ten(x);
        println!("y is from test: {}", y);

        // return true or false to indicate did the function passed the unit test
        assert_eq!(y, 90);
    }
}
