use super::{Data, Expr, Factor, Iter};

impl Factor {
    pub fn read_factor(iter: &mut Iter) -> Option<Factor> {
        let mut res = None;
        let data = iter.preview()?;
        if let Data::Sign(sign) = data {
            
            if sign == "(" {
                iter.next()?;

                let temp = Factor::Aera(Box::new(Expr::read_expr(iter)?));
                let data = iter.preview()?;
                if let Data::Sign(sign) = data {
                    if sign == ")" {
                        iter.next()?;
                        res = Some(temp);
                    }
                }
            } else if sign == "+" || sign == "-" {
                iter.next()?;
                let data = iter.preview()?;

                if let Data::NumInt(num) = data {
                    iter.next()?;
                    let num:i64=num.into();
                    let num:i64=(num) * if sign == "-" { -1 } else { 1 };
                    res = Some(Factor::Digit(num))
                }
            }
        } else if let Data::NumInt(num) = data {
            iter.next()?;
            res = Some(Factor::Digit(num.into()))
        } else if let Data::NumFloat(num) = data {
            iter.next()?;
            todo!()
        }
        res
    }
}

#[cfg(test)]
mod factor_test{
    use super::*;

    #[test]
    fn read_postive_num() {
        let mut iter=Iter::new(vec![Data::NumInt(112)]);
        let factor=Factor::read_factor(&mut iter).unwrap();

        assert_eq!(factor,Factor::Digit(112));
        assert_eq!(iter.next(),None);
    }
    #[test]
    fn read_negative_num() {
        let mut iter=Iter::new(vec![Data::Sign("-".to_string()),Data::NumInt(11)]);
        let factor=Factor::read_factor(&mut iter).unwrap();

        assert_eq!(factor,Factor::Digit(-11));
        assert_eq!(iter.next(),None);
    }

    #[test]
    fn read_no_num() {
        let mut iter=Iter::new(vec![Data::Sign("[".to_string())]);
        let factor=Factor::read_factor(&mut iter);

        assert_eq!(factor,None);
        assert_ne!(iter.next(),None);
    }
}