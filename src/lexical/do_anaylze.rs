use super::{Lexical, LexicalData, LexicalIter};

const SINGLE_SIGNS: [&str; 9] = ["{", "}", "[", "]", "(", ")", ":", ".", ";"];
const EXCPT_SIGNS: [&str; 5] = [",", "$", "#", "@", "~"];
impl Lexical {
    fn check_keyword(&self, key: &String) -> Option<LexicalData> {
        Some((self.keyword.get(key)?).clone())
    }
    fn check_vaild_sign(&self, key: &String) -> bool {
        self.sign_set.contains(key)
    }

    pub fn do_lexical(&mut self, in_data: &'static str) -> Option<Self> {
        let mut iter = LexicalIter::new(in_data);

        loop {
            clear_space(&mut iter)?;
            let ch = iter.next()?;
            //数字开头
            if ch.is_digit(10) {
                self.res_list.push(digit_read(ch, &mut iter)?)
            }
            //字母，下划线开头
            else if ch.is_ascii_alphabetic() || ch == '_' {
                self.res_list
                    .push(alphabet_read(ch, &mut iter, |s| -> Option<LexicalData> {
                        self.check_keyword(s)
                    })?)
            } else {
                self.res_list.push(load_sign(ch, &mut iter, |s| -> bool {
                    self.check_vaild_sign(s)
                })?);
            }
        }
    }
}

fn digit_read(ch: char, iter: &mut LexicalIter) -> Option<LexicalData> {
    let mut is_float = false;

    let mut number = if ch == '.' {
        is_float = true;
        0
    } else {
        ch.to_digit(10)?
    };
    let mut f_number = number.to_string();
    if is_float {
        f_number.push('.');
    }
    loop {
        match iter.get_preview() {
            Some(pre_ch) => {
                if !is_float {
                    if pre_ch == '.' {
                        is_float = true;
                        f_number.push('.');
                    } else if pre_ch.is_digit(10) {
                        let n = pre_ch.to_digit(10)?;
                        number = number * 10 + n;
                        f_number.push(pre_ch);
                    } else {
                        break;
                    }
                } else {
                    if pre_ch.is_digit(10) {
                        f_number.push(pre_ch);
                    } else {
                        break;
                    }
                }
            }
            None => break,
        }
        iter.next();
    }
    if is_float {
        Some(LexicalData::NumFloat(
            f_number.parse().unwrap_or_else(|s| -> f64 { 0.0 }),
        ))
    } else {
        Some(LexicalData::NumInt(number))
    }
}

fn alphabet_read<F>(ch: char, iter: &mut LexicalIter, check_keyword: F) -> Option<LexicalData>
where
    F: Fn(&String) -> Option<LexicalData>,
{
    let mut word = String::from(ch);
    loop {
        match iter.get_preview() {
            Some(ch) => {
                if ch.is_ascii_alphabetic() || ch.is_digit(10) || ch == '_' {
                    word.push(ch);
                } else {
                    break;
                }
                iter.next();
            }
            None => break,
        }
    }
    check_keyword(&word).or_else(|| Some(LexicalData::Var(word)))
}

fn load_sign<F>(ch: char, iter: &mut LexicalIter, check: F) -> Option<LexicalData>
where
    F: Fn(&String) -> bool,
{
    let mut sign = String::from(ch);
    match  iter.get_preview(){
        Some(s) => {
            sign.push(s)
        },
        None => {},
    }
    //valid sign add
    if check(&sign) {
        Some(LexicalData::Sign(sign))
    } else {
        sign.pop()?;
        if check(&sign) {
            Some(LexicalData::Sign(sign))
        } else {
            None
        }
    }
}

fn clear_comment(iter: &mut LexicalIter) -> Option<()> {
    let sign = iter.get_preview()?;
    if sign == '/' {
        iter.next()?;
        let sign = iter.get_preview()?;
        if sign == '/' {
            while iter.get_preview()? != '\n' {
                iter.next()?;
            }
            iter.next()?;
        } else if sign == '*' {
            while !(iter.next()? == '*' && iter.get_preview()? == '/') {}
            iter.next()?;
        }
    }

    Some(())
}
fn clear_space(iter: &mut LexicalIter) -> Option<()> {
    loop {
        let ch = iter.get_preview()?;
        if ch.is_whitespace() {
            iter.next()?;
        } else {
            break;
        }
    }
    Some(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_clear_space() {
        let mut iter = LexicalIter::new("  aa   \n\t\t a");
        clear_space(&mut iter).unwrap();

        assert_eq!(iter.next().unwrap(), 'a');
        assert_eq!(iter.next().unwrap(), 'a');

        clear_space(&mut iter).unwrap();

        assert_eq!(iter.next().unwrap(), 'a');
    }
    #[test]
    fn test_clear_common() {
        let mut iter = LexicalIter::new("/* 爱爸爸 \n\n*/int//挺好的\nfloat");
        clear_comment(&mut iter);

        assert_eq!(iter.next().unwrap(), 'i');
        assert_eq!(iter.next().unwrap(), 'n');
        assert_eq!(iter.next().unwrap(), 't');

        clear_comment(&mut iter);

        assert_eq!(iter.next().unwrap(), 'f');
        assert_eq!(iter.next().unwrap(), 'l');
        assert_eq!(iter.next().unwrap(), 'o');
        assert_eq!(iter.next().unwrap(), 'a');
        assert_eq!(iter.next().unwrap(), 't');
    }
    #[test]
    fn test_load_sign() {
        let mut iter = LexicalIter::new("+={11+22}");
        let temp = |s: &String| -> bool { s == "+=" || s == "{" };
        let ch = iter.next().unwrap();
        assert_eq!(
            load_sign(ch, &mut iter, temp).unwrap(),
            LexicalData::Sign("+=".to_string())
        );
        let ch = iter.next().unwrap();
        assert_eq!(
            load_sign(ch, &mut iter, temp).unwrap(),
            LexicalData::Sign("{".to_string())
        )
    }

    #[test]
    fn test_load_var() {
        let temp = |key: &String| -> Option<LexicalData> { None };
        let mut iter = LexicalIter::new("_goods Avvda_ds Dads11");
        clear_space(&mut iter);
        let ch = iter.next().unwrap();
        assert_eq!(
            alphabet_read(ch, &mut iter, temp).unwrap(),
            LexicalData::Var("_goods".to_string())
        );
        clear_space(&mut iter);
        let ch = iter.next().unwrap();
        assert_eq!(
            alphabet_read(ch, &mut iter, temp).unwrap(),
            LexicalData::Var("Avvda_ds".to_string())
        );
        clear_space(&mut iter);
        let ch = iter.next().unwrap();
        assert_eq!(
            alphabet_read(ch, &mut iter, temp).unwrap(),
            LexicalData::Var("Dads11".to_string())
        );
    }

    #[test]
    fn test_read_digit() {
        let mut iter = LexicalIter::new("114145 21.33 333.");

        clear_space(&mut iter);
        let ch = iter.next().unwrap();
        assert_eq!(
            digit_read(ch, &mut iter).unwrap(),
            LexicalData::NumInt(114145)
        );

        clear_space(&mut iter);
        let ch = iter.next().unwrap();
        assert_eq!(
            digit_read(ch, &mut iter).unwrap(),
            LexicalData::NumFloat(21.33)
        );

        clear_space(&mut iter);
        let ch = iter.next().unwrap();
        assert_eq!(
            digit_read(ch, &mut iter).unwrap(),
            LexicalData::NumFloat(333.)
        );
    }
}
