//use libm::{log, log2, log10, sin, cos, sqrt, exp};
//use std::f64::consts::{PI, E};
//use std::collections::HashMap;
use iced::{Application,Task,Settings};
use iced::widget::{text};
use iced::{color,Element};
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

#[derive(Debug)]
enum Message {
    InsertValue1,
    InsertValue2,
    InsertOperator,
}
//#[derive(Debug)]
//enum Ops{
//    Add,
//    Sub,
//    Div,
//    Mul,
//}

#[derive(Debug, Default)]
struct Calculator {
    buffer: [f64;2],
    ops: String,
    result: f64,
    default: String,
    error: Option<Error>,
    //theme: Theme,
}


fn view(calculator: &Calculator) -> Element<Message> {
    text("Default Text")
        .size(16)
        //.color(color!(0xffffff))
        .into()
} 

fn update(calculator: &mut Calculator, message: Message) {
    match message {
        Message::InsertValue1 => calculator.buffer[0] = 10.0,
        Message::InsertValue2 => calculator.buffer[1] = 12.0,
        Message::InsertOperator => calculator.ops = String::from("+"),
    }
}

    //let operators: HashMap<&str, &str> = HashMap::from([("Add", "+"), ("Neg","-"), ("Mul","*"), ("Div","/")]);
    //let mut values: [f64; 2] = [3.0, PI];
    //let mut operator: &str = operators["Add"];
    //let mut rvalue: f64 = result(values, operator);
