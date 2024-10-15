/*pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test] //it is teest function so test will know that it is test function
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
*/

//now the test will write explor

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn exploration() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

//how the teesst will fail when something in test functioon paanic

/*
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exploration() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn another() {
        panic!("Make this test fail");
    }
}

*/

//assert! macro, provided by the standard library, is useful when you want to ensure that some condition in a test evaluates to true
