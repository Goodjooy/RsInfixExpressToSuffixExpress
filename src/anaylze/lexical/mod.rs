use std::slice::Iter;
use std::usize;
use std::{collections::{HashMap, HashSet}, str};

use crate::status;

// #词法分析器
// * 分析表达式词法，便于语法分析器分析
//
mod iter;
 mod lexical;
mod do_anaylze;

#[derive(Debug)]
pub struct Lexical {
    //关键词散列表
    keyword: HashMap<String, LexicalData>,
    sign_set:HashSet<String>,
    res_list: Vec<LexicalData>,
}
pub struct LexicalDataIter{
    data:Vec<LexicalData>,
    index:usize
}
pub struct LexicalIter<'a> {
    preview: char,
    iter: std::str::Chars<'a>,
}
#[derive(Clone,PartialEq, PartialOrd,Debug)]
pub enum LexicalData {
    Nil,
    // 保留字- for if foreach loop else 等
    KeyWord(&'static str),
    // 数字
    NumInt(u32),
    NumFloat(f64),
    // 声明 类型
    Type(&'static str),
    // 变量
    Var(String),
    // 符号
    Sign(String),
}

