use crate::digit::Digit;
use crate::factor::Factor;
use crate::item::Item;
#[derive(PartialEq, Debug)]
pub enum Express {
    Nil,
    Add(Box<Express>, Item),
    Min(Box<Express>, Item),
    Item(Item),
}

impl Express {
    pub fn add_sign(last_exp: Express) -> Express {
        match last_exp {
            Express::Nil => Express::Item(Item::Factor(Factor::Digit(Digit::PostiveSign))),
            Express::Add(exp, fac) => match fac {
                Item::Nil => Express::Add(exp, Item::Factor(Factor::Digit(Digit::PostiveSign))),
                v => Express::Add(Box::new(Express::Add(exp, v)), Item::Nil),
            },
            Express::Min(exp, fac) => match fac {
                Item::Nil => Express::Add(exp, Item::Factor(Factor::Digit(Digit::PostiveSign))),
                v => Express::Add(Box::new(Express::Min(exp, v)), Item::Nil),
            },
            fac => Express::Add(Box::new(fac), Item::Nil),
        }
    }

    pub fn min_sign(last_exp: Express) -> Express {
        match last_exp {
            Express::Nil => Express::Item(Item::Factor(Factor::Digit(Digit::NegativeSign))),
            Express::Add(exp, fac) => match fac {
                Item::Nil => Express::Min(exp, Item::Factor(Factor::Digit(Digit::PostiveSign))),
                v => Express::Min(Box::new(Express::Add(exp, v)), Item::Nil),
            },
            Express::Min(exp, fac) => match fac {
                Item::Nil => Express::Min(exp, Item::Factor(Factor::Digit(Digit::PostiveSign))),
                v => Express::Min(Box::new(Express::Min(exp, v)), Item::Nil),
            },
            fac => Express::Min(Box::new(fac), Item::Nil),
        }
    }
}
