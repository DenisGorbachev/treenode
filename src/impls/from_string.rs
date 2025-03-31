use crate::Node;

impl From<String> for Node<String> {
    fn from(data: String) -> Self {
        Self {
            data,
            children: vec![],
        }
    }
}
