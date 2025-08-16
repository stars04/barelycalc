//use libm::pow;
//use std::f64::consts::{PI, E};
//use std::collections::HashMap;
use iced::widget::{button, column, container, row, text};
use iced::{Application, Element, Length, Settings, Task};
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
struct Calculator {
    values: [f64; 2],
    is_dec: bool,
    is_op2: bool,
    function: Option<Function>,
    buffer: Vec<i64>,
    decbuf: Vec<i64>,
    ops: Option<Operator>,
    result: f64,
    error: Option<Error>,
}

impl Default for Calculator {
    fn default() -> Self {
        Calculator {
            values: [0.0, 0.0],
            is_dec: false,
            is_op2: false,
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
            row![
                text(if calculator.result.is_nan() == true {
                    match calculator.is_op2 {
                        false => display(&calculator.values[0], calculator.function.clone()),
                        true => display(&calculator.values[1], calculator.function.clone()),
                    }
                } else {
                    display(&calculator.result, calculator.function.clone())
                })
                .size(60)
            ]
            .padding(10),
            row![
                button(text("sin(x)").size(20)).on_press(Message::FunctionToggle(Function::Sin)),
                button(text("cos(x)").size(20)).on_press(Message::FunctionToggle(Function::Cos)),
                button(text("tan(x)").size(20)).on_press(Message::FunctionToggle(Function::Tan)),
            ]
            .spacing(3),
            row![
                button(text("CE").size(40))
                    .width(Length::Fixed(102.5))
                    .on_press(Message::ClearEverything),
                button(text("%").size(40)).width(Length::Fixed(52.5)),
                button(text("x").size(40))
                    .width(Length::Fixed(50.0))
                    .on_press(Message::InsertOperator(Operator::Mul)),
            ]
            .spacing(3),
            row![
                button(text("7").size(40))
                    .width(Length::Fixed(50.0))
                    .on_press(Message::InsertNumber(7)),
                button(text("8").size(40))
                    .width(Length::Fixed(50.0))
                    .on_press(Message::InsertNumber(8)),
                button(text("9").size(40))
                    .width(Length::Fixed(50.0))
                    .on_press(Message::InsertNumber(9)),
                button(text("/").size(40))
                    .width(Length::Fixed(50.0))
                    .on_press(Message::InsertOperator(Operator::Div)),
            ]
            .spacing(3),
            row![
                button(text("4").size(40))
                    .width(Length::Fixed(50.0))
                    .on_press(Message::InsertNumber(4)),
                button(text("5").size(40))
                    .width(Length::Fixed(50.0))
                    .on_press(Message::InsertNumber(5)),
                button(text("6").size(40))
                    .width(Length::Fixed(50.0))
                    .on_press(Message::InsertNumber(6)),
                button(text("-").size(40))
                    .width(Length::Fixed(50.0))
                    .on_press(Message::InsertOperator(Operator::Sub)),
            ]
            .spacing(3),
            row![
                button(text("1").size(40))
                    .width(Length::Fixed(50.0))
                    .on_press(Message::InsertNumber(1)),
                button(text("2").size(40))
                    .width(Length::Fixed(50.0))
                    .on_press(Message::InsertNumber(2)),
                button(text("3").size(40))
                    .width(Length::Fixed(50.0))
                    .on_press(Message::InsertNumber(3)),
                button(text("+").size(40))
                    .width(Length::Fixed(50.0))
                    .on_press(Message::InsertOperator(Operator::Add)),
            ]
            .spacing(3),
            row![
                button(text("±").size(40)).width(Length::Fixed(50.0)),
                button(text("0").size(40))
                    .width(Length::Fixed(50.0))
                    .on_press(Message::InsertNumber(0)),
                button(text(".").size(40))
                    .width(Length::Fixed(50.0))
                    .on_press(Message::DecimalToggle),
                button(text("=").size(40))
                    .width(Length::Fixed(50.0))
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

fn display(value: &f64, function: Option<Function>) -> String {
    match function {
        Some(Function::Sin) => format!("sin({value})"),
        Some(Function::Cos) => format!("cos({value})"),
        Some(Function::Tan) => format!("tan({value})"),
        Some(Function::Exp) => format!("exp({value})"),
        Some(Function::Ln) => format!("ln({value})"),
        _ => format!("{value}"),
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
