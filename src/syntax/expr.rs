use super::{Data, Expr, Factor, Item, Iter};
/// expr 跟表达式
///  
///

impl Expr {
    /// 从解析列表中加载一个expr
    ///
    /// # Example
    /// ```rust
    ///
    /// ```

    pub fn read_expr(iter: &mut Iter) -> Option<Expr> {
        match Item::read_item(iter) {
            Some(item) => {
                let mut res = None;
                //let left = Expr::Item(item);
                let sign = iter.preview().unwrap_or(Data::Nil);
                println!("{:?} - {:?}", item, sign);
                if let Data::Sign(sign) = sign {
                    if sign == "+" {
                        iter.next()?;
                        res = Some(Expr::Add(Box::new(Self::read_expr(iter)?), item));
                    } else if sign == "-" {
                        iter.next()?;
                        res = Some(Expr::Min(Box::new(Self::read_expr(iter)?), item));
                    } else {
                        res = Some(Expr::Item(item))
                    }
                } else {
                    res = Some(Expr::Item(item))
                }
                res
            }
            None => None,
        }
    }
}
