pub mod gofl;
pub use crate::gofl::board_wrapper::BoardWrapper;

#[cfg(test)]
mod tests {
    use crate::gofl::BoardWrapper;

    fn helper() -> (i64, i64) {
        (387i64, 387i64)
    }

    #[test]
    fn it_works() {
        let (a, b) = helper();
        let bo = BoardWrapper::new(a, b, 11u64);
        let result = bo.add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn add_cross() {
        let (a, b) = helper();
        let mut bo = BoardWrapper::new(a, b, 11u64);
        let alive_count_before = bo.count_alive();
        bo.set_cross();
        let alive_count_after = bo.count_alive();
        assert_ne!(alive_count_before, alive_count_after);
    }
}
