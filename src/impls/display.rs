use std::fmt::{Display, Formatter};

use crate::Node;

impl<Data: Display> Display for Node<Data> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        // Helper function to format the structure with indentation
        fn format_with_indent<Data: Display>(node: &Node<Data>, indent: usize, f: &mut Formatter<'_>) -> std::fmt::Result {
            let Node {
                data,
                children,
            } = node;

            if indent == 0 {
                writeln!(f, "{data}\n")?;
            } else {
                let indent_str = " ".repeat((indent - 1) * 2);
                writeln!(f, "{indent_str}* {data}")?;
            }

            // If there are children, recursively display them with increased indentation
            for child in children {
                format_with_indent(child, indent + 1, f)?;
            }

            Ok(())
        }

        let indent = f.alternate() as usize;
        format_with_indent(self, indent, f)
    }
}
