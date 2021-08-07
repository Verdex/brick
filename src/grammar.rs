
use std::collections::HashMap;
use rparse::{ParseRule, Data, Field, self};

fn whitespace(rules : &mut HashMap<String, ParseRule>) {
    rules.insert("whitespace".to_string()
                , zero_or_more!( on!( |c| c.is_whitespace() ) )
                );
}

fn identifier(rules : &mut HashMap<String, ParseRule>) {
    rules.insert( "identifier".to_string()
                , and! { invoke_rule!( "whitespace" ) 
                       , and!{ on!(|c| c.is_alphabetic() || c == '_')
                             , zero_or_more!( on!( |c| c.is_alphanumeric() || c == '_' ) )
                             } 
                       , invoke_rule!( "whitespace" )
                       }
                );
}

pub fn parse(input : &str) -> Result<Data, ()> {
    let mut rules = HashMap::new();

    whitespace(&mut rules);
    identifier(&mut rules);

    rparse::parse("main", &rules, input)
}