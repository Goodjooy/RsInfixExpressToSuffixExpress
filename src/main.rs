use crate::item::Item;
use crate::status::StatusMachin;

use crate::digit::Digit;
use crate::express::Express;

mod digit;
mod express;
mod factor;
mod item;
mod status;

trait GerneratExp {
    fn get_sign();
    fn do_generate(char_iter: std::str::Chars);
}

fn main() {
    println!("Hello, world!");
    let exp = "1+23-22+(12-1+(12-11+(-22)))";
    let status: StatusMachin = StatusMachin::new();
    let (r, _) = anayles(&mut exp.chars(), status);
    println!("{:#?}", r)
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
                    status::Status::Produce => todo!(),
                    status::Status::Divide => todo!(),
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
                            Express::Item(fac) => Express::Item(fac),
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
