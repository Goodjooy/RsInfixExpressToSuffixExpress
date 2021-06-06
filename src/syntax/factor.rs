use super::{Data, Expr, Factor, Iter};

impl Factor {
    pub fn read_factor(iter: &mut Iter) -> Option<Factor> {
        let mut res = None;
        let data = iter.next()?;
        println!("{:?}",data);
        if let Data::Sign(sign) = data {
            if sign == "(" {
                let temp = Factor::Aera(Box::new(Expr::read_expr(iter)?));
                let data = iter.preview()?;
                println!("{:?}",data);
                if let Data::Sign(sign) = data {
                    if sign == ")" {
                        iter.next()?;
                        res = Some(temp);
                    }
                }
            } else if sign == "+" || sign == "-" {
                let data = iter.next()?;
                println!("{:?}",data);
                if let Data::NumInt(num) = data {
                    let num:i64=num.into();
                    let num:i64=(num) * if sign == "-" { -1 } else { 1 };
                    res = Some(Factor::Digit(num))
                }
            }
        } else if let Data::NumInt(num) = data {
            res = Some(Factor::Digit(num.into()))
        } else if let Data::NumFloat(num) = data {
            todo!()
        }
        res
    }
}
