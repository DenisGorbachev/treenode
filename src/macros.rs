#[macro_export]
macro_rules! n {
    ($data:expr) => {
        $crate::Node::new($data, vec![])
    };
    ($data:expr, $children:expr) => {
        $crate::Node::new($data, $children)
    };
}

#[macro_export]
macro_rules! s {
    ($data:expr) => {
        $crate::Node::<String>::new_from($data, vec![])
    };
    ($data:expr, $children:expr) => {
        $crate::Node::<String>::new_from($data, $children)
    };
}

#[cfg(test)]
mod tests {
    use crate::Node;

    #[test]
    fn must_test_n_macro() {
        assert_eq!(n!("foo"), Node::new("foo", vec![]))
    }
}
