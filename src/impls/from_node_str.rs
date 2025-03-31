use crate::Node;

impl<'a> From<Node<&'a str>> for Node<String> {
    fn from(node: Node<&'a str>) -> Self {
        Self {
            data: node.data.to_string(),
            children: node.children.into_iter().map(Into::into).collect(),
        }
    }
}
