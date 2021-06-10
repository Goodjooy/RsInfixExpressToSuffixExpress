mod expr;
mod factor;
mod item;
mod display;
mod to_surfix;

pub use crate::anaylze::lexical::{LexicalData, LexicalDataIter};
use std::collections::HashMap;

pub type Iter = LexicalDataIter;
pub type Data = LexicalData;
// 语法分析器
// 分析语法
#[derive(Debug,PartialEq)]
pub struct SignTables {
    table: HashMap<(), ()>,
    prevent: Option<Box<SignTables>>,
}
//top node
#[derive(Debug,PartialEq)]
pub enum Expr {
    Some(Item, SubExpr),
}
#[derive(Debug,PartialEq)]
pub enum SubExpr {
    Add(Item, Box<SubExpr>),
    Min(Item, Box<SubExpr>),
    Nil,
}

#[derive(Debug,PartialEq)]
pub enum Item {
    Some(Factor,SubItem),
}
#[derive(Debug,PartialEq)]
pub enum SubItem {
    Pro(Factor, Box<SubItem>),
    Div(Factor, Box<SubItem>),
    Nil,
}

#[derive(Debug,PartialEq)]
pub enum Factor {
    // 数字
    Digit(i64),
    // 符号表
    Sign(isize),
    // 小括号
    Aera(Box<Expr>),
}
#[derive(Debug,PartialEq)]
enum OptExpr {
    Nil,
    Expr(Expr),
}

#[derive(Debug,PartialEq, PartialOrd)]
pub enum Final {
    Digit(i64),
    Add,
    Min,

    Pro,
    Div

}

pub trait ToSurfix {
    fn to_surfix(self)->Vec<Final>;
}