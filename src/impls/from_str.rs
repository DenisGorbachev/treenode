use crate::Node;

impl From<&str> for Node<String> {
    fn from(s: &str) -> Self {
        Self {
            data: s.to_string(),
            children: vec![],
        }
    }
}
