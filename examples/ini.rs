extern crate pest;
#[macro_use]
extern crate pest_derive;
use pest::Parser;

#[derive(Parser)]
#[grammar = "ini.pest"]
pub struct INIParser;

use std::collections::HashMap;
use std::fs;

fn main() {
    let unparsed_file = fs::read_to_string("examples/config.ini").expect("cannot read file");

    let file = INIParser::parse(Rule::file, &unparsed_file)
        .expect("unsucessful parse")
        .next()
        .unwrap();

    let mut properties: HashMap<&str, HashMap<&str, &str>> = HashMap::new();

    let mut current_section_name = "";

    for line in file.into_inner() {
        match line.as_rule() {
            Rule::section => {
                let mut inner_rules = line.into_inner();
                current_section_name = inner_rules.next().unwrap().as_str();
            }
            Rule::property => {
                let mut inner_rules = line.into_inner();
                let name: &str = inner_rules.next().unwrap().as_str();
                let value: &str = inner_rules.next().unwrap().as_str();
                let section = properties.entry(current_section_name).or_default();
                section.insert(name, value);
            }
            Rule::EOI => (),
            _ => unreachable!(),
        }
    }

    println!("{:#?}", properties);
}
