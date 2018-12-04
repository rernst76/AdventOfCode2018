pub fn do_something() {
    unimplemented!()
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn a_test() {
        assert_eq!(1,1);
    }
}