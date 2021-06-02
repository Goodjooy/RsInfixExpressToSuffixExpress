use crate::express::Express;
use crate::factor::Factor;
use crate::item::Item;
#[derive(PartialEq, Debug)]

pub enum Digit {
    PostiveSign,
    NegativeSign,
    Num(i32),
}

impl Digit {
    pub fn to_digit(last_exp: Express, ch: char) -> Express {
        let num = match ch.to_digit(10) {
            Some(n) => Digit::new(last_exp,n as i32),
            None => {
                if ch == '+' {
                    Express::Item(Item::Factor(Factor::Digit(Digit::PostiveSign))) 
                } else {
                    Express::Item(Item::Factor(Factor::Digit(Digit::NegativeSign))) 
                }
            }
        };
        num
    }   

    pub fn new(last_exp: Express, num: i32) -> Express {
        //更新 最终表达式
        let last_exp = match last_exp {
            Express::Nil => Express::Item(Item::to_item(num)),
            Express::Add(exp, fac) => {
                Express::Add(exp, Item::to_item(Digit::item_update(fac, num)))
            }
            Express::Min(exp, fac) => {
                Express::Min(exp, Item::to_item(Digit::item_update(fac, num)))
            }
            Express::Item(item) => match item {
                Item::PackExp(v) => Express::Item(Item::PackExp(v)),
                _ => Express::Item(Item::to_item(Digit::item_update(item, num))),
            },
        };
        return last_exp;
    }
    fn item_update(item: Item, num: i32) -> i32 {
        match item {
            Item::Factor(fac) => Digit::factor_update(fac, num),
            Item::Nil => num,
            _ => 0,
        }
    }
    fn factor_update(fac: Factor, num: i32) -> i32 {
        match fac {
            //期待得到值
            Factor::_Exprect => num,
            Factor::Digit(v) => {
                let new_v = match v {
                    Digit::PostiveSign => num,
                    Digit::NegativeSign => -num,
                    Digit::Num(last_v) => {
                        if last_v < 0 {
                            last_v * 10 - num
                        } else {
                            last_v * 10 + num
                        }
                    }
                };
                new_v
            }
            _ => 0,
        }
    }
}
