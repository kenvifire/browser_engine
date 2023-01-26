use std::collections::HashMap;
use dom::{Node, ElementData, NodeType};
use css::{Stylesheet, Rule, Selector, SimpleSelector, Value, Specificity, };
use css::Selector::Simple;
use css::Value::Keyword;
use layout::{Display};

type PropertyMap = HashMap<String, Value>;

pub struct StyledNode<'a> {
    node: &'a Node,
    specified_values: PropertyMap,
    pub(crate) children: Vec<StyledNode<'a>>,
}

impl<'a> StyledNode<'a> {
    pub fn value(&self, name: &str) -> Option<Value> {
        self.specified_values.get(name).map(|v| v.clone())
    }

    pub fn lookup(&self, name: &str, fallback_name: &str, default: &Value) -> Value {
        self.value(name).unwrap_or_else(|| self.value(fallback_name)
            .unwrap_or_else(|| default.clone()))
    }

    pub(crate) fn display(&self) -> Display {
        match self.value("display") {
            Some(Keyword(s)) => match &*s {
                "block" => Display::Block,
                "none" => Display::None,
                _ => Display::Inline
            },
            _ => Display::Inline
        }
    }






}


fn matches(ele: &ElementData, selector: &Selector) -> bool {
    match *selector {
        Simple(ref simple_selector) => matches_simple_selector(ele, simple_selector)
    }
}

fn matches_simple_selector(elem: &ElementData, selector: &SimpleSelector) -> bool {
    if selector.tag_name.iter().any(|name| elem.tag_name != *name) {
        return false;
    }

    if selector.id.iter().any(|id| elem.id() != Some(id)) {
        return false;
    }

    let ele_classes = elem.classes();
    if selector.class.iter().any(|class| !ele_classes.contains(&**class)) {
        return false;
    }
    return true;
}

type MatchedRule<'a> = (Specificity, &'a Rule);

fn match_rule<'a>(elem: &ElementData, rule: &'a Rule) -> Option<MatchedRule<'a>> {
    rule.selectors.iter()
        .find(|selector| matches(elem, *selector))
        .map(|selector| (selector.specificity(), rule))
}

fn matching_rules<'a>(elem: &ElementData, stylesheet: &'a Stylesheet) -> Vec<MatchedRule<'a>> {
    stylesheet.rules.iter().filter_map(|rule| match_rule(elem, rule)).collect()
}

fn specified_values(elem: &ElementData, stylesheet: &Stylesheet) -> PropertyMap {
    let mut values = HashMap::new();
    let mut rules = matching_rules(elem, stylesheet);

    rules.sort_by(|&(a, _), &(b, _)| a.cmp(&b));

    for (_, rule) in rules {
        for declaration in &rule.declarations {
            values.insert(declaration.name.clone(), declaration.value.clone());
        }
    }
    return values;
}

pub fn style_tree<'a>(root: &'a Node, stylesheet: &'a Stylesheet) -> StyledNode<'a> {
    StyledNode {
        node: root,
        specified_values: match root.node_type {
            NodeType::Element(ref elem) => specified_values(elem, stylesheet),
            NodeType::Text(_) => HashMap::new()
        },

        children: root.children.iter().map(|child| style_tree(child, stylesheet)).collect(),
    }
}