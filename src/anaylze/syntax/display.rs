use super::{Expr, Factor, Item, SubExpr, SubItem};
use core::fmt::Display;

impl Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::Some(item, sub) => write!(f, "{}{}", item, sub),
        }
    }
}

impl Display for SubExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SubExpr::Add(item, next) => {
                write!(f, "{}+{}", item, next)
            }
            SubExpr::Min(item, next) => {
                write!(f, "{}-{}", item, next)
            }
            SubExpr::Nil => write!(f, ""),
        }
    }
}

impl Display for Item {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Item::Some(factor, next) => {
                write!(f, "{}{}", factor, next)
            }
        }
    }
}

impl Display for SubItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SubItem::Pro(factor, next) => write!(f, "{}*{}", factor, next),
            SubItem::Div(factor, next) => write!(f, "{}/{}", factor, next),
            SubItem::Nil => write!(f, ""),
        }
    }
}

impl Display for Factor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Factor::Digit(num) => {
                write!(f, " <{}> ", num)
            }
            Factor::Sign(sign) => {
                write!(f, "<SIGN, {}>", sign)
            }
            Factor::Aera(expr) => {
                write!(f, "{}", expr)
            }
        }
    }
}
