use crate::{express::Express, factor::Factor};

#[derive(PartialEq, Debug)]
pub enum Item {
    Nil,
    Produce(Box<Item>, Factor),
    Divide(Box<Item>, Factor),
    Factor(Factor),

    PackExp(Box<Express>),
}

impl Item {
    pub fn update_item(num:i32,item:Item)->Item{
        match item {
            Item::Produce(a, _f) => Item::Produce(a,Factor::to_factor(num)),
            Item::Divide(a, _f) => Item::Divide(a,Factor::to_factor(num)),
            _ =>Item::to_item(num),
            
        }
    }
    pub fn to_item(num: i32) -> Item {
        Item::Factor(Factor::to_factor(num))
    }

    pub fn pack_opt<F>(last_exp: Express, fun: F) -> Express
    where
        F: Fn(Item) -> Item,
    {
        
            match last_exp {
                Express::Nil => Express::Nil,
                Express::Add(a, f) => {
                    Express::Add(a,fun(f))
                },
                Express::Min(a,f) => {Express::Min(a,fun(f))},
                Express::Item(f) => Express::Item(fun(f)),
            }

        
    }

    pub fn produce_sign(last_exp: Item) -> Item {
        Item::Produce(Box::new(last_exp), Factor::Expect)
    }
    pub fn divide_sign(last_exp: Item) -> Item {
        Item::Divide(Box::new(last_exp), Factor::Expect)
    }
}
