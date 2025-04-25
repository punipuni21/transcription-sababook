use alloc::rc::Rc;
use alloc::rc::Weak;
use alloc::string::String;
use alloc::vec::Vec;
use core::cell::RefCell;

use crate::renderer::html::attribute::Attribute;

#[derive(Debug, Clone)]
pub struct Node {
    pub kind: NodeKind,
    window: Weak<RefCell<Window>>,
    parent: Weak<RefCell<Node>>,
    first_child: Option<Rc<RefCell<Node>>>,
    last_child: Weak<RefCell<Node>>,
    previous_sibling: Weak<RefCell<Node>>,
    next_sibling: Option<Rc<RefCell<Node>>>,
}

impl Node {
    pub fn new(kind: NodeKind) -> Self {
        Self {
            kind,
            window: Weak::new(),
            parent: Weak::new(),
            first_child: None,
            last_child: Weak::new(),
            previous_sibling: Weak::new(),
            next_sibling: None,
        }
    }

    pub fn set_parent(&mut self, parent: Weak<RefCell<Node>>) {
        self.parent = parent;
    }

    pub fn parent(&self) -> Weak<RefCell<Node>> {
        self.parent.clone()
    }

    pub fn set_first_child(&mut self, first_child: Option<Rc<RefCell<Node>>>) {
        self.first_child = first_child;
    }

    pub fn first_child(&self) -> Option<Rc<RefCell<Node>>> {
        self.first_child.as_ref().cloned()
    }

    pub fn set_last_child(&mut self, last_child: Weak<RefCell<Node>>) {
        self.last_child = last_child;
    }

    pub fn last_child(&self) -> Weak<RefCell<Node>> {
        self.last_child.clone()
    }

    pub fn set_previous_sibling(&mut self, previous_sibling: Weak<RefCell<Node>>) {
        self.previous_sibling = previous_sibling;
    }

    pub fn previous_sibling(&self) -> Weak<RefCell<Node>> {
        self.previous_sibling.clone()
    }

    pub fn set_next_sibling(&mut self, next_sibling: Option<Rc<RefCell<Node>>>) {
        self.next_sibling = next_sibling;
    }

    pub fn next_sibling(&self) -> Option<Rc<RefCell<Node>>> {
        self.next_sibling.as_ref().cloned()
    }
}

#[derive(Debug, Clone)]
pub struct Window {
    document: Rc<RefCell<Node>>,
}

#[derive(Debug, Clone)]
pub enum NodeKind {
    /// https://dom.spec.whatwg.org/#interface-document
    Document,
    /// https://dom.spec.whatwg.org/#interface-element
    Element(Element),
    /// https://dom.spec.whatwg.org/#interface-text
    Text(String),
}

#[derive(Debug, Clone)]
pub struct Element {
    kind: ElementKind,
    attributes: Vec<Attribute>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
/// https://dom.spec.whatwg.org/#interface-element
pub enum ElementKind {
    /// https://html.spec.whatwg.org/multipage/semantics.html#the-html-element
    Html,
    /// https://html.spec.whatwg.org/multipage/semantics.html#the-head-element
    Head,
    /// https://html.spec.whatwg.org/multipage/semantics.html#the-style-element
    Style,
    /// https://html.spec.whatwg.org/multipage/scripting.html#the-script-element
    Script,
    /// https://html.spec.whatwg.org/multipage/sections.html#the-body-element
    Body,
    /// https://html.spec.whatwg.org/multipage/grouping-content.html#the-p-element
    P,
    /// https://html.spec.whatwg.org/multipage/sections.html#the-h1,-h2,-h3,-h4,-h5,-and-h6-elements
    H1,
    H2,
    /// https://html.spec.whatwg.org/multipage/text-level-semantics.html#the-a-element
    A,
}
