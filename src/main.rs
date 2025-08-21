use libm::{cos, exp, log, sin, tan};
use std::f64::consts::PI;
//use std::collections::HashMap;
use corefunctions::round;
use iced::alignment::{Horizontal, Vertical};
use iced::widget::{button, column, container, row, text};
use iced::{Application, Element, Length, Settings, Task};
use std::io;
mod corefunctions;

fn main() -> iced::Result {
    iced::application("Calculator", update, view)
        .window_size(iced::Size::new(310.0, 505.0))
        .theme(|_s| iced::Theme::KanagawaDragon)
        .run()
}

#[derive(Debug)]
enum Error {
    Io(io::ErrorKind),
}

#[derive(Debug, Clone)]
enum Message {
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
        Message::InsertNumber(i128) => {
            match &calculator.ops {
                None => {
                    if calculator.is_dec == false {
                        calculator.buffer.push(i128);
                        calculator.values[0] = match vector_to_value(&calculator.buffer) {
                            Ok(f64) => f64,
                            Err(err) => {
                                calculator.error = Some(err);
                                f64::INFINITY
                            }
                        };
                    } else {
                        calculator.decbuf.push(i128);
                        calculator.values[0] =
                            match decimal_value(&calculator.buffer, &calculator.decbuf) {
                                Ok(f64) => f64,
                                Err(err) => {
                                    calculator.error = Some(err);
                                    f64::INFINITY
                                }
                            };
                    }
                }
                Some(Operator) => {
                    if calculator.is_dec == false {
                        calculator.buffer.push(i128);
                        calculator.values[1] = match vector_to_value(&calculator.buffer) {
                            Ok(f64) => f64,
                            Err(err) => {
                                calculator.error = Some(err);
                                f64::INFINITY
                            }
                        };
                    } else {
                        calculator.decbuf.push(i128);
                        calculator.values[1] =
                            match decimal_value(&calculator.buffer, &calculator.decbuf) {
                                Ok(f64) => f64,
                                Err(err) => {
                                    calculator.error = Some(err);
                                    f64::INFINITY
                                }
                            };
                    }
                }
            }
            calculator.display();
            Task::none()
        }
        Message::FunctionToggle(Function) => {
            calculator.function = Some(Function);
            calculator.display();
            Task::none()
        }
        Message::DecimalToggle => {
            match calculator.is_dec {
                false => calculator.is_dec = true,
                true => calculator.is_dec = false,
            }
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
        Message::InsertOperator(Operator) => {
            calculator.ops = Some(Operator);
            calculator.buffer = Vec::new();
            calculator.decbuf = Vec::new();
            calculator.is_dec = false;
            calculator.result = f64::NAN;
            Task::none()
        }
        Message::Calculate => {
            calculator.evaluate();
            calculator.display();
            calculator.values[0] = calculator.result;
            calculator.values[1] = 0.0;
            calculator.ops = None;
            calculator.function = None;
            Task::none()
        }
    }
}

//Cause for concern is the continued displaying of the final result when formatted as a percent
// To help address this,the path forward may be storing the 'final' result as a String and then clearing the actual result if needed
// this would allow the String to be formatted however I like while not being dependent on the function variable and other things to store the state.
// Generally even if the direct above doesn't solve the problem. Solution lies with altering disaplay final output such that it
// does not depend on the state of function and other fields of the Calculator Struct

impl Calculator {
    fn display(&mut self) {
        let mut _value = 0.0;
        if self.result.is_nan() == true {
            match &self.ops {
                None => _value = self.values[0],
                Some(Operator) => _value = self.values[1],
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
            _value = self.result;
            self.display_result = format!("{_value}");
        };
    }

    fn evaluate(&mut self) {
        let mut _calculator_result = match self.ops {
            Some(Operator::Add) => Ok(self.values[0] + self.values[0]),
            Some(Operator::Sub) => Ok(self.values[0] - self.values[0]),
            Some(Operator::Mul) => Ok(self.values[0] * self.values[0]),
            Some(Operator::Div) => Ok(self.values[0] / self.values[0]),
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
            Some(Function::Sin) => Ok(round(sin(angle(value)))),
            Some(Function::Cos) => Ok(round(cos(angle(value)))),
            Some(Function::Tan) => Ok(round(sin(angle(value))) / round(cos(angle(value)))),
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

    fn vector_to_value(&mut self) {
        //Need to implement this method in places where the old function was used
        let vector_value_1: Result<f64, Error> =
            Ok(self.buffer.iter().fold(0, |acc, x| acc * 10 + x) as f64);
        let value_1 = match vector_value_1 {
            Ok(f64) => f64,
            Err(err) => {
                self.error = Some(err);
                f64::INFINITY
            }
        };
        if self.is_dec == false {
            match self.ops {
                None => self.values[0] = value_1,
                _ => self.values[1] = value_1,
            }
        } else {
            let vector_value_2: Result<f64, Error> =
                Ok(self.decbuf.iter().fold(0, |acc, x| acc * 10 + x) as f64);
            let value_2 = match vector_value_2 {
                Ok(f64) => f64,
                Err(err) => {
                    self.error = Some(err);
                    f64::INFINITY
                }
            };
            let power_of_ten: f64 = 10_i128.pow(self.decbuf.len() as u32) as f64;
            let decimal_values = value_2 / power_of_ten;
            let decimal_result = value_1 / decimal_values;

            match self.ops {
                None => self.values[0] = decimal_result,
                _ => self.values[1] = decimal_result,
            }
        }
    }
}

fn angle(degrees: f64) -> f64 {
    degrees * (PI / 180.0)
}

fn function_calculation(func: &Option<Function>, value: f64) -> Result<f64, Error> {
    let calc_result = match func {
        Some(Function::Sin) => Ok(round(sin(angle(value)))),
        Some(Function::Cos) => Ok(round(cos(angle(value)))),
        Some(Function::Tan) => Ok(round(sin(angle(value))) / round(cos(angle(value)))),
        Some(Function::Exp) => Ok(exp(value)),
        Some(Function::Percent) => Ok(value / 100.0),
        Some(Function::Ln) => Ok(log(value)),
        None => Ok(value),
    };
    calc_result
}

fn display(value: &f64, function: &Option<Function>) -> String {
    match function {
        Some(Function::Sin) => format!("sin({value})"),
        Some(Function::Cos) => format!("cos({value})"),
        Some(Function::Tan) => format!("tan({value})"),
        Some(Function::Exp) => format!("exp({value})"),
        Some(Function::Percent) => format!("{value}%"),
        Some(Function::Ln) => format!("ln({value})"),
        None => format!("{value}"),
    }
}

fn vector_to_value(vector: &Vec<i128>) -> Result<f64, Error> {
    Ok(vector.iter().fold(0, |acc, x| acc * 10 + x) as f64)
}

fn decimal_value(vector_1: &Vec<i128>, vector_2: &Vec<i128>) -> Result<f64, Error> {
    let main_values = match vector_to_value(vector_1) {
        Ok(f64) => f64,
        Err(err) => {
            println!("{:?}", err);
            f64::INFINITY
        }
    };
    let mut decimal_values = match vector_to_value(vector_2) {
        Ok(f64) => f64,
        Err(err) => {
            println!("{:?}", err);
            f64::INFINITY
        }
    };

    let power_of_ten: f64 = 10_i128.pow(vector_2.len() as u32) as f64;
    decimal_values = decimal_values / power_of_ten;
    Ok(main_values + decimal_values)
}
