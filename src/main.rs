use std::{fs, process};
use std::io::Read;
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
            .find(|node| node.tag_name().name() == "attributes")
            .unwrap().children().collect();
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
             doc.root().descendants()
                 .filter(|n| n.is_element()).count());
    let attrs: Vec<Node> = doc.descendants()
        .find(|node| node.tag_name().name() == "attributes")
        .unwrap().children().collect();

    println!("Attributes: {:?}", attrs);
    let an = AttributeNode::new(&doc);
    println!("AttributeNode: {:?}", an.node);

    let mut vs = Vec::new();
    for node in &an.node {
        let an= node.next_sibling().filter(|node| node.is_element())
            .map(|node| node.descendants()
                .filter(|node| node.tag_name().name() == "option"));
        let _ar = match an {
            Some(mut v) => vs.push(v.next()),
            None => println!("None"),
        };
    }
    println!("{:?}", vs)


//    let ch = doc.descendants().find(|node| node.tag_name().name() == "attributes").unwrap()
//        .descendants().filter_map(|node| node.resolve_tag_name_prefix()).collect();
//    println!("{:?}", ch);
}
