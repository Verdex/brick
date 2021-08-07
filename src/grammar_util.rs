
use rparse::{Data, Field};

fn to_string(input : Vec<&Data>) -> String {
    let mut ret = vec![];
    for l in input {
        match l {
            Data::Char(c) => ret.push(c),
            x => panic!("Encountered non-Char item: {:?}", x),
        }
    }
    ret.into_iter().collect()
}

fn to_vec<T>(input : &Data, f : fn(&Data) -> T) -> Vec<T> {
    let (list, structure) = match input { 
        Data::Table { list, structure } => (list, structure),
        x => panic!("to_vec encountered non-table: {:?}", x),
    };

    assert_eq!( structure.len(), 0 );

    let mut ret = vec![];
    for l in list {
        ret.push(f(l));
    }

    ret
}

pub fn id_to_string(input : &Data) -> String { 
    let table = match input { 
        Data::Field(f) if f.rule == "identifier" => match &f.data {
            Data::Table{ list, .. } => list,
            x => panic!("id_to_string expected f.data to be table but found: {:?}", x),
        },
        x => panic!("id_to_string expected identifier but found: {:?}", x),
    };

    assert!( table.len() == 1 );
    
    let items = match &table[0] { 
        Data::Table { list, .. } => list,
        x => panic!("id_to_string expected table[0] to be table but found {:?}", x),
    };

    "blarg".to_string()
}