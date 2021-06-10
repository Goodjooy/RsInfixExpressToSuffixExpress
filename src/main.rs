use anaylze::syntax::Final;

use crate::anaylze::lexical::Lexical;
use crate::anaylze::syntax::Expr;
use crate::factor::Factor;
use crate::item::Item;
use crate::status::StatusMachin;

use crate::anaylze::syntax::ToSurfix;
use crate::digit::Digit;
use crate::express::Express;

mod digit;
mod express;
mod factor;
mod item;
mod status;

mod anaylze;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Enter Expression: ");
    let mut exp=String::new();
    
    std::io::stdin().read_line(&mut exp)?;


    let mut lexical = Lexical::init();
    lexical.do_lexical(&exp[..]);


    let expr = Expr::read_expr(&mut lexical.into_iter())
    .ok_or("Bad Express")?;
    let res = expr.to_surfix();
    println!("Orgin: {}", exp);
    println!("Surfix: {:?}", &res);
    println!("Result: {}", work(res).unwrap());

    Ok(())
}

fn work(data: Vec<Final>) -> Option<i64> {
    let mut stack = Vec::new();
    for d in data {
        match d {
            Final::Digit(num) => {
                stack.push(num);
            }
            Final::Add => {
                let right = stack.pop()?;
                let left = stack.pop()?;
                stack.push(left + right);
            }
            Final::Min => {
                let right = stack.pop()?;
                let left = stack.pop()?;
                stack.push(left - right);
            }
            Final::Pro => {
                let right = stack.pop()?;
                let left = stack.pop()?;
                stack.push(left * right);
            }
            Final::Div => {
                let right = stack.pop()?;
                let left = stack.pop()?;
                stack.push(left / right);
            }
        }
    }
    stack.pop()
}

fn anayles(in_str: &mut std::str::Chars, status: StatusMachin) -> (Option<Express>, StatusMachin) {
    let str_iter = in_str;
    let mut final_express = Express::Nil;
    let mut status = status;
    loop {
        match str_iter.next() {
            Some(ch) => {
                status = status.switch_status(ch);

                match status.get_status() {
                    status::Status::Init => {}
                    status::Status::LoadNum => {
                        final_express = Digit::to_digit(final_express, ch);
                    }
                    status::Status::Add => final_express = Express::add_sign(final_express),
                    status::Status::Min => final_express = Express::min_sign(final_express),
                    status::Status::Produce => {
                        final_express = Item::pack_opt(final_express, |item| -> Item {
                            Item::produce_sign(item)
                        })
                    }
                    status::Status::Divide => {
                        final_express = Item::pack_opt(final_express, |item| -> Item {
                            Item::divide_sign(item)
                        })
                    }
                    status::Status::SyntaxError => break,
                    status::Status::SubProcess => {
                        let new_exp = match anayles(str_iter, status) {
                            (Some(exp), s) => {
                                status = s;
                                exp
                            }
                            (None, s) => {
                                status = s;
                                break;
                            }
                        };
                        let fac_ex = Item::PackExp(Box::new(new_exp));
                        //更新 最终表达式
                        final_express = match final_express {
                            Express::Nil => Express::Item(fac_ex),
                            Express::Add(exp, _fac) => Express::Add(exp, fac_ex),
                            Express::Min(exp, _fac) => Express::Min(exp, fac_ex),
                            Express::Item(fac) => Express::Item({
                                match fac {
                                    Item::Nil => Item::Factor(Factor::PackItem(Box::new(fac_ex))),
                                    Item::Produce(exp, _f) => {
                                        Item::Produce(exp, Factor::PackItem(Box::new(fac_ex)))
                                    }
                                    Item::Divide(exp, _f) => {
                                        Item::Divide(exp, Factor::PackItem(Box::new(fac_ex)))
                                    }
                                    v => v,
                                }
                            }),
                        }
                    }
                    status::Status::ExitProcess => return (Some(final_express), status),
                }
            }
            None => break,
        }
        println!("{:?}", &final_express);
    }
    match status.is_recive() {
        true => (Some(final_express), status),
        false => (None, status),
    }
}
