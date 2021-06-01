use crate::express::Express;
use crate::factor::Factor;
#[derive(PartialEq, Debug)]

pub enum Digit {
    PostiveSign,
    NegativeSign,
    Num(i32),
}

impl Digit {
    pub fn new(last_exp: Express, num: i32) -> Express {
        //更新 最终表达式
        let last_exp = match last_exp {
            Express::Nil => Express::Factor(Factor::to_factor(num)),
            Express::Add(exp, fac) => {
                Express::Add(exp, Factor::to_factor(Digit::factor_update(fac, num)))
            }
            Express::Min(exp, fac) => {
                Express::Min(exp, Factor::to_factor(Digit::factor_update(fac, num)))
            }
            Express::Factor(fac) => match fac {
                Factor::PackExp(v) => Express::Factor(Factor::PackExp(v)),
                _ => Express::Factor(Factor::Digit(Digit::Num(Digit::factor_update(fac, num)))),
            },
        };
        return last_exp;
    }

    fn factor_update(fac: Factor, num: i32) -> i32 {
        match fac {
            //期待得到值
            Factor::Exprect => num,
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
