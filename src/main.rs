use iced::{Align, button, Button, Row, Column, Element, Sandbox, Settings, Text, Length};

pub fn main() -> iced::Result {
    Calculator::run(Settings::default())
}

#[derive(Default)]
struct Calculator {
    value: i64,
    is_operator: bool,
    zero_button: button::State,
    one_button: button::State,
    two_button: button::State,
    three_button: button::State,
    four_button: button::State,
    five_button: button::State,
    six_button: button::State,
    seven_button: button::State,
    eight_button: button::State,
    nine_button: button::State,
    clear_button: button::State,
    plus_button: button::State,
    minus_button: button::State,
    multiply_button: button::State,
    divide_button: button::State
}

#[derive(Debug, Clone, Copy)]
enum Operator {
    Plus,
    Minus,
    Multiply,
    Divide,
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine
}

impl Sandbox for Calculator {
    type Message = Operator;

    fn new() -> Self {
        Self::default()
    }

    fn title(&self) -> String {
        String::from("rust-iced-calculator")
    }

    fn update(&mut self, operator: Operator) {
        match operator {
            Operator::Zero => {
                self.value = 0;
                self.is_operator = false
            }
            Operator::One => {
                self.value = 1;
                self.is_operator = false
            }
            Operator::Two => {
                self.value = 2;
                self.is_operator = false
            }
            Operator::Three => {
                self.value = 3;
                self.is_operator = false
            }
            Operator::Four => {
                self.value = 4;
                self.is_operator = false
            }
            Operator::Five => {
                self.value = 5;
                self.is_operator = false
            }
            Operator::Six => {
                self.value = 6;
                self.is_operator = false
            }
            Operator::Seven => {
                self.value = 7;
                self.is_operator = false
            }
            Operator::Eight => {
                self.value = 8;
                self.is_operator = false
            }
            Operator::Nine => {
                self.value = 9;
                self.is_operator = false
            }
            Operator::Plus => {
                self.is_operator = false
            }
            Operator::Minus => {
                self.is_operator = true
            }
            Operator::Multiply => {
                self.is_operator = true
            }
            Operator::Divide => {
                self.is_operator = true
            }
        }
    }

    fn view(&mut self) -> Element<Operator> {
        // TODO(kuckjwi): design
        Row::new()
            .padding(10)
            .align_items(Align::Center)
            .push(Text::new(self.value.to_string()).size(50))
            .push(Column::new()
                .padding(10)
                .align_items(Align::Center)
                .push(Button::new(&mut self.one_button, Text::new("1")).on_press(Operator::One))
                .push(Button::new(&mut self.four_button, Text::new("4")).on_press(Operator::Four))
                .push(Button::new(&mut self.seven_button, Text::new("7")).on_press(Operator::Seven))
                .push(Button::new(&mut self.zero_button, Text::new("0")).on_press(Operator::Zero)))
            .push(Column::new()
                .padding(10)
                .align_items(Align::Center)
                .push(Button::new(&mut self.two_button, Text::new("2")).on_press(Operator::Two))
                .push(Button::new(&mut self.five_button, Text::new("5")).on_press(Operator::Five))
                .push(Button::new(&mut self.eight_button, Text::new("8")).on_press(Operator::Eight)))
            .push(Column::new()
                .padding(10)
                .align_items(Align::Center)
                .push(Button::new(&mut self.three_button, Text::new("3")).on_press(Operator::Three))
                .push(Button::new(&mut self.six_button, Text::new("6")).on_press(Operator::Six))
                .push(Button::new(&mut self.nine_button, Text::new("9")).on_press(Operator::Nine)))
            .into()
    }
}