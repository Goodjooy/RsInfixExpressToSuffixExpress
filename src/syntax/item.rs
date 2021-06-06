use super::{Data, Factor, Item, Iter};

impl Item {
    pub fn read_item(iter: &mut Iter) -> Option<Item> {
        //try read factor
        match Factor::read_factor(iter) {
            Some(factor) => {
                //let left = Item::Factor(factor);
                let sign = iter.preview().unwrap_or(Data::Nil);
                println!("{:?} - {:?}", factor, sign);
                let mut res = None;
                // 后面跟着 * 或者 /
                if let Data::Sign(sign) = sign {
                    if sign == "*" {
                        iter.next()?;
                        res = Some(Item::Pro(Box::new(Self::read_item(iter)?), factor))
                    } else if sign == "/" {
                        iter.next()?;
                        res = Some(Item::Div(Box::new(Self::read_item(iter)?), factor))
                    } else {
                        res = Some(Item::Factor(factor));
                    }
                } else {
                    res = Some(Item::Factor(factor))
                }
                res
            }
            None => None,
        }
    }
}

// 1*2
