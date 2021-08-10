
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

fn let_expr(rules : &mut HashMap<String, ParseRule>) {

}

fn lambda_expr(rules : &mut HashMap<String, ParseRule>) {

}

fn string_literal(rules : &mut HashMap<String, ParseRule>) {

}

fn bool_literal(rules : &mut HashMap<String, ParseRule>) {
    rules.insert( "true".to_string(), match_string!("true") );
    rules.insert( "false".to_string(), match_string!("false") );

    rules.insert( "bool_literal".to_string()
                , and! { invoke_rule!( "whitespace" ) 
                       , or!( invoke_rule!("true"), invoke_rule!("false") )
                       , invoke_rule!( "whitespace" )
                       }
                );
}

fn number_literal(rules : &mut HashMap<String, ParseRule>) {
    rules.insert( "number_literal".to_string()
                , and! { invoke_rule!( "whitespace" ) 
                       , one_or_more!( on!(|c| c.is_numeric()) )
                       , invoke_rule!( "whitespace" )
                       }
                );
}

pub fn parse(input : &str) -> Result<Data, ()> {
    let mut rules = HashMap::new();

    whitespace(&mut rules);
    identifier(&mut rules);
    number_literal(&mut rules);
    bool_literal(&mut rules);

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

    fn get_ids(data : &Data) -> Vec<String> {

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

    fn get_numbers(data : &Data) -> Vec<u32> {
        let find : fn(&Data) -> Vec<&Data> = all_matches!( next_data
                                                         , Data::List(_)
                                                         , Data::Field { .. }
                                                         , Data::List(_)
                                                         , Data::List(_)
                                                         );
        find(&data).iter().map(|number| {
            let digits = match number {
                Data::List(digits) => digits,
                _ => panic!("expected list"),
            };

            digits.iter().map(|digit| match digit {
                Data::Char(d) => d,
                _ => panic!("expected char"),
            }).collect::<String>().parse::<u32>().unwrap()
        }).collect::<Vec<u32>>()
    }

    #[test]
    fn should_parse_numbers() -> Result<(), ()> {
        let mut rules = HashMap::new();

        whitespace(&mut rules);
        number_literal(&mut rules);

        rules.insert("main".to_string(), one_or_more!( invoke_rule!("number_literal") ) );

        let result = rparse::parse("main", &rules, "1 1234 09876")?;

        let ids = get_numbers(&result);

        assert_eq!( ids.len(), 3 );

        assert_eq!( ids[0], 1 );
        assert_eq!( ids[1], 1234 );
        assert_eq!( ids[2], 9876 );

        Ok(())
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