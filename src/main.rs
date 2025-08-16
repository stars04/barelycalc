//use libm::pow;
//use std::f64::consts::{PI, E};
//use std::collections::HashMap;
use iced::widget::{button, column, container, row, text};
use iced::{Application, Element, Settings, Task};
use std::io;
mod corefunctions;
fn main() -> iced::Result {
    iced::application("Calculator", update, view)
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
    InsertNumber(i64),
    Calculate,
    ClearBuffer,
    DecimalToggle,
}

#[derive(Debug, Clone)]
enum Operator {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug)]
struct Calculator {
    values: [f64; 2],
    is_dec: bool,
    is_op2: bool,
    buffer: Vec<i64>,
    decbuf: Vec<i64>,
    ops: Option<Operator>,
    result: f64,
    error: Option<Error>,
    //theme: Theme,
}

impl Default for Calculator {
    fn default() -> Self {
        Calculator {
            values: [0.0, 0.0],
            is_dec: false,
            is_op2: false,
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
            row![
                text(if calculator.result.is_nan() == true {
                    match calculator.is_op2 {
                        false => calculator.values[0],
                        true => calculator.values[1],
                    }
                } else {
                    calculator.result
                })
                .size(30)
            ],
            row![
                button(text("CE").size(20))
                    .on_press(Message::ClearBuffer)
                    .padding(10),
                button(text("+/-").size(20)).padding(10),
                button(text("%").size(20)).padding(10),
                button(text("x").size(20))
                    .on_press(Message::InsertOperator(Operator::Mul))
                    .padding(10),
            ]
            .spacing(3),
            row![
                button(text("7").size(20))
                    .on_press(Message::InsertNumber(7))
                    .padding(10),
                button(text("8").size(20))
                    .on_press(Message::InsertNumber(8))
                    .padding(10),
                button(text("9").size(20))
                    .on_press(Message::InsertNumber(9))
                    .padding(10),
                button(text("/").size(20))
                    .on_press(Message::InsertOperator(Operator::Div))
                    .padding(10),
            ]
            .spacing(3),
            row![
                button(text("4").size(20))
                    .on_press(Message::InsertNumber(4))
                    .padding(10),
                button(text("5").size(20))
                    .on_press(Message::InsertNumber(5))
                    .padding(10),
                button(text("6").size(20))
                    .on_press(Message::InsertNumber(6))
                    .padding(10),
                button(text("-").size(20))
                    .on_press(Message::InsertOperator(Operator::Sub))
                    .padding(10),
            ]
            .spacing(3),
            row![
                button(text("1").size(20))
                    .on_press(Message::InsertNumber(1))
                    .padding(10),
                button(text("2").size(20))
                    .on_press(Message::InsertNumber(2))
                    .padding(10),
                button(text("3").size(20))
                    .on_press(Message::InsertNumber(3))
                    .padding(10),
                button(text("+").size(20))
                    .on_press(Message::InsertOperator(Operator::Add))
                    .padding(10),
            ]
            .spacing(3),
            row![
                button(text("0").size(20))
                    .on_press(Message::InsertNumber(0))
                    .padding(10),
                button(text(".").size(20))
                    .on_press(Message::DecimalToggle)
                    .padding(10),
                button(text("=").size(20))
                    .on_press(Message::Calculate)
                    .padding(10),
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
        Message::InsertNumber(i64) => {
            if calculator.is_dec == false {
                calculator.buffer.push(i64);
                match calculator.is_op2 {
                    false => {
                        calculator.values[0] =
                            calculator.buffer.iter().fold(0, |acc, x| acc * 10 + x) as f64
                    }
                    true => {
                        calculator.values[1] =
                            calculator.buffer.iter().fold(0, |acc, x| acc * 10 + x) as f64
                    }
                }
            } else {
                calculator.decbuf.push(i64);
                match calculator.is_op2 {
                    false => {
                        calculator.values[0] =
                            decimal_value(&calculator.buffer, &calculator.decbuf);
                    }
                    true => {
                        calculator.values[1] =
                            decimal_value(&calculator.buffer, &calculator.decbuf);
                    }
                }
            }
            Task::none()
        }
        Message::DecimalToggle => {
            match calculator.is_dec {
                false => calculator.is_dec = true,
                true => calculator.is_dec = false,
            }
            Task::none()
        }
        Message::ClearBuffer => {
            calculator.values = [0.0, f64::NAN];
            calculator.buffer = Vec::new();
            calculator.decbuf = Vec::new();
            calculator.is_op2 = false;
            calculator.is_dec = false;
            Task::none()
        }
        Message::InsertOperator(Operator) => {
            calculator.ops = Some(Operator);
            match calculator.is_op2 {
                false => calculator.is_op2 = true,
                true => calculator.is_op2 = false,
            }
            calculator.buffer = Vec::new();
            calculator.decbuf = Vec::new();
            Task::none()
        }
        Message::Calculate => {
            match calculator.ops {
                Some(Operator::Add) => {
                    calculator.result = calculator.values[0] + calculator.values[1]
                }
                Some(Operator::Sub) => {
                    calculator.result = calculator.values[0] - calculator.values[1]
                }
                Some(Operator::Mul) => {
                    calculator.result = calculator.values[0] * calculator.values[1]
                }
                Some(Operator::Div) => {
                    calculator.result = calculator.values[0] / calculator.values[1]
                }
                _ => calculator.result = f64::NAN,
            }
            Task::none()
        }
    }
}

fn decimal_value(vector_1: &Vec<i64>, vector_2: &Vec<i64>) -> f64 {
    let power_of_ten: f64 = 10_i64.pow(vector_2.len() as u32) as f64;
    let mut decimal_values = vector_2.iter().fold(0, |acc, x| acc * 10 + x) as f64;
    decimal_values = decimal_values / power_of_ten;
    let main_values = vector_1.iter().fold(0, |acc, x| acc * 10 + x) as f64;
    main_values + decimal_values
}

//fn value_to_insert(buffer_value: Option<f64>, decbuffer_value: Option<f64>) -> f64 {
//    let value_1: f64 = match bu
//}
