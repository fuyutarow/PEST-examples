extern crate pest;
#[macro_use]
extern crate pest_derive;
use std::fs;

use pest::Parser;

#[derive(Parser)]
#[grammar = "csv.pest"]

pub struct CSVParser;

fn main() {
    let successful_paarse = CSVParser::parse(Rule::field, "-273.15");
    println!("{:?}", successful_paarse);

    let unsuccessful_paarse = CSVParser::parse(Rule::field, "this is no a nubmer");
    println!("{:?}", unsuccessful_paarse);

    let unparsed_file = fs::read_to_string("examples/numbers.csv").expect("cannot read file");

    let file = CSVParser::parse(Rule::file, &unparsed_file)
        .expect("unsucessful parse")
        .next()
        .unwrap();

    dbg!(&file);

    let mut field_sum: f64 = 0.0;
    let mut record_count: u64 = 0;
    for record in file.into_inner() {
        match record.as_rule() {
            Rule::record => {
                record_count += 1;
                for field in record.into_inner() {
                    field_sum += field.as_str().parse::<f64>().unwrap();
                }
            }
            Rule::EOI => (),
            _ => unreachable!(),
        }
    }
    println!("Sum of fields: {}", field_sum);
    println!("Number of records: {}", record_count);
}
