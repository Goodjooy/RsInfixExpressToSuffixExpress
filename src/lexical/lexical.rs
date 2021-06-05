use std::{collections::HashMap, str};

use super::{Lexical, LexicalData, LexicalIter};

impl Lexical {
    fn init() -> Self {
        let mut v = Lexical {
            keyword: HashMap::new(),
            res_list: Vec::new(),
        };
        v.add_keyword(LexicalData::new_keyword("if"));
        v.add_keyword(LexicalData::new_keyword("for"));
        v.add_keyword(LexicalData::new_keyword("foreach"));

        v
    }

    fn add_keyword(&mut self, data: (String, LexicalData)) -> Option<LexicalData> {
        self.keyword.insert(data.0, data.1)
    }
    
}



impl LexicalData {
    fn new_keyword(name: &'static str) -> (String, LexicalData) {
        (name.to_string(), LexicalData::KeyWord(name))
    }
}
