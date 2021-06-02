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
            Some(n) => Digit::new(last_exp, n as i32),
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
                Express::Add(exp, Item::update_item(Digit::item_update(&fac, num), fac))
            }
            Express::Min(exp, fac) => {
                Express::Min(exp, Item::update_item(Digit::item_update(&fac, num), fac))
            }
            Express::Item(item) => Express::Item(match item {
                Item::Produce(item, f) => {
                    Item::Produce(item, Factor::to_factor(Digit::factor_update(&f, num)))
                }
                Item::Divide(item, f) => {
                    Item::Divide(item, Factor::to_factor(Digit::factor_update(&f, num)))
                }
                Item::PackExp(v) => Item::PackExp(v),
                v => Item::to_item(Digit::item_update(&v, num)),
            }),
        };
        return last_exp;
    }
    fn item_update(item: &Item, num: i32) -> i32 {
        let fac_data = |fac: &Factor| -> i32 { Digit::factor_update(fac, num) };
        match item {
            Item::Factor(fac) => fac_data(fac),
            Item::Nil => num,
            Item::Produce(_b, f) => fac_data(f),
            Item::Divide(_b, f) => fac_data(f),
            Item::PackExp(_b) => num,
        }
    }
    fn factor_update(fac: &Factor, num: i32) -> i32 {
        match fac {
            //期待得到值
            Factor::Expect => num,
            Factor::Digit(v) => {
                let new_v = match v {
                    Digit::PostiveSign => num,
                    Digit::NegativeSign => -num,
                    Digit::Num(last_v) => {
                        if *last_v < 0 {
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
