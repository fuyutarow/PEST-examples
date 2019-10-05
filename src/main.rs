extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;

#[derive(Parser)]
#[grammar = "csv.pest"]

pub struct CSVParser;

fn main() {
    let successful_paarse = CSVParser::parse(Rule::field, "-273.15");
    println!("{:?}", successful_paarse);

    let unsuccessful_paarse = CSVParser::parse(Rule::field, "this is no a nubmer");
    println!("{:?}", unsuccessful_paarse);
}
