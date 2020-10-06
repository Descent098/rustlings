// iterators4.rs

pub fn factorial(num: u64) -> u64 {
    // Complete this function to return the factorial of num
    // Do not use:
    // - return
    // Try not to use:
    // - imperative style loops (for, while)
    // - additional variables
    // For an extra challenge, don't use:
    // - recursion
    // Execute `rustlings hint iterators4` for hints.

    // // My recursive implementation
    // if num == 0{
    //     1
    // } else if num == 1{
    //     1
    // } else {
    //     factorial(num - 1) * num
    // }


    // (1..=num).fold(1, | accumulator, value | accumulator * value)
    /* This goes through the range 1-num (inclusive) accumulator is equal
    to every prior value, and value is the current itteration value. This
    being the case each itteration the current accumulator is multiplied by
    the value and then becomes the new accumulator, so if num = 4 then the loop goes:
        Iteration 1: 
            accumulator == 1, 
            value == 1,
            accumulator * value == 1
        Iteration 2: 
            accumulator == 1, 
            value == 2,
            accumulator * value == 2
        Iteration 3: 
            accumulator == 2, 
            value == 3,
            accumulator * value == 6
        Iteration 4: 
            accumulator == 6, 
            value == 4,
            accumulator * value == 24
     */


    (1..=num).product() // This is a shortform of the function on line 23
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn factorial_of_1() {
        assert_eq!(1, factorial(1));
    }
    #[test]
    fn factorial_of_2() {
        assert_eq!(2, factorial(2));
    }

    #[test]
    fn factorial_of_4() {
        assert_eq!(24, factorial(4));
    }
}
