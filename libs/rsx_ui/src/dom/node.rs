use std::{
    borrow::BorrowMut,
    cell::RefCell,
    collections::HashMap,
    fmt::Debug,
    rc::{Rc, Weak},
};

use super::{
    css::Value,
    dom::{Node, NodeType},
};

// Parent-child tree: https://gist.github.com/piboistudios/c8db0e9b25efbcd5fa22c53e3db89780
#[derive(Debug)]
pub struct StyledNode {
    pub node: Node,
    pub children: RefCell<Vec<Rc<StyledNode>>>,
    parent: RefCell<Weak<StyledNode>>,
    pub specified_values: PropertyMap,
}

#[derive(Debug)]
pub struct StyledTree {
    pub root: RefCell<Rc<StyledNode>>,
}

type PropertyMap = HashMap<String, Value>;

impl StyledTree {
    pub fn new() -> Self {
        println!("New Styled Node!");
        StyledTree {
            root: RefCell::new(Rc::new(StyledNode {
                node: Node {
                    node_type: NodeType::Comment(String::from("comment1")),
                    children: vec![],
                },
                children: RefCell::new(vec![]),
                parent: RefCell::new(Default::default()),
                specified_values: Default::default(),
            })),
        }
    }
    pub fn make(&self) -> Rc<StyledNode> {
        println!("Making Styled Node!");
        Rc::new(StyledNode {
            node: Node {
                node_type: NodeType::Comment(String::from("comment2")),
                children: vec![],
            },
            children: RefCell::new(vec![]),
            parent: RefCell::new(Weak::new()),
            specified_values: Default::default(),
        })
    }
    pub fn make_with(
        &self,
        node: Node,
        specified_values: PropertyMap,
        children: RefCell<Vec<Rc<StyledNode>>>,
    ) -> Rc<StyledNode> {
        let rc = Rc::new(StyledNode {
            node,
            children,
            parent: RefCell::new(Default::default()),
            specified_values,
        });
        for ch in rc.children.borrow().iter() {
            *ch.parent.borrow_mut() = Rc::downgrade(&rc);
        }
        return rc;
    }
    pub fn set_root(&self, node: &Rc<StyledNode>) {
        println!("Setting root {:?}", node);
        *self.root.borrow_mut() = Rc::clone(node);
    }
    pub fn append(&self, parent: &Rc<StyledNode>, child: &Rc<StyledNode>) {
        println!("Appending {:?} {:?}", parent, child);
        parent.children.borrow_mut().push(Rc::clone(child));
        *child.parent.borrow_mut() = Rc::downgrade(parent);
    }

    pub fn walk(&self) {
        println!("Walking tree!");
        println!("Root: {:?}", self.root);

        self.walk_recursive(self.root.borrow_mut().as_ref());
    }

    pub fn walk_recursive(&self, node: &StyledNode) {
        node.children.borrow_mut().iter_mut().for_each(|child| {
            println!("Child: {:?}", child);
            self.walk_recursive(child.borrow_mut().as_ref())
        })
    }
}
