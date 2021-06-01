use crate::digit::Digit;
use crate::factor::Factor;
#[derive(PartialEq, Debug)]
pub enum Express {
    Nil,
    Add(Box<Express>, Factor),
    Min(Box<Express>, Factor),
    Factor(Factor),
}

impl Express {
    pub fn add_sign(last_exp: Express) -> Express {
        match last_exp {
            Express::Nil => Express::Factor(Factor::Digit(Digit::PostiveSign)),
            Express::Add(exp, fac) => match fac {
                Factor::Exprect => Express::Add(exp, Factor::Digit(Digit::PostiveSign)),
                v => Express::Add(Box::new(Express::Add(exp, v)), Factor::Exprect),
            },
            Express::Min(exp, fac) => match fac {
                Factor::Exprect => Express::Add(exp, Factor::Digit(Digit::PostiveSign)),
                v => Express::Add(Box::new(Express::Min(exp, v)), Factor::Exprect),
            },
            fac => Express::Add(Box::new(fac), Factor::Exprect),
        }
    }

    pub fn min_sign(last_exp: Express) -> Express {
        match last_exp {
            Express::Nil => Express::Factor(Factor::Digit(Digit::NegativeSign)),
            Express::Add(exp, fac) => match fac {
                Factor::Exprect => Express::Min(exp, Factor::Digit(Digit::PostiveSign)),
                v => Express::Min(Box::new(Express::Add(exp, v)), Factor::Exprect),
            },
            Express::Min(exp, fac) => match fac {
                Factor::Exprect => Express::Min(exp, Factor::Digit(Digit::PostiveSign)),
                v => Express::Min(Box::new(Express::Min(exp, v)), Factor::Exprect),
            },
            fac => Express::Min(Box::new(fac), Factor::Exprect),
        }
    }
}
