use std::collections::{HashMap, HashSet};

#[derive(Debug)]
pub struct Node {
    pub(crate) children: Vec<Node>,
    pub(crate) node_type: NodeType,
}

#[derive(Debug)]
pub enum NodeType {
    Text(String),
    Element(ElementData),
}

#[derive(Debug)]
pub struct ElementData {
    pub tag_name: String,
    pub attributes: AttrMap,
}

impl ElementData {
    pub fn id(&self) -> Option<&String> {
        self.attributes.get("id")
    }

    pub fn classes(&self) -> HashSet<&str> {
        match self.attributes.get("class") {
            Some(classlist) => classlist.split(' ').collect(),
            Node => HashSet::new()
        }
    }
}

pub type AttrMap = HashMap<String, String>;

pub fn text(data: String) -> Node {
    Node{ children: Vec::new(), node_type: NodeType::Text(data)}
}

pub fn elem(name: String, attrs: AttrMap, children: Vec<Node>) -> Node {
    Node {
        children,
        node_type: NodeType::Element(ElementData {
            tag_name: name,
            attributes: attrs,
        })
    }
}

