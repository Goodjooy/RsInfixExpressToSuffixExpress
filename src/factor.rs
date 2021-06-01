use crate::express::Express;
use crate::Digit;
#[derive(PartialEq, Debug)]
pub enum Factor {
    Exprect,
    Digit(Digit),
    PackExp(Box<Express>),
}

impl Factor {
    pub fn to_factor(num: i32) -> Factor {
        Factor::Digit(Digit::Num(num))
    }
}
