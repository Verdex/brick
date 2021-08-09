
use std::collections::HashMap;
use rparse::{ParseRule, Data, self};

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

#[cfg(test)]
mod test {
    use super::*;

    fn next_data(data : &Data) -> Vec<&Data> {
        match data {
            Data::Nil => vec![],
            Data::Char(_) => vec![],
            Data::Field { data, .. } => vec![&data],
            Data::List(list) => {
                list.iter().collect::<Vec<&Data>>()
            },
        }
    }

    #[test]
    fn should_parse_identifiers() -> Result<(), ()> {
        let mut rules = HashMap::new();

        whitespace(&mut rules);
        identifier(&mut rules);

        rules.insert("main".to_string(), one_or_more!( invoke_rule!("identifier") ) );
        
        let result = rparse::parse("main", &rules, "blah Blah _blah _901 blah_blah BLAH6")?;

        let f : fn(&Data) -> Vec<&Data> = all_matches!(next_data, Data::List(_), Data::Field { .. }, Data::List(_), Data::List(_));

        println!( "{:?}", f(&result));
        panic!("blarg");

        Ok(())
    }
}