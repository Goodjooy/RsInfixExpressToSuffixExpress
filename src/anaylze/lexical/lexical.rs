use std::{
    collections::{HashMap, HashSet},
    str,
};

use super::{Lexical, LexicalData};

impl Lexical {
    pub fn init() -> Self {
        let mut v = Lexical {
            keyword: HashMap::new(),
            sign_set: HashSet::new(),
            res_list: Vec::new(),
        };
        v.add_keyword(LexicalData::new_keyword("if"));
        v.add_keyword(LexicalData::new_keyword("for"));
        v.add_keyword(LexicalData::new_keyword("foreach"));
        v.add_keyword(LexicalData::new_keyword("while"));
        v.add_keyword(LexicalData::new_keyword("loop"));
        v.add_keyword(LexicalData::new_keyword("match"));

        v.add_keyword(LexicalData::new_type("i32"));
        v.add_keyword(LexicalData::new_type("f64"));

        let signs: Vec<String> = vec![
            "+", "-", "*", "/", "%", //基本运算符
            "+=", "-=", "*=", "/=", "%=", //赋值运算符
            "{", "}", "[", "]", "(", ")", //各级括号
            "<", ">", "==", "<=", ">=", "!=", // 逻辑判定符
            "||", "&&", "!", // 逻辑运算符
            ":", ":=", "=", //赋值运算符
            "^", "|", "&", //位运算符
            "^=", "|=", "&=", // 位运算赋值运算符
        ]
        .iter()
        .map(|s| -> String { String::from(*s) })
        .collect();
        v.sign_set.extend(signs);
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
    fn new_type(name: &'static str) -> (String, LexicalData) {
        (name.to_string(), LexicalData::Type(name))
    }
}
