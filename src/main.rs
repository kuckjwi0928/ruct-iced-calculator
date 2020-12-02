// It's very difficult TT...

use iced::{Container, Align, button, Button, Row, Column, Element, Sandbox, Settings, Text, Length};

pub fn main() -> iced::Result {
    Calculator::run(Settings::default())
}

enum Operator {
    Plus,
    Minus,
    Multiply,
    Divide,
    None
}

impl Operator {
    fn apply(&self, value: i64, calc_value: i64) -> i64 {
        return match self {
            Operator::Plus => {
                calc_value + value
            }
            Operator::Minus => {
                calc_value - value
            }
            Operator::Multiply => {
                calc_value * value
            }
            Operator::Divide => {
                calc_value / value
            }
            Operator::None => {
                value
            }
        }
    }

    fn is_none(&self) -> bool {
        if let Operator::None = self {
            true
        } else {
            false
        }
    }
}

//
// Operator default impl
//
impl Default for Operator {
    fn default() -> Self { Operator::None }
}

#[derive(Debug, Clone, Copy)]
enum OperatorMessage {
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
    Nine,
    Clear
}

#[derive(Default)]
struct Calculator {
    value: String,
    operator: Operator,
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
    divide_button: button::State,

    empty_button: button::State,
    empty_button2: button::State,
    empty_button3: button::State,
    empty_button4: button::State,
    empty_button5: button::State,
}

impl Sandbox for Calculator {
    type Message = OperatorMessage;

    fn new() -> Self {
        Self::default()
    }

    fn title(&self) -> String {
        String::from("rust-iced-calculator")
    }

    fn update(&mut self, operator_message: OperatorMessage) {
        let calc_value: i64 = match self.value.trim().parse() {
            Ok(num) => num,
            Err(_) => 0,
        };

        if !self.operator.is_none() {
            self.value = String::from("");
        }

        match operator_message {
            OperatorMessage::Zero => {
                self.value.push_str(self.operator.apply(0, calc_value).to_string().as_ref());
                self.operator = Operator::None;
            }
            OperatorMessage::One => {
                self.value.push_str(self.operator.apply(1, calc_value).to_string().as_ref());
                self.operator = Operator::None;
            }
            OperatorMessage::Two => {
                self.value.push_str(self.operator.apply(2, calc_value).to_string().as_ref());
                self.operator = Operator::None;
            }
            OperatorMessage::Three => {
                self.value.push_str(self.operator.apply(3, calc_value).to_string().as_ref());
                self.operator = Operator::None;
            }
            OperatorMessage::Four => {
                self.value.push_str(self.operator.apply(4, calc_value).to_string().as_ref());
                self.operator = Operator::None;
            }
            OperatorMessage::Five => {
                self.value.push_str(self.operator.apply(5, calc_value).to_string().as_ref());
                self.operator = Operator::None;
            }
            OperatorMessage::Six => {
                self.value.push_str(self.operator.apply(6, calc_value).to_string().as_ref());
                self.operator = Operator::None;
            }
            OperatorMessage::Seven => {
                self.value.push_str(self.operator.apply(7, calc_value).to_string().as_ref());
                self.operator = Operator::None;
            }
            OperatorMessage::Eight => {
                self.value.push_str(self.operator.apply(8, calc_value).to_string().as_ref());
                self.operator = Operator::None;
            }
            OperatorMessage::Nine => {
                self.value.push_str(self.operator.apply(9, calc_value).to_string().as_ref());
                self.operator = Operator::None;
            }
            OperatorMessage::Plus => {
                self.operator = Operator::Plus;
            }
            OperatorMessage::Minus => {
                self.operator = Operator::Minus;
            }
            OperatorMessage::Multiply => {
                self.operator = Operator::Multiply;
            }
            OperatorMessage::Divide => {
                self.operator = Operator::Divide;
            },
            OperatorMessage::Clear => {
                self.value = String::from("");
                self.operator = Operator::None;
            }
        }
    }

    fn view(&mut self) -> Element<OperatorMessage> {
        let row = Row::new()
            .align_items(Align::Center)
            .spacing(5)
            .push(Text::new(self.value.to_owned()).size(50))
            .push(Column::new()
                .align_items(Align::Center)
                .spacing(5)
                .push(Button::new(&mut self.empty_button, Text::new(" ")).padding(10))
                .push(Button::new(&mut self.seven_button, Text::new("7")).on_press(OperatorMessage::Seven).padding(10))
                .push(Button::new(&mut self.four_button, Text::new("4")).on_press(OperatorMessage::Four).padding(10))
                .push(Button::new(&mut self.one_button, Text::new("1")).on_press(OperatorMessage::One).padding(10))
                .push(Button::new(&mut self.zero_button, Text::new("0")).on_press(OperatorMessage::Zero).padding(10)))
            .push(Column::new()
                .align_items(Align::Center)
                .spacing(5)
                .push(Button::new(&mut self.empty_button2, Text::new(" ")).padding(10))
                .push(Button::new(&mut self.eight_button, Text::new("8")).on_press(OperatorMessage::Eight).padding(10))
                .push(Button::new(&mut self.five_button, Text::new("5")).on_press(OperatorMessage::Five).padding(10))
                .push(Button::new(&mut self.two_button, Text::new("2")).on_press(OperatorMessage::Two).padding(10))
                .push(Button::new(&mut self.empty_button3, Text::new(" ")).padding(10)))
            .push(Column::new()
                .align_items(Align::Center)
                .spacing(5)
                .push(Button::new(&mut self.clear_button, Text::new("C")).on_press(OperatorMessage::Clear).padding(10))
                .push(Button::new(&mut self.nine_button, Text::new("9")).on_press(OperatorMessage::Nine).padding(10))
                .push(Button::new(&mut self.six_button, Text::new("6")).on_press(OperatorMessage::Six).padding(10))
                .push(Button::new(&mut self.three_button, Text::new("3")).on_press(OperatorMessage::Three).padding(10))
                .push(Button::new(&mut self.empty_button4, Text::new(" ")).padding(10)))
            .push(Column::new()
                .align_items(Align::Center)
                .spacing(5)
                .push(Button::new(&mut self.plus_button, Text::new("+")).on_press(OperatorMessage::Plus).padding(10))
                .push(Button::new(&mut self.minus_button, Text::new("-")).on_press(OperatorMessage::Minus).padding(10))
                .push(Button::new(&mut self.multiply_button, Text::new("*")).on_press(OperatorMessage::Multiply).padding(10))
                .push(Button::new(&mut self.divide_button, Text::new("/")).on_press(OperatorMessage::Divide).padding(10))
                .push(Button::new(&mut self.empty_button5, Text::new(" ")).padding(10)));

        Container::new(row)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}
