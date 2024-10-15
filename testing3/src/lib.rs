fn prints_and_return10(a: i32) -> i32 {
    println!("i got the {a}");
    10
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]
    fn thistestwillpass() {
        let value = prints_and_return10(4);
        assert_eq!(value, 10);
    }

    #[test]
    fn thistestwillfail() {
        let value = prints_and_return10(8);
        assert_eq!(value, 5);
    }
}
