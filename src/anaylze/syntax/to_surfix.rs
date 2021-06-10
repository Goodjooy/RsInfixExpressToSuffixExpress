use crate::anaylze::syntax::Factor;
use crate::anaylze::syntax::Final;
use crate::anaylze::syntax::ToSurfix;
use crate::Expr;

use super::Item;
use super::SubExpr;
use super::SubItem;

impl ToSurfix for Expr {
    fn to_surfix(self) -> Vec<Final> {
        let mut res = Vec::new();

        match self {
            Expr::Some(item, sub) => {
                res.extend(item.to_surfix());
                res.extend(sub.to_surfix());
            }
        }

        res
    }
}

impl ToSurfix for SubExpr {
    fn to_surfix(self) -> Vec<Final> {
        let mut res = Vec::new();
        match self {
            SubExpr::Add(item, sub) => {
                res.extend(item.to_surfix());
                res.push(Final::Add);
                res.extend(sub.to_surfix());
            }
            SubExpr::Min(item, sub) => {
                res.extend(item.to_surfix());
                res.push(Final::Min);
                res.extend(sub.to_surfix());
            }
            SubExpr::Nil => {}
        }
        res
    }
}

impl ToSurfix for Item {
    fn to_surfix(self) -> Vec<Final> {
        let mut res = Vec::new();
        match self {
            Item::Some(factor, sub) => {
                res.extend(factor.to_surfix());
                res.extend(sub.to_surfix());
            }
        }
        res
    }
}
impl ToSurfix for SubItem {
    fn to_surfix(self) -> Vec<Final> {
        let mut res = Vec::new();

        match self {
            SubItem::Pro(factor, sub) => {
                res.extend(factor.to_surfix());
                res.push(Final::Pro);
                res.extend(sub.to_surfix());
            }
            SubItem::Div(factor, sub) => {
                res.extend(factor.to_surfix());
                res.push(Final::Div);
                res.extend(sub.to_surfix());
            }
            SubItem::Nil => {}
        }

        res
    }
}

impl ToSurfix for Factor {
    fn to_surfix(self) -> Vec<Final> {
        let mut res = Vec::new();
        match self {
            Factor::Digit(num) => res.push(Final::Digit(num)),
            Factor::Sign(_) => todo!(),
            Factor::Aera(expr) => res.extend(expr.to_surfix()),
        }

        res
    }
}
