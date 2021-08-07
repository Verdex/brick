
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

#[cfg(test)]
mod test {
    use super::*;


    #[test]
    fn should_parse_identifiers() -> Result<(), ()> {
        let mut rules = HashMap::new();

        whitespace(&mut rules);
        identifier(&mut rules);

        rules.insert("main".to_string(), one_or_more!( invoke_rule!("identifier") ) );
        
        let result = rparse::parse("main", &rules, "blah Blah _blah _901 blah_blah BLAH6")?;

        let ids = result.find(|d| match d {
            Data::Field(f) => {
                let Field { rule, data } = &**f;
                rule == "identifier"
            },
            _ => false,
        });

        assert_eq!( ids.len(), 6) ;

        let ids = ids.into_iter().map( |d| match d {
            Data::Field(f) => match f.data {
                Data::Table { list, .. } => list,
                _ => panic!( "Field Data should be table"),
            },
            _ => panic!( "expected field"),
        });

        for id in ids {
            println!("{:?}", id);
        }

        panic!("blar");
        Ok(())
    }
}