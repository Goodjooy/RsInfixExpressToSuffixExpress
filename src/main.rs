use crate::digit::Digit;
use crate::express::Express;
use crate::factor::Factor;

mod digit;
mod express;
mod factor;

trait GerneratExp {
    fn get_sign();
    fn do_generate(char_iter: std::str::Chars);
}

fn main() {
    println!("Hello, world!");
    let s = 1 + -1;
    let exp = "1+23-22+(12-1+(12-11+(-22)))";
    let r = anayles(&mut exp.chars());
    println!("{:#?}", r)
}

fn anayles(in_str: &mut std::str::Chars) -> Option<Express> {
    let str_iter = in_str;
    let mut final_express = Express::Nil;
    loop {
        match str_iter.next() {
            Some(ch) => {
                //digit 类型
                if ch.is_digit(10) {
                    let num = ch.to_digit(10).unwrap() as i32;
                    final_express = Digit::new(final_express, num);
                } else if ch == '+' {
                    final_express = Express::add_sign(final_express);
                } else if ch == '-' {
                    final_express = Express::min_sign(final_express);
                } else if ch == '(' {
                    let new_exp = match anayles(str_iter) {
                        Some(exp) => exp,
                        None => return None,
                    };
                    let fac_ex = Factor::PackExp(Box::new(new_exp));
                    //更新 最终表达式
                    final_express = match final_express {
                        Express::Nil => Express::Factor(fac_ex),
                        Express::Add(exp, _fac) => Express::Add(exp, fac_ex),
                        Express::Min(exp, _fac) => Express::Min(exp, fac_ex),
                        Express::Factor(fac) => Express::Factor(fac),
                    }
                } else if ch == ')' {
                    break;
                }
            }
            None => break,
        }
        println!("{:?}", &final_express);
    }
    match &final_express {
        //Nothing in express
        Express::Nil => return None,
        // add express with no follow data
        Express::Add(_exp, fac) => {
            if *fac == Factor::Exprect {
                return None;
            }
        }
        Express::Min(_exp, fac) => {
            if *fac == Factor::Exprect {
                return None;
            }
        }
        Express::Factor(_fac) => {}
    }

    return Some(final_express);
}
