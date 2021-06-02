use crate::Digit;
use crate::item::Item;
#[derive(PartialEq, Debug)]
pub enum Factor {
    Expect,
    Digit(Digit),
    PackItem(Box<Item>),
}

impl Factor {
    pub fn to_factor(num: i32) -> Factor {
        Factor::Digit(Digit::Num(num))
    }
}
