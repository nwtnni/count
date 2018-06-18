#[macro_export]
macro_rules! generate_counter {
    ($name:ident, $type:ident) => {

        #[allow(non_snake_case)]
        pub mod $name {
            use std::cell::Cell;

            thread_local!(
                static COUNTER: Cell<$type> = Cell::new(0);
            );

            pub fn next() -> $type {
                COUNTER.with(|cell| {
                    let n = cell.get();
                    cell.set(n + 1);
                    n
                })
            }

            #[allow(dead_code)]
            pub fn reset() {
                COUNTER.with(|cell| cell.set(0));
            }
        }
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_basic() {
        generate_counter!(Counter, i8);
        assert_eq!(0, Counter::next());
        assert_eq!(1, Counter::next());
        assert_eq!(2, Counter::next());
    }

    #[test]
    fn test_reset() {
        generate_counter!(Counter, i8);
        assert_eq!(0, Counter::next());
        assert_eq!(1, Counter::next());
        Counter::reset();
        assert_eq!(0, Counter::next());
    }
}
