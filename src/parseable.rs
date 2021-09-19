use html_parser::{Dom, Element, Node};

pub trait Parseable {
    fn first_child(&self) -> &Node;
}

impl Parseable for Element {
    fn first_child(&self) -> &Node {
        return &self.children[0];
    }
}

impl Parseable for Dom {
    fn first_child(&self) -> &Node {
        return &self.children[0];
    }
}
