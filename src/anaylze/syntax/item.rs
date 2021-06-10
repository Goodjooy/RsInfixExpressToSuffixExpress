use super::{Data, Factor, Item, Iter};
use crate::anaylze::syntax::SubItem;

impl SubItem {
    pub fn new(iter: &mut Iter) -> Option<SubItem> {
        match iter.preview() {
            Some(sign) => match sign {
                Data::Sign(sign) => {
                    if sign == "*" || sign == "/" {
                        iter.next()?;
                        let factor = Factor::read_factor(iter)?;
                        let sub = Box::new(Self::new(iter)?);
                        Some(if sign == "*" {
                            SubItem::Pro(factor, sub)
                        } else {
                            SubItem::Div(factor, sub)
                        })
                    } else {
                        Some(SubItem::Nil)
                    }
                }
                _ => Some(SubItem::Nil),
            },
            None => Some(SubItem::Nil),
        }
    }
}

impl Item {
    pub fn read_item(iter: &mut Iter) -> Option<Item> {
        let factor = Factor::read_factor(iter)?;
        let sub = SubItem::new(iter)?;

        Some(Item::Some(factor, sub))
    }
}

// 1*2
#[cfg(test)]
mod item {
    use super::*;

    #[test]
    fn num_item() {
        let mut iter = Iter::new(vec![Data::NumInt(22), Data::Sign("+".to_string())]);
        let item = Item::read_item(&mut iter).unwrap();

        assert_eq!(item, Item::Some(Factor::Digit(22), SubItem::Nil));
        assert_eq!(iter.next(), Some(Data::Sign("+".to_string())));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn last_num_item() {
        let mut iter = Iter::new(vec![Data::NumInt(22)]);
        let item = Item::read_item(&mut iter).unwrap();

        assert_eq!(item, Item::Some(Factor::Digit(22), SubItem::Nil));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn items_item() {
        let mut iter = Iter::new(vec![
            Data::NumInt(22),
            Data::Sign("*".to_string()),
            Data::NumInt(12),
        ]);
        let item = Item::read_item(&mut iter).unwrap();

        assert_eq!(
            item,
            Item::Some(
                Factor::Digit(22),
                SubItem::Pro(Factor::Digit(12), Box::new(SubItem::Nil))
            )
        );
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn no_done_item() {
        let mut iter = Iter::new(vec![Data::NumInt(22), Data::Sign("*".to_string())]);
        let item = Item::read_item(&mut iter);

        assert_eq!(item, None);
        assert_eq!(iter.next(), None);
    }
    #[test]
    fn follow_items_item() {
        let mut iter = Iter::new(vec![
            Data::NumInt(22),
            Data::Sign("*".to_string()),
            Data::NumInt(12),
            Data::Sign("/".to_string()),
            Data::NumInt(33),
        ]);
        let item = Item::read_item(&mut iter).unwrap();

        assert_eq!(
            item,
            Item::Some(
                Factor::Digit(22),
                SubItem::Pro(
                    Factor::Digit(12),
                    Box::new(SubItem::Div(Factor::Digit(33), Box::new(SubItem::Nil)))
                )
            )
        );
        assert_eq!(iter.next(), None);
    }
}
