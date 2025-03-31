use derive_builder::Builder;
use derive_new::new;
use derive_setters::Setters;

mod impls;
mod macros;

pub use impls::*;

#[derive(new, Builder, Setters, Default, Ord, PartialOrd, Eq, PartialEq, Hash, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[setters(into, strip_option)]
pub struct Node<Data> {
    pub data: Data,
    pub children: Vec<Node<Data>>,
}

pub type Nodes<Data> = Vec<Node<Data>>;

impl<Data> Node<Data> {
    pub fn new_from(data: impl Into<Data>, children: impl IntoIterator<Item = Node<Data>>) -> Self {
        Self {
            data: data.into(),
            children: children.into_iter().collect(),
        }
    }
}
