use std::{fs, process};
use std::io::Read;
use std::collections::HashSet;
use serde::{Serialize, Deserialize};


fn load_file(path: &str) -> String {
    let mut file = fs::File::open(&path).unwrap();
    let mut text = String::new();
    file.read_to_string(&mut text).unwrap();
    text
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ThemeData {
    theme_attrs: Vec<AttrOption>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AttrOption {
    name: String,
    value: Vec<AttrValue>,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct AttrValue {
    name: String,
    value: String,
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
    let attrs = doc.descendants().filter(|node| node.tag_name().name() == "attributes").count();

    println!("{}", attrs);

    let mut uris = HashSet::new();
    for node in doc.descendants().find(|node| node.tag_name().name() == "attributes").unwrap().descendants() {
        for ns in node.namespaces() {
            uris.insert((ns.name().unwrap_or("\"\"").to_string(), ns.uri().to_string()));
        }
    }
    println!("{:?}", uris.len());

//    let ch = doc.descendants().find(|node| node.tag_name().name() == "attributes").unwrap()
//        .descendants().filter_map(|node| node.resolve_tag_name_prefix()).collect();
//    println!("{:?}", ch);
}
