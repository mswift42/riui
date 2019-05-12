use std::{fs, process};
use std::io::Read;
use std::collections::HashSet;
use roxmltree::{Node, Document};


fn load_file(path: &str) -> String {
    let mut file = fs::File::open(&path).unwrap();
    let mut text = String::new();
    file.read_to_string(&mut text).unwrap();
    text
}

pub struct AttributeNode<'a, 'd: 'a> {
    node: Vec<Node<'a, 'd>>
}

impl<'a, 'd: 'a> AttributeNode<'a, 'd> {
    pub fn new(doc: &'d Document) -> AttributeNode<'a, 'd> {
        let an= doc.descendants()
            .filter(|node| node.tag_name().name() == "attributes")
            .next().unwrap().children().collect();
        AttributeNode{node: an}
    }
}


fn main() {
    let txt = load_file("white-sand.xml");
    let doc = match roxmltree::Document::parse(&txt) {
        Ok(v) => v,
        Err(e) => {
            println!("Error: {}.", e);
            process::exit(1);
        }
    };
    println!("Elements count: {}",
             doc.root().descendants().filter(|n| n.is_element()).count());
//    for node in doc.descendants() {
//        if node.is_element() {
//            println!("{:?} at {},", node.tag_name(), doc.text_pos_at(node.range().start));
//        }
//    }
    let attrs: Vec<Node> = doc.descendants().filter(|node| node.tag_name().name() == "attributes")
        .next().unwrap().children().collect();

    println!("Attributes: {:?}", attrs);
    let an = AttributeNode::new(&doc);
    println!("AttributeNode: {:?}", an.node)


//    let ch = doc.descendants().find(|node| node.tag_name().name() == "attributes").unwrap()
//        .descendants().filter_map(|node| node.resolve_tag_name_prefix()).collect();
//    println!("{:?}", ch);
}
