//use libm::{log, log2, log10, sin, cos, sqrt, exp};
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
    InsertNumber(f64),
}

#[derive(Debug)]
struct Calculator {
    buffer: [f64; 2],
    ops: String,
    result: f64,
    //error: Option<Error>,
    //theme: Theme,
}

impl Default for Calculator {
    fn default() -> Self {
        Calculator {
            buffer: [f64::NAN, f64::NAN],
            ops: String::from(""),
            result: f64::NAN,
        }
    }
}

fn view(calculator: &Calculator) -> Element<Message> {
    container(
        column![
            row![text(calculator.buffer[0]).size(20)],
            row![
                button(text("7").size(20)).on_press(Message::InsertNumber(7.0)),
                button(text("8").size(20)).on_press(Message::InsertNumber(8.0)),
                button(text("9").size(20)).on_press(Message::InsertNumber(9.0)),
            ],
            row![
                button(text("4").size(20)).on_press(Message::InsertNumber(4.0)),
                button(text("5").size(20)).on_press(Message::InsertNumber(5.0)),
                button(text("6").size(20)).on_press(Message::InsertNumber(6.0)),
            ],
            row![
                button(text("1").size(20)).on_press(Message::InsertNumber(1.0)),
                button(text("2").size(20)).on_press(Message::InsertNumber(2.0)),
                button(text("3").size(20)).on_press(Message::InsertNumber(3.0)),
            ],
            row![button(text("0").size(20)).on_press(Message::InsertNumber(0.0))],
        ]
        .spacing(10),
    )
    .padding(10)
    .into()
}

fn update(calculator: &mut Calculator, message: Message) -> Task<Message> {
    match message {
        Message::InsertValue1 => {
            calculator.buffer[0] = 1.09;
            Task::none()
        }
        Message::InsertNumber(f64) => {
            calculator.buffer[0] = f64;
            Task::none()
        }
        Message::InsertValue2 => {
            calculator.buffer[1] = 12.0;
            Task::none()
        }
        Message::InsertOperator => {
            calculator.ops = String::from("+");
            Task::none()
        }
    }
}
