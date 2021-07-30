use std::collections::{HashMap, HashSet};

#[derive(Debug, PartialEq, Clone)]
pub struct Node {
    pub node_type: NodeType,
    pub children: Vec<Node>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum NodeType {
    Text(String),
    Comment(String),
    Cdata(String),
    Element(ElementData),
    Meta(MetaData),
}

#[derive(Debug, PartialEq, Clone)]
pub struct ElementData {
    pub tag_name: String,
    pub attributes: AttrMap,
}

#[derive(Debug, PartialEq, Clone)]
pub struct MetaData {
    pub attributes: AttrMap,
}

impl ElementData {
    pub fn id(&self) -> Option<&String> {
        self.attributes.get("id")
    }
    pub fn classes(&self) -> HashSet<&str> {
        match self.attributes.get("class") {
            Some(classlist) => classlist.split(' ').collect(),
            None => HashSet::new(),
        }
    }
}

type AttrMap = HashMap<String, String>;
