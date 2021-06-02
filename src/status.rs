#[derive(Clone)]
pub struct StatusMachin {
    status: Status,
    stack: Vec<StackInfo>,
}
#[derive(PartialEq, Clone)]
pub enum Status {
    Init,

    LoadNum,

    Add,
    Min,
    Produce,
    Divide,

    SubProcess,
    ExitProcess,

    SyntaxError,
}
#[derive(Clone)]
enum StackInfo {
    Parenthesis(Status), //小括号
    _Bracket(Status),     //中括号
    _Brace(Status),       //大括号
}

impl StatusMachin {
    pub fn new() -> StatusMachin {
        StatusMachin {
            status: Status::Init,
            stack: Vec::new(),
        }
    }

    pub fn switch_status(mut self, next_char: char) -> Self {
        match self.status {
            Status::Init => {
                //在初始状态下，读取到数字，或者数字前符号
                //进入读取数字状态
                if next_char.is_digit(10) || next_char == '+' || next_char == '-' {
                    self.status = Status::LoadNum;
                } else if next_char == '(' {
                    //左括号，记录
                    self.stack.push(StackInfo::Parenthesis(Status::Init));
                    self.status = Status::SubProcess;
                } else {
                    //其他情况都是异常
                    self.status = Status::SyntaxError;
                };
            }
            Status::SubProcess => {
                //在初始状态下，读取到数字，或者数字前符号
                //进入读取数字状态
                if next_char.is_digit(10) || next_char == '+' || next_char == '-' {
                    self.status = Status::LoadNum;
                } else if next_char == '(' {
                    //左括号，记录
                    self.stack.push(StackInfo::Parenthesis(Status::Init));
                    self.status = Status::SubProcess;
                } else {
                    //其他情况都是异常
                    self.status = Status::SyntaxError;
                };
            }
            Status::Add => {
                if next_char.is_digit(10) || next_char == '+' || next_char == '-' {
                    self.status = Status::LoadNum;
                } else if next_char == '(' {
                    self.status = Status::SubProcess;
                    self.stack.push(StackInfo::Parenthesis(Status::Add));
                } else {
                    //其他情况都是异常
                    self.status = Status::SyntaxError;
                };
            }
            Status::Min => {
                if next_char.is_digit(10) || next_char == '+' || next_char == '-' {
                    self.status = Status::LoadNum;
                } else if next_char == '(' {
                    self.status = Status::SubProcess;
                    self.stack.push(StackInfo::Parenthesis(Status::Min));
                } else {
                    //其他情况都是异常
                    self.status = Status::SyntaxError;
                };
            }
            Status::Produce => {
                if next_char.is_digit(10) || next_char == '+' || next_char == '-' {
                    self.status = Status::LoadNum;
                } else if next_char == '(' {
                    self.status = Status::SubProcess;
                    self.stack.push(StackInfo::Parenthesis(Status::Produce));
                } else {
                    //其他情况都是异常
                    self.status = Status::SyntaxError;
                };
            }
            Status::Divide => {
                if next_char.is_digit(10) || next_char == '+' || next_char == '-' {
                    self.status = Status::LoadNum;
                } else if next_char == '(' {
                    self.status = Status::SubProcess;
                    self.stack.push(StackInfo::Parenthesis(Status::Divide));
                } else {
                    //其他情况都是异常
                    self.status = Status::SyntaxError;
                };
            }
            Status::LoadNum => {
                if next_char.is_digit(10) {
                } else if next_char == '+' {
                    self.status = Status::Add;
                } else if next_char == '-' {
                    self.status = Status::Min;
                } else if next_char == '*' {
                    self.status = Status::Produce
                } else if next_char == '/' {
                    self.status = Status::Divide
                } else if next_char == ')' {
                    let stack = self.stack.pop();
                    match stack {
                        Some(info) => {
                            if let StackInfo::Parenthesis(_st) = info {
                                self.status = Status::ExitProcess;
                            } else {
                                self.status = Status::SyntaxError;
                            }
                        }
                        None => {
                            self.status = Status::SyntaxError;
                        }
                    }
                } else {
                    //其他情况都是异常
                    self.status = Status::SyntaxError;
                };
            }
            Status::SyntaxError => {}
            Status::ExitProcess => {//在初始状态下，读取到数字，或者数字前符号
                //进入读取数字状态
                println!("{}",next_char);

                 if next_char == '+' {
                    self.status = Status::Add;
                } else if next_char == '-' {
                    self.status = Status::Min;
                } else if next_char == '*' {
                    self.status = Status::Produce
                } else if next_char == '/' {
                    self.status = Status::Divide
                } else if next_char == ')' {
                    let stack = self.stack.pop();
                    match stack {
                        Some(info) => {
                            if let StackInfo::Parenthesis(_st) = info {
                                self.status = Status::ExitProcess;
                            } else {
                                self.status = Status::SyntaxError;
                            }
                        }
                        None => {
                            self.status = Status::SyntaxError;
                        }
                    }
                } else {
                    //其他情况都是异常
                    self.status = Status::SyntaxError;
                };},
        };

        self
    }
    pub fn is_recive(&self) -> bool {
        self.stack.len() == 0 && self.status != Status::SyntaxError
    }

    pub fn get_status(&self) -> &Status {
        &self.status
    }
}
