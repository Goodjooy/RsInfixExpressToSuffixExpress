mod expr;
mod factor;
mod item;

use std::collections::HashMap;
pub use crate::lexical::{LexicalDataIter,LexicalData};

pub type Iter = LexicalDataIter;
pub type Data = LexicalData;
// 语法分析器
// 分析语法
#[derive(Debug)]
pub struct SignTables{
    table : HashMap<(),()>,
    prevent:Option<Box<SignTables>>
}
//top node
#[derive(Debug)]
pub enum Expr {
    Add(Box<Expr>,Item),
    Min(Box<Expr>,Item),
    Item(Item),
}
#[derive(Debug)]
pub enum Item {
    Pro(Box<Item>,Factor),
    Div(Box<Item>,Factor),
    Factor(Factor)
}
#[derive(Debug)]
pub enum Factor {
    // 数字
    Digit(i64),
    // 符号表
    Sign(isize),
    // 小括号
    Aera(Box<Expr>)
}
#[derive(Debug)]
enum OptExpr {
    Nil,
    Expr(Expr)
}