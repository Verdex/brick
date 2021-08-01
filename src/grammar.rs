
use std::collections::HashMap;
use rparse::{ParseRule, Data, self};


fn identifier(rules : &mut HashMap<String, ParseRule>) {
    rules.insert( "identifier".to_string()
                , Any!()
                );
}


pub fn parse(input :&str) -> Result<Data, ()> {
    let mut rules = HashMap::new();

    identifier(&mut rules);

    rparse::parse("main", &rules, input)
}