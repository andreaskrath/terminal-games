use crate::app::App;
use tui::{
    backend::Backend,
    layout::Alignment,
    prelude::Rect,
    style::{Color, Modifier, Style},
    widgets::Paragraph,
    Frame,
};

/// Renders the user interface widgets.
pub fn render<B: Backend>(app: &mut App, frame: &mut Frame<B>) {
    menu_titles(app, frame);
    game_list(app, frame);
}

#[inline(always)]
fn menu_titles<B: Backend>(app: &mut App, frame: &mut Frame<B>) {
    let y_axis = (frame.size().height / 2) - (app.games.len() as u16 / 2);
    let x_axis = frame.size().width / 2;

    let title_text = String::from("Game Center");
    let title_length = title_text.len() as u16;
    let title_style = Style::default()
        .add_modifier(Modifier::BOLD)
        .fg(Color::White);
    let title = Paragraph::new(title_text).style(title_style);
    let title_x_offset = title_length / 2;
    let title_area = Rect::new(x_axis - title_x_offset, y_axis - 3, title_length, 1);
    frame.render_widget(title, title_area);

    let sub_title_text = String::from("Please pick a game from the list below!");
    let sub_title_length = sub_title_text.len() as u16;
    let sub_title_style = Style::default()
        .add_modifier(Modifier::ITALIC)
        .fg(Color::White);
    let sub_title = Paragraph::new(sub_title_text).style(sub_title_style);
    let sub_title_x_offset = sub_title_length / 2;
    let sub_title_area = Rect::new(x_axis - sub_title_x_offset, y_axis - 2, sub_title_length, 1);
    frame.render_widget(sub_title, sub_title_area);
}

#[inline(always)]
fn game_list<B: Backend>(app: &mut App, frame: &mut Frame<B>) {
    let longest_game_name_length = app
        .games
        .iter()
        .max()
        .expect("should always have a list of games")
        .len() as u16;

    let x_axis = (frame.size().width / 2) - (longest_game_name_length / 2);
    let y_axis = (frame.size().height / 2) - (app.games.len() as u16 / 2);

    for (index, game_name) in app.games.iter().enumerate() {
        let area = Rect::new(x_axis, y_axis + index as u16, longest_game_name_length, 1);

        let style = if index as u8 == app.state {
            Style::default().fg(Color::Cyan)
        } else {
            Style::default().fg(Color::DarkGray)
        };

        let game_listing = Paragraph::new(game_name.to_owned())
            .alignment(Alignment::Center)
            .style(style);
        frame.render_widget(game_listing, area);
    }
}
