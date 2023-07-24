use super::Chess;
use tui::{
    prelude::{Alignment, Backend, Rect},
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame,
};

const BOARD_WIDTH: u16 = 36;
const BOARD_HEIGHT: u16 = 18;

pub fn render<B: Backend>(chess: &mut Chess<B>, frame: &mut Frame<'_, B>) {
    render_outer_block(frame);
    render_board(chess, frame);
    render_white_player_nameplate(frame);
    render_black_player_nameplate(frame);
}

#[inline(always)]
fn render_outer_block<B: Backend>(frame: &mut Frame<'_, B>) {
    let outer_block = Block::default()
        .title("Chess")
        .title_alignment(Alignment::Center)
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded);
    frame.render_widget(outer_block, frame.size());
}

#[inline(always)]
fn render_board<B: Backend>(chess: &mut Chess<B>, frame: &mut Frame<'_, B>) {
    // these centerings do not account for the row numbers and column letters
    // add 1 to both results to center in relation to the board itself
    let x_axis = (frame.size().width / 2) - (BOARD_WIDTH / 2);
    let y_axis = (frame.size().height / 2) - (BOARD_HEIGHT / 2);
    let board_area = Rect::new(x_axis, y_axis, BOARD_WIDTH, BOARD_HEIGHT);
    frame.render_widget(chess.game.owned_board(), board_area);
}

#[inline(always)]
fn render_white_player_nameplate<B: Backend>(frame: &mut Frame<'_, B>) {
    let text = "WHITE PLAYER";

    let x_axis = (frame.size().width / 2) - (text.len() / 2) as u16;
    let y_axis = (frame.size().height / 2) + 14;
    let text_area = Rect::new(x_axis, y_axis, text.len() as u16, 1);
    let paragraph = Paragraph::new(text);
    frame.render_widget(paragraph, text_area);
}

#[inline(always)]
fn render_black_player_nameplate<B: Backend>(frame: &mut Frame<'_, B>) {
    let text = "BLACK PLAYER";

    let x_axis = (frame.size().width / 2) - (text.len() / 2) as u16;
    let y_axis = (frame.size().height / 2) - 14;
    let text_area = Rect::new(x_axis, y_axis, text.len() as u16, 1);
    let paragraph = Paragraph::new(text);
    frame.render_widget(paragraph, text_area);
}
