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
            error: None,
        }
    }
}

fn view(calculator: &Calculator) -> Element<Message> {
    container(
        column![
            row![container(
                text(if calculator.result.is_nan() == true {
                    match &calculator.ops {
                        None => display(&calculator.values[0], &calculator.function),
                        Some(Operator) => display(&calculator.values[1], &calculator.function),
                    }
                } else {
                    display(&calculator.result, &calculator.function)
                })
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
                button(text("%").size(40).align_x(Horizontal::Center)).width(Length::Fixed(70.0)),
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
                        calculator.values[1] =
                            calculator.buffer.iter().fold(0, |acc, x| acc * 10 + x) as f64;
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
            Task::none()
        }
        Message::FunctionToggle(Function) => {
            calculator.function = Some(Function);
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
            match calculator.ops {
                Some(Operator::Add) => {
                    calculator.result = match function_calculation(
                        &calculator.function,
                        calculator.values[0] + calculator.values[1],
                    ) {
                        Ok(f64) => f64,
                        Err(err) => {
                            calculator.error = Some(err);
                            f64::INFINITY
                        }
                    }
                }
                Some(Operator::Sub) => {
                    calculator.result = match function_calculation(
                        &calculator.function,
                        calculator.values[0] - calculator.values[1],
                    ) {
                        Ok(f64) => f64,
                        Err(err) => {
                            calculator.error = Some(err);
                            f64::INFINITY
                        }
                    }
                }
                Some(Operator::Mul) => {
                    calculator.result = match function_calculation(
                        &calculator.function,
                        calculator.values[0] * calculator.values[1],
                    ) {
                        Ok(f64) => f64,
                        Err(err) => {
                            calculator.error = Some(err);
                            f64::INFINITY
                        }
                    }
                }
                Some(Operator::Div) => {
                    calculator.result = match function_calculation(
                        &calculator.function,
                        calculator.values[0] / calculator.values[1],
                    ) {
                        Ok(f64) => f64,
                        Err(err) => {
                            calculator.error = Some(err);
                            f64::INFINITY
                        }
                    }
                }
                _ => {
                    calculator.result =
                        match function_calculation(&calculator.function, calculator.values[0]) {
                            Ok(f64) => f64,
                            Err(err) => {
                                calculator.error = Some(err);
                                f64::INFINITY
                            }
                        }
                }
            }
            calculator.values[0] = calculator.result;
            calculator.values[1] = 0.0;
            calculator.ops = None;
            calculator.function = None;
            Task::none()
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

//fn value_to_insert(buffer_value: Option<f64>, decbuffer_value: Option<f64>) -> f64 {
//    let value_1: f64 = match bu
//}
