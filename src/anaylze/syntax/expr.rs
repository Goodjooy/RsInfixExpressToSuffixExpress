use super::{Data, Expr, Final, Item, Iter};
use crate::anaylze::syntax::SubExpr;

impl SubExpr {
    pub fn new(iter: &mut Iter) -> Option<SubExpr> {
        match iter.preview() {
            Some(sign) => {
                if let Data::Sign(sign) = sign {
                    if sign == "+" || sign == "-" {
                        iter.next()?;
                        let item = Item::read_item(iter)?;
                        let sub = Box::new(SubExpr::new(iter)?);
                        Some(if sign == "+" {
                            SubExpr::Add(item, sub)
                        } else {
                            SubExpr::Min(item, sub)
                        })
                    } else {
                        Some(SubExpr::Nil)
                    }
                } else {
                    Some(SubExpr::Nil)
                }
            }
            None => Some(SubExpr::Nil),
        }
    }
}

impl Expr {
    pub fn read_expr(iter: &mut Iter) -> Option<Expr> {
        let item = Item::read_item(iter)?;
        let sub = SubExpr::new(iter)?;

        Some(Expr::Some(item, sub))
    }
}

#[cfg(test)]
mod expr {
    use crate::anaylze::syntax::{Factor, SubItem};

    use super::*;

    #[test]
    fn signle_num() {
        let mut iter = Iter::new(vec![Data::NumInt(11)]);
        let expr = Expr::read_expr(&mut iter).unwrap();

        assert_eq!(
            expr,
            Expr::Some(Item::Some(Factor::Digit(11), SubItem::Nil), SubExpr::Nil)
        );
        assert_eq!(iter.next(), None);
    }
    #[test]
    fn signle_op_num() {
        let mut iter = Iter::new(vec![
            Data::NumInt(11),
            Data::Sign("+".to_string()),
            Data::NumInt(23),
            Data::Sign("-".to_string()),
            Data::NumInt(11),
        ]);
        let expr = Expr::read_expr(&mut iter).unwrap();

        assert_eq!(
            expr,
            Expr::Some(
                Item::Some(Factor::Digit(11), SubItem::Nil),
                SubExpr::Add(
                    Item::Some(Factor::Digit(23), SubItem::Nil),
                    Box::new(SubExpr::Min(
                        Item::Some(Factor::Digit(11), SubItem::Nil),
                        Box::new(SubExpr::Nil)
                    ))
                )
            )
        );
        assert_eq!(iter.next(), None);
    }
}
