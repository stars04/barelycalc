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
    InsertValue1,
    InsertValue2,
    InsertOperator,
    InsertNumber(i64),
    ClearBuffer,
    DecimalToggle,
}

#[derive(Debug)]
struct Calculator {
    values: [f64; 2],
    is_dec: bool,
    buffer: Vec<i64>,
    decbuf: Vec<i64>,
    ops: String,
    result: f64,
    //error: Option<Error>,
    //theme: Theme,
}

impl Default for Calculator {
    fn default() -> Self {
        Calculator {
            values: [f64::NAN, f64::NAN],
            is_dec: false,
            buffer: Vec::new(),
            decbuf: Vec::new(),
            ops: String::from(""),
            result: f64::NAN,
        }
    }
}

fn view(calculator: &Calculator) -> Element<Message> {
    container(
        column![
            row![
                text(if calculator.is_dec == false {
                    calculator.buffer.iter().fold(0, |acc, x| acc * 10 + x) as f64
                } else {
                    decimal_value(&calculator.buffer, &calculator.decbuf)
                })
                .size(20)
            ],
            row![
                button(text("7").size(20)).on_press(Message::InsertNumber(7)),
                button(text("8").size(20)).on_press(Message::InsertNumber(8)),
                button(text("9").size(20)).on_press(Message::InsertNumber(9)),
                button(text("CE").size(20)).on_press(Message::ClearBuffer),
            ],
            row![
                button(text("4").size(20)).on_press(Message::InsertNumber(4)),
                button(text("5").size(20)).on_press(Message::InsertNumber(5)),
                button(text("6").size(20)).on_press(Message::InsertNumber(6)),
            ],
            row![
                button(text("1").size(20)).on_press(Message::InsertNumber(1)),
                button(text("2").size(20)).on_press(Message::InsertNumber(2)),
                button(text("3").size(20)).on_press(Message::InsertNumber(3)),
            ],
            row![
                button(text("0").size(20)).on_press(Message::InsertNumber(0)),
                button(text(".").size(20)).on_press(Message::DecimalToggle),
            ],
        ]
        .spacing(10),
    )
    .padding(10)
    .into()
}

fn update(calculator: &mut Calculator, message: Message) -> Task<Message> {
    match message {
        Message::InsertValue1 => {
            calculator.buffer[0] = 1;
            Task::none()
        }
        Message::InsertNumber(i64) => {
            if calculator.is_dec == false {
                calculator.buffer.push(i64);
            } else {
                calculator.decbuf.push(i64);
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
            calculator.buffer = Vec::new();
            calculator.decbuf = Vec::new();
            calculator.is_dec = false;
            Task::none()
        }
        Message::InsertValue2 => {
            calculator.buffer[1] = 12;
            Task::none()
        }
        Message::InsertOperator => {
            calculator.ops = String::from("+");
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
