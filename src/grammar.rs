
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

    fn get_ids(data : &Data) -> Vec<String> {
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

        let find : fn(&Data) -> Vec<&Data> = all_matches!(next_data, Data::List(_), Data::Field { .. }, Data::List(_), Data::List(_));
        let find_first : fn(&Data) -> Vec<&Data> = all_matches!(next_data, Data::List(_), Data::Char(_));
        let find_rest : fn(&Data) -> Vec<&Data> = all_matches!(next_data, Data::List(_), Data::List(_), Data::Char(_));

        let proto = find(data);

        proto.iter().map(|x| {

            let first = find_first(x);
            let rest = find_rest(x);

            first.iter().chain(rest.iter()).map( |x| match x {
                Data::Char(c) => c,
                _ => panic!("expected only Data::Char"),
            }).collect()
        }).collect()
    }

    #[test]
    fn should_parse_identifiers() -> Result<(), ()> {
        let mut rules = HashMap::new();

        whitespace(&mut rules);
        identifier(&mut rules);

        rules.insert("main".to_string(), one_or_more!( invoke_rule!("identifier") ) );
        
        let result = rparse::parse("main", &rules, "blah Blah _blah _901 blah_blah BLAH6")?;

        let ids = get_ids(&result);

        assert_eq!( ids.len(), 6 );

        assert_eq!( ids[0], "blah" );
        assert_eq!( ids[1], "Blah" );
        assert_eq!( ids[2], "_blah" );
        assert_eq!( ids[3], "_901" );
        assert_eq!( ids[4], "blah_blah" );
        assert_eq!( ids[5], "BLAH6" );

        Ok(())
    }
}