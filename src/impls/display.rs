use std::fmt::{Display, Error, Formatter};

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
                let indent_sub = indent
                    .checked_sub(1)
                    .expect("always succeeds because indent != 0");
                let repeat_count = indent_sub.checked_mul(2).ok_or(Error)?;
                let indent_str = " ".repeat(repeat_count);
                writeln!(f, "{indent_str}* {data}")?;
            }

            if !children.is_empty() {
                let indent_for_children = indent.checked_add(1).ok_or(Error)?;
                // If there are children, recursively display them with increased indentation
                for child in children {
                    format_with_indent(child, indent_for_children, f)?;
                }
            }

            Ok(())
        }

        let indent = f.alternate() as usize;
        format_with_indent(self, indent, f)
    }
}
