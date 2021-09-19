use html_parser::{Dom, Element, Node};

pub trait Parseable {
    fn first_child(&self) -> Option<&Node>;
    fn first_child_unchecked(&self) -> &Node;
}

impl Parseable for Element {
    fn first_child(&self) -> Option<&Node> {
        self.children.get(0)
    }

    fn first_child_unchecked(&self) -> &Node {
        &self.children[0]
    }
}

impl Parseable for Dom {
    fn first_child(&self) -> Option<&Node> {
        self.children.get(0)
    }

    fn first_child_unchecked(&self) -> &Node {
        &self.children[0]
    }
}
