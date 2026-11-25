mod components;


use iced::{
    alignment::Horizontal::Center,
    widget::{column, container, row, text},
    *,
};
use iced_aw::NumberInput;

use crate::components::*;

struct App {
    counter: u32,
    increment_size:u32,
    decrement_size:u32,
}

impl Default for App {
    fn default() -> Self {
        Self {
            counter: 0,
            increment_size: 1,
            decrement_size: 1,
        }
    }
}

#[derive(Debug, Clone)]
enum Message {
    Increment,
    Decrement,
    Multiply,
    Square,
    NumberInputIncrementChanged(u32),
    NumberInputDecrementChanged(u32),
    CounterReset,
}

impl App {
    fn update(&mut self, message: Message) {
        match message {
            Message::Increment => {
                self.counter = self.counter.saturating_add(self.increment_size);
            }
            Message::Decrement => {
                self.counter = self.counter.saturating_sub(self.decrement_size);
            }
            Message::Multiply => {
                self.counter = self.counter.saturating_mul(2);
            }
            Message::Square => {
                self.counter = self.counter.saturating_pow(2);
            }
            Message::NumberInputIncrementChanged(u32) => {
                self.increment_size = u32;
            }
            Message::NumberInputDecrementChanged(u32) => {
                self.decrement_size = u32;
            }
            Message::CounterReset => {
                self.counter = 0;
            },
        }
    }

    fn view(&self) -> Element<'_, Message> {
        container(
            column![
                navbar("My Iced App", None::<Element<'_, Message>>),
                container(
                    card(text!("Counter: {}", self.counter).width(Fill).align_x(Center), 
                        column![
                            row![
                                btn_success("Add", Message::Increment, Some(Center)),
                                btn_danger("Subtract", Message::Decrement, Some(Center))
                            ].spacing(10),

                            row![
                                btn_secondary("Multiply", Message::Multiply, Some(Center)),
                                btn_primary("Square", Message::Square, Some(Center))
                            ].spacing(10),

                            row![
                                column![
                                    text("Increment Size"),
                                    NumberInput::new(
                                        &self.increment_size, 
                                        1..=u32::MAX, 
                                        Message::NumberInputIncrementChanged
                                    )
                                    .width(Fill),
                                ].spacing(5).width(Fill),
                                column![
                                    text("Decrement Size"),
                                    NumberInput::new(
                                        &self.decrement_size, 
                                        1..=u32::MAX, 
                                        Message::NumberInputDecrementChanged
                                    )
                                    .width(Fill),
                                ].spacing(5).width(Fill),
                            ].spacing(10).width(Fill),

                            btn_danger("Reset Counter", Message::CounterReset, Some(Center)).width(Fill),
                        ].spacing(10),
                    )
                )
                .center(Fill),
            ]
        )
        .width(Fill)
        .height(Fill)
        .into()
    }
}

fn main() -> iced::Result {
    iced::application(App::default, App::update, App::view)
        .title("My Iced App")
        .font(iced_aw::ICED_AW_FONT_BYTES)
        .theme(|_: &App| Theme::Dracula)
        .run()
}
