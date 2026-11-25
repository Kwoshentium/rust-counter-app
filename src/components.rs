use iced::{
    alignment::Horizontal,
    widget::{column, *},
    Border, Color, Length, *,
};

use crate::Message;

// 1. Buttons -> Button<'a, Message>
pub fn btn_primary<'a>(
    label: &'a str,
    on_press: Message,
    align: Option<Horizontal>,
) -> Button<'a, Message> {
    let content = match align {
        Some(a) => text(label).width(Length::Fill).align_x(a),
        None => text(label),
    };

    button(content)
        .padding([8, 16])
        .on_press(on_press)
        .style(|theme: &Theme, status| {
            let palette = theme.extended_palette();
            let bg = match status {
                button::Status::Hovered => palette.primary.strong.color,
                button::Status::Pressed => palette.primary.weak.color,
                _ => palette.primary.base.color,
            };
            button::Style {
                background: Some(Background::Color(bg)),
                text_color: palette.primary.base.text,
                border: Border::default().rounded(6),
                ..Default::default()
            }
        })
}

pub fn btn_secondary<'a>(
    label: &'a str,
    on_press: Message,
    align: Option<Horizontal>,
) -> Button<'a, Message> {
    let content = match align {
        Some(a) => text(label).width(Length::Fill).align_x(a),
        None => text(label),
    };

    button(content)
        .padding([8, 16])
        .on_press(on_press)
        .style(|theme: &Theme, status| {
            let palette = theme.extended_palette();
            let bg = match status {
                button::Status::Hovered => palette.secondary.strong.color,
                button::Status::Pressed => palette.secondary.weak.color,
                _ => palette.secondary.base.color,
            };
            button::Style {
                background: Some(Background::Color(bg)),
                text_color: palette.secondary.base.text,
                border: Border::default().rounded(6),
                ..Default::default()
            }
        })
}

pub fn btn_success<'a>(
    label: &'a str,
    on_press: Message,
    align: Option<Horizontal>,
) -> Button<'a, Message> {
    let content = match align {
        Some(a) => text(label).width(Length::Fill).align_x(a),
        None => text(label),
    };

    button(content)
        .padding([8, 16])
        .on_press(on_press)
        .style(|theme: &Theme, status| {
            let palette = theme.extended_palette();
            let bg = match status {
                button::Status::Hovered => palette.success.strong.color,
                button::Status::Pressed => palette.success.weak.color,
                _ => palette.success.base.color,
            };
            button::Style {
                background: Some(Background::Color(bg)),
                text_color: palette.success.base.text,
                border: Border::default().rounded(6),
                ..Default::default()
            }
        })
}

pub fn btn_danger<'a>(
    label: &'a str,
    on_press: Message,
    align: Option<Horizontal>,
) -> Button<'a, Message> {
    let content = match align {
        Some(a) => text(label).width(Length::Fill).align_x(a),
        None => text(label),
    };

    button(content)
        .padding([8, 16])
        .on_press(on_press)
        .style(|theme: &Theme, status| {
            let palette = theme.extended_palette();
            let bg = match status {
                button::Status::Hovered => palette.danger.strong.color,
                button::Status::Pressed => palette.danger.weak.color,
                _ => palette.danger.base.color,
            };
            button::Style {
                background: Some(Background::Color(bg)),
                text_color: palette.danger.base.text,
                border: Border::default().rounded(6),
                ..Default::default()
            }
        })
}

// 2. Card -> Container<'a, Message>
pub fn card<'a>(
    header_content: impl Into<Element<'a, Message>>,
    body: impl Into<Element<'a, Message>>,
) -> Container<'a, Message> {
    let header = container(header_content)
        .padding([12, 16])
        .width(Length::Fill)
        .style(|theme: &Theme| {
            let palette = theme.extended_palette();
            container::Style {
                background: Some(Background::Color(palette.background.strong.color)),
                text_color: Some(palette.background.strong.text),
                border: Border {
                    color: palette.background.strong.color,
                    width: 1.0,
                    radius: iced::border::Radius {
                        top_left: 8.0,
                        top_right: 8.0,
                        bottom_right: 0.0,
                        bottom_left: 0.0,
                    },
                },
                ..Default::default()
            }
        });

    let content = container(body)
        .padding(16)
        .width(Length::Fill);

    container(column![header, content])
        .max_width(400)
        .style(|theme: &Theme| {
            let palette = theme.extended_palette();
            container::Style {
                background: Some(Background::Color(palette.background.weak.color)),
                text_color: Some(palette.background.weak.text),
                border: Border {
                    color: palette.background.strong.color,
                    width: 1.0,
                    radius: 8.0.into(),
                },
                ..Default::default()
            }
        })
}

// 3. Alert Banners -> Container<'a, Message>
pub fn alert_success<'a>(msg: &'a str) -> Container<'a, Message> {
    container(text(msg))
        .padding([12, 16])
        .width(Length::Fill)
        .style(|theme: &Theme| {
            let palette = theme.extended_palette();
            container::Style {
                background: Some(Background::Color(palette.success.weak.color)),
                text_color: Some(palette.success.weak.text),
                border: Border {
                    color: palette.success.base.color,
                    width: 1.0,
                    radius: 6.0.into(),
                },
                ..Default::default()
            }
        })
}

pub fn alert_danger<'a>(msg: &'a str) -> Container<'a, Message> {
    container(text(msg))
        .padding([12, 16])
        .width(Length::Fill)
        .style(|theme: &Theme| {
            let palette = theme.extended_palette();
            container::Style {
                background: Some(Background::Color(palette.danger.weak.color)),
                text_color: Some(palette.danger.weak.text),
                border: Border {
                    color: palette.danger.base.color,
                    width: 1.0,
                    radius: 6.0.into(),
                },
                ..Default::default()
            }
        })
}

// 4. Badge / Pill -> Container<'a, Message>
pub fn badge<'a>(label: &'a str) -> Container<'a, Message> {
    container(text(label).size(12))
        .padding([4, 10])
        .style(|theme: &Theme| {
            let palette = theme.extended_palette();
            container::Style {
                background: Some(Background::Color(palette.secondary.base.color)),
                text_color: Some(palette.secondary.base.text),
                border: Border::default().rounded(999),
                ..Default::default()
            }
        })
}

// 5. Navbar -> Container<'a, Message>
pub fn navbar<'a>(
    brand: &'a str,
    actions: Option<impl Into<Element<'a, Message>>>,
) -> Container<'a, Message> {
    let mut content = row![text(brand).size(20)].align_y(Alignment::Center);

    if let Some(actions) = actions {
        content = content.push(space::horizontal()).push(actions.into());
    }

    container(content)
        .padding([12, 20])
        .width(Length::Fill)
        .style(|theme: &Theme| {
            let palette = theme.extended_palette();
            container::Style {
                background: Some(Background::Color(palette.background.strong.color)),
                text_color: Some(palette.background.strong.text),
                ..Default::default()
            }
        })
}

// 6. Modal Overlay -> Stack<'a, Message>
pub fn modal<'a>(
    base: impl Into<Element<'a, Message>>,
    dialog: impl Into<Element<'a, Message>>,
    on_backdrop_click: Message,
) -> Stack<'a, Message> {
    let dialog_box = container(dialog)
        .padding(24)
        .style(|theme: &Theme| {
            let palette = theme.extended_palette();
            container::Style {
                background: Some(Background::Color(palette.background.base.color)),
                text_color: Some(palette.background.base.text),
                border: Border {
                    color: palette.background.strong.color,
                    width: 1.0,
                    radius: 10.0.into(),
                },
                ..Default::default()
            }
        });

    let backdrop = button(dialog_box)
        .on_press(on_backdrop_click)
        .style(|_theme: &Theme, _status| button::Style {
            background: Some(Background::Color(Color::from_rgba(0.0, 0.0, 0.0, 0.6))),
            ..Default::default()
        });

    stack![
        base.into(),
        container(backdrop)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x(Length::Fill)
            .center_y(Length::Fill)
    ]
}