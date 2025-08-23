use corefunctions::round;
use iced::alignment::{Horizontal, Vertical};
use iced::event::{self, Status};
use iced::keyboard::{Event::KeyPressed, Key, key::Named};
use iced::widget::{button, column, container, row, text};
use iced::{Application, Element, Length, Settings, Task};
use libm::{cos, exp, log, sin, tan};
use std::f64::consts::PI;
use std::io;
mod corefunctions;

fn main() -> iced::Result {
    iced::application("Calculator", update, view)
        .window_size(iced::Size::new(310.0, 505.0))
        .subscription(Calculator::subscription)
        .theme(|_s| iced::Theme::KanagawaDragon)
        .run()
}

#[derive(Debug)]
enum Error {
    Io(io::ErrorKind),
}

#[derive(Debug, Clone)]
enum Message {
    KeyBoardButton(Key),
    InsertOperator(Operator),
    InsertNumber(i128),
    Calculate,
    ClearEverything,
    DecimalToggle,
    FunctionToggle(Function),
}

#[derive(Debug, Clone)]
enum Function {
    Sin,
    Cos,
    Tan,
    Exp,
    Percent,
    Ln,
}

#[derive(Debug, Clone)]
enum Operator {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug)]
pub struct Calculator {
    values: [f64; 2],
    is_dec: bool,
    key_value: Option<Key>,
    buffer: Vec<i128>,
    decbuf: Vec<i128>,
    result: f64,
    display_result: String,
    function: Option<Function>,
    ops: Option<Operator>,
    error: Option<Error>,
}

impl Default for Calculator {
    fn default() -> Self {
        Calculator {
            values: [0.0, 0.0],
            is_dec: false,
            key_value: None,
            function: None,
            buffer: Vec::new(),
            decbuf: Vec::new(),
            ops: None,
            result: f64::NAN,
            display_result: String::from("0"),
            error: None,
        }
    }
}

fn view(calculator: &Calculator) -> Element<Message> {
    container(
        column![
            row![container(
                text(&calculator.display_result)
                    .size(50)
                    .align_x(Horizontal::Right)
                    .width(Length::Fixed(280.0))
            )]
            .padding(10),
            row![
                button(text("sin(x)").size(18).align_x(Horizontal::Center))
                    .width(Length::Fixed(70.0))
                    .on_press(Message::FunctionToggle(Function::Sin)),
                button(text("cos(x)").size(18).align_x(Horizontal::Center))
                    .width(Length::Fixed(70.0))
                    .on_press(Message::FunctionToggle(Function::Cos)),
                button(text("tan(x)").size(18).align_x(Horizontal::Center))
                    .width(Length::Fixed(70.0))
                    .on_press(Message::FunctionToggle(Function::Tan)),
                button(text("exp(x)").size(18).align_x(Horizontal::Center))
                    .width(Length::Fixed(70.0))
                    .on_press(Message::FunctionToggle(Function::Exp)),
            ]
            .spacing(3),
            row![
                button(text("CE").size(40))
                    .width(Length::Fixed(142.5))
                    .on_press(Message::ClearEverything),
                button(text("%").size(40).align_x(Horizontal::Center))
                    .width(Length::Fixed(70.0))
                    .on_press(Message::FunctionToggle(Function::Percent)),
                button(text("x").size(40).align_x(Horizontal::Center))
                    .width(Length::Fixed(70.0))
                    .on_press(Message::InsertOperator(Operator::Mul)),
            ]
            .spacing(3),
            row![
                button(text("7").size(40).align_x(Horizontal::Center))
                    .width(Length::Fixed(70.0))
                    .on_press(Message::InsertNumber(7)),
                button(text("8").size(40).align_x(Horizontal::Center))
                    .width(Length::Fixed(70.0))
                    .on_press(Message::InsertNumber(8)),
                button(text("9").size(40).align_x(Horizontal::Center))
                    .width(Length::Fixed(70.0))
                    .on_press(Message::InsertNumber(9)),
                button(text("/").size(40).align_x(Horizontal::Center))
                    .width(Length::Fixed(70.0))
                    .on_press(Message::InsertOperator(Operator::Div)),
            ]
            .spacing(3),
            row![
                button(text("4").size(40).align_x(Horizontal::Center))
                    .width(Length::Fixed(70.0))
                    .on_press(Message::InsertNumber(4)),
                button(text("5").size(40).align_x(Horizontal::Center))
                    .width(Length::Fixed(70.0))
                    .on_press(Message::InsertNumber(5)),
                button(text("6").size(40).align_x(Horizontal::Center))
                    .width(Length::Fixed(70.0))
                    .on_press(Message::InsertNumber(6)),
                button(text("-").size(40).align_x(Horizontal::Center))
                    .width(Length::Fixed(70.0))
                    .on_press(Message::InsertOperator(Operator::Sub)),
            ]
            .spacing(3),
            row![
                button(text("1").size(40).align_x(Horizontal::Center))
                    .width(Length::Fixed(70.0))
                    .on_press(Message::InsertNumber(1)),
                button(text("2").size(40).align_x(Horizontal::Center))
                    .width(Length::Fixed(70.0))
                    .on_press(Message::InsertNumber(2)),
                button(text("3").size(40).align_x(Horizontal::Center))
                    .width(Length::Fixed(70.0))
                    .on_press(Message::InsertNumber(3)),
                button(text("+").size(40).align_x(Horizontal::Center))
                    .width(Length::Fixed(70.0))
                    .on_press(Message::InsertOperator(Operator::Add)),
            ]
            .spacing(3),
            row![
                button(text("±").size(40).align_x(Horizontal::Center)).width(Length::Fixed(70.0)),
                button(text("0").size(40).align_x(Horizontal::Center))
                    .width(Length::Fixed(70.0))
                    .on_press(Message::InsertNumber(0)),
                button(text(".").size(40).align_x(Horizontal::Center))
                    .width(Length::Fixed(70.0))
                    .on_press(Message::DecimalToggle),
                button(text("=").size(40).align_x(Horizontal::Center))
                    .width(Length::Fixed(70.0))
                    .on_press(Message::Calculate),
            ]
            .spacing(3),
        ]
        .spacing(6),
    )
    .padding(10)
    .into()
}

fn update(calculator: &mut Calculator, message: Message) -> Task<Message> {
    match message {
        Message::KeyBoardButton(key) => {
            calculator.key_value = Some(key);
            calculator.insert_or_remove_value(None);
            Task::none()
        }
        Message::InsertNumber(i128) => {
            calculator.insert_or_remove_value(Some(i128));
            calculator.vector_to_value();
            calculator.display();
            Task::none()
        }
        Message::FunctionToggle(function) => {
            calculator.function = Some(function);
            calculator.display();
            Task::none()
        }
        Message::DecimalToggle => {
            match calculator.is_dec {
                false => calculator.is_dec = true,
                true => calculator.is_dec = false,
            }
            calculator.display_result.push_str(".0");
            Task::none()
        }
        Message::ClearEverything => {
            calculator.values = [0.0, 0.0];
            calculator.buffer = Vec::new();
            calculator.decbuf = Vec::new();
            calculator.result = f64::NAN;
            calculator.display_result = String::from("0");
            calculator.ops = None;
            calculator.function = None;
            calculator.is_dec = false;
            Task::none()
        }
        Message::InsertOperator(operator) => {
            calculator.ops = Some(operator);
            calculator.buffer = Vec::new();
            calculator.decbuf = Vec::new();
            calculator.is_dec = false;
            calculator.result = f64::NAN;
            calculator.display();
            Task::none()
        }
        Message::Calculate => {
            calculator.evaluate();
            calculator.display();
            calculator.values[0] = calculator.result;
            calculator.values[1] = 0.0;
            calculator.buffer = Vec::new();
            calculator.decbuf = Vec::new();
            calculator.ops = None;
            calculator.function = None;
            Task::none()
        }
    }
}

impl Calculator {
    fn display(&mut self) {
        let mut _value = String::new();
        if self.result.is_nan() == true {
            match &self.ops {
                None => _value = format!("{}", self.values[0]),
                Some(operator) => {
                    let operator_string = match operator {
                        Operator::Add => "+".to_string(),
                        Operator::Sub => "-".to_string(),
                        Operator::Mul => "x".to_string(),
                        Operator::Div => "/".to_string(),
                    };
                    _value = format!("{} {operator_string} {}", self.values[0], self.values[1])
                }
            }
            match &self.function {
                Some(Function::Sin) => self.display_result = format!("sin({_value})"),
                Some(Function::Cos) => self.display_result = format!("cos({_value})"),
                Some(Function::Tan) => self.display_result = format!("tan({_value})"),
                Some(Function::Exp) => self.display_result = format!("exp({_value})"),
                Some(Function::Percent) => self.display_result = format!("{_value}%"),
                Some(Function::Ln) => self.display_result = format!("ln({_value})"),
                None => self.display_result = format!("{_value}"),
            }
        } else {
            _value = format!("{}", self.result);
            self.display_result = format!("{_value}");
        };
    }

    fn evaluate(&mut self) {
        let mut _calculator_result = match self.ops {
            Some(Operator::Add) => Ok(self.values[0] + self.values[1]),
            Some(Operator::Sub) => Ok(self.values[0] - self.values[1]),
            Some(Operator::Mul) => Ok(self.values[0] * self.values[1]),
            Some(Operator::Div) => Ok(self.values[0] / self.values[1]),
            None => Ok(self.values[0]),
        };
        let value = match _calculator_result {
            Ok(f64) => f64,
            Err(err) => {
                self.error = Some(err);
                f64::INFINITY
            }
        };
        let calc_result = match self.function {
            Some(Function::Sin) => Ok(round(sin(self.angle(value)))),
            Some(Function::Cos) => Ok(round(cos(self.angle(value)))),
            Some(Function::Tan) => {
                Ok(round(sin(self.angle(value))) / round(cos(self.angle(value))))
            }
            Some(Function::Exp) => Ok(exp(value)),
            Some(Function::Percent) => Ok(value / 100.0),
            Some(Function::Ln) => Ok(log(value)),
            None => Ok(value),
        };
        self.result = match calc_result {
            Ok(f64) => f64,
            Err(err) => {
                self.error = Some(err);
                f64::INFINITY
            }
        };
    }

    fn angle(&self, degrees: f64) -> f64 {
        degrees * (PI / 180.0)
    }

    fn insert_or_remove_value(&mut self, integer: Option<i128>) {
        match integer {
            Some(i128) => match self.is_dec {
                false => {
                    self.buffer.push(i128);
                    self.vector_to_value();
                    self.display();
                }
                true => {
                    self.decbuf.push(i128);
                    self.vector_to_value();
                    self.display();
                }
            },
            None => match self.key_value.as_ref() {
                Some(Key::Character(c)) => match c.parse::<i128>() {
                    Ok(i128) => self.insert_or_remove_value(Some(i128)),
                    Err(err) => {
                        println!(
                            "Not an integer when parsed {:?}\n checking if key is an operator",
                            err
                        );
                        let possible_operator = match self.key_value.as_ref() {
                            Some(Key::Character(c)) => c.as_ref(),
                            _ => "",
                        };
                        if "+-*/".contains(possible_operator) {
                            match possible_operator {
                                "+" => self.ops = Some(Operator::Add),
                                "-" => self.ops = Some(Operator::Sub),
                                "*" => self.ops = Some(Operator::Mul),
                                "/" => self.ops = Some(Operator::Div),
                                _ => println!("No match in ops"),
                            }
                            self.buffer = Vec::new();
                            self.decbuf = Vec::new();
                            self.is_dec = false;
                            self.result = f64::NAN;
                            if possible_operator == "*" {
                                self.display_result.push_str(&format!(" x"));
                            } else {
                                self.display_result
                                    .push_str(&format!(" {}", possible_operator));
                            }
                        } else if possible_operator == "." {
                            match self.is_dec {
                                false => {
                                    self.is_dec = true;
                                    self.display_result.push_str(".0")
                                }
                                true => {
                                    self.is_dec = false;
                                }
                            }
                        }
                    }
                },
                Some(Key::Named(Named::Backspace)) => {
                    match self.is_dec {
                        false => {
                            self.buffer.pop();
                            if self.buffer == [] && !self.ops.is_none() {
                                self.ops = None;
                            } else if self.buffer == []
                                && self.ops.is_none()
                                && self.values[0] == 0.0
                            {
                                self.function = None;
                            } else {
                                self.vector_to_value();
                            }
                        }
                        true => {
                            self.decbuf.pop();
                            if self.decbuf == [] {
                                self.is_dec = false;
                            } else {
                                self.vector_to_value();
                            }
                        }
                    }
                    self.display();
                }
                Some(Key::Named(Named::Enter)) => {
                    self.evaluate();
                    self.display();
                    self.values[0] = self.result;
                    self.values[1] = 0.0;
                    self.buffer = Vec::new();
                    self.decbuf = Vec::new();
                    self.ops = None;
                    self.function = None;
                }
                _ => {
                    println!("No key was pressed");
                }
            },
        }
    }

    fn vector_to_value(&mut self) {
        //Need to implement this method in places where the old function was used
        let value_1 = self.buffer.iter().fold(0, |acc, x| acc * 10 + x) as f64;

        if self.is_dec == false && self.decbuf == Vec::new() {
            match &self.ops {
                None => self.values[0] = value_1,
                Some(_operator) => self.values[1] = value_1,
            }
        } else if self.is_dec == true || self.decbuf != Vec::new() {
            let value_2 = self.decbuf.iter().fold(0, |acc, x| acc * 10 + x) as f64;

            let power_of_ten: f64 = 10_i128.pow(self.decbuf.len() as u32) as f64;
            let decimal_values = value_2 / power_of_ten;
            let decimal_result = value_1 + decimal_values;

            match &self.ops {
                None => self.values[0] = decimal_result,
                Some(_operator) => self.values[1] = decimal_result,
            }
        }
    }

    fn subscription(&self) -> iced::Subscription<Message> {
        event::listen_with(|event, status, _| match (event, status) {
            (
                iced::Event::Keyboard(KeyPressed {
                    key: _,
                    modified_key,
                    physical_key: _,
                    location: _,
                    modifiers: _,
                    text: _,
                }),
                Status::Ignored,
            ) => Some(Message::KeyBoardButton(modified_key)),
            _ => None,
        })
    }
}
