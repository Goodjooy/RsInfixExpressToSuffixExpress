use crate::{express::Express, factor::Factor};

#[derive(PartialEq, Debug)]
pub enum Item {
    Nil,
    _Produce(Box<Item>,Factor),
    _Divide(Box<Item>,Factor),
    Factor(Factor),

    PackExp(Box<Express>)
}

impl Item {
    pub fn to_item(num:i32)->Item{
        Item::Factor(Factor::to_factor(num))
    }
}