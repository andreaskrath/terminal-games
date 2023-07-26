use super::Chess;
use tui::{
    prelude::{Alignment, Backend, Rect},
    widgets::{Block, BorderType, Borders, List, ListItem, Paragraph},
    Frame,
};

/// The total width of the board in characters; includes the number markings.
const BOARD_WIDTH: u16 = 36;

/// The total height of the board in characters; includes the letter markings.
const BOARD_HEIGHT: u16 = 18;

/// The minimum width the terminal requires to render all the components.
const MINIMUM_TERMINAL_WIDTH: u16 = 62;

// The minimum height the terminal requires to render all the components.
const MINIMUM_TERMINAL_HEIGHT: u16 = 31;

pub fn render<B: Backend>(chess: &mut Chess<B>, frame: &mut Frame<'_, B>) {
    if increase_terminal_size(frame) {
        return;
    }

    outer_block(frame);
    board(chess, frame);
    white_player(chess, frame);
    black_player(chess, frame);
}

fn increase_terminal_size<B: Backend>(frame: &mut Frame<'_, B>) -> bool {
    let placeholder = match (
        frame.size().width < MINIMUM_TERMINAL_WIDTH,
        frame.size().height < MINIMUM_TERMINAL_HEIGHT,
    ) {
        (true, true) => "width and height",
        (true, false) => "width",
        (false, true) => "height",
        (false, false) => return false,
    };

    let text = format!("Please increase the {placeholder} of the terminal.");

    // the programs will crash if the text cannot be rendered within the given terminal size
    // ideally no comment at all with a super small terminal should be a dead giveaway
    // that the terminal size is too small - alas, there is no way to actually communicate this
    if (frame.size().width < text.len() as u16) || (frame.size().height < 1) {
        return true;
    }

    let x_axis = (frame.size().width / 2) - (text.len() / 2) as u16;
    let y_axis = frame.size().height / 2;
    let area = Rect::new(x_axis, y_axis, text.len() as u16, 1);
    let paragraph = Paragraph::new(text).alignment(Alignment::Center);
    frame.render_widget(paragraph, area);

    true
}

#[inline(always)]
fn outer_block<B: Backend>(frame: &mut Frame<'_, B>) {
    let outer_block = Block::default()
        .title("Chess")
        .title_alignment(Alignment::Center)
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded);
    frame.render_widget(outer_block, frame.size());
}

#[inline(always)]
fn board<B: Backend>(chess: &mut Chess<B>, frame: &mut Frame<'_, B>) {
    // these centerings do not account for the row numbers and column letters
    // add 1 to both results to center in relation to the board itself
    let x_axis = (frame.size().width / 2) - (BOARD_WIDTH / 2);
    let y_axis = (frame.size().height / 2) - (BOARD_HEIGHT / 2);
    let board_area = Rect::new(x_axis, y_axis, BOARD_WIDTH, BOARD_HEIGHT);
    frame.render_widget(chess.game.owned_board(), board_area);
}

#[inline(always)]
fn black_player<B: Backend>(chess: &mut Chess<B>, frame: &mut Frame<'_, B>) {
    black_player_nameplate(frame);
    black_player_taken_pieces(chess, frame);
    black_player_moves(chess, frame);
}

#[inline(always)]
fn black_player_nameplate<B: Backend>(frame: &mut Frame<'_, B>) {
    let text = "BLACK PLAYER";

    let x_axis = (frame.size().width / 2) - (text.len() / 2) as u16;
    let y_axis = (frame.size().height / 2) - 14;
    let text_area = Rect::new(x_axis, y_axis, text.len() as u16, 1);
    let paragraph = Paragraph::new(text);
    frame.render_widget(paragraph, text_area);
}

#[inline(always)]
fn black_player_taken_pieces<B: Backend>(chess: &mut Chess<B>, frame: &mut Frame<'_, B>) {
    let taken_pieces = " ♜ ♞ ♝ ♛ ♚ ♝ ♞ ♜ ♜ ♞ ♝ ♛ ♚ ♝ ♞ ♜";
    let x_axis = (frame.size().width / 2) - (BOARD_WIDTH / 2) + 2;
    let y_axis = (frame.size().height / 2) - 13;
    let area = Rect::new(x_axis, y_axis, 32, 1);
    let text = Paragraph::new(taken_pieces).alignment(Alignment::Left);
    frame.render_widget(text, area);
}

#[inline(always)]
fn black_player_moves<B: Backend>(chess: &mut Chess<B>, frame: &mut Frame<'_, B>) {
    let v = vec![
        "♖ E2->E4",
        "♖ E2->E4",
        "♖ E2->E4",
        "♖ E2->E4",
        "♖ E2->E4",
        "♖ E2->E4",
        "♖ E2->E4",
        "♖ E2->E4",
        "♖ E2->E4",
        "♖ E2->E4",
    ];

    let items: Vec<ListItem> = v.iter().map(|item| ListItem::new(*item)).collect();
    let x_axis = (frame.size().width / 2) - 30;
    let y_axis = (frame.size().height / 2) - 5;
    let area = Rect::new(x_axis, y_axis, 8, items.len() as u16);
    let moves = List::new(items);
    frame.render_widget(moves, area);
}

#[inline(always)]
fn white_player<B: Backend>(chess: &mut Chess<B>, frame: &mut Frame<'_, B>) {
    white_player_nameplate(frame);
    white_player_taken_pieces(chess, frame);
    white_player_moves(chess, frame);
}

#[inline(always)]
fn white_player_nameplate<B: Backend>(frame: &mut Frame<'_, B>) {
    let text = "WHITE PLAYER";

    let x_axis = (frame.size().width / 2) - (text.len() / 2) as u16;
    let y_axis = (frame.size().height / 2) + 14;
    let text_area = Rect::new(x_axis, y_axis, text.len() as u16, 1);
    let paragraph = Paragraph::new(text);
    frame.render_widget(paragraph, text_area);
}

#[inline(always)]
fn white_player_taken_pieces<B: Backend>(chess: &mut Chess<B>, frame: &mut Frame<'_, B>) {
    let taken_pieces = " ♖ ♘ ♗ ♕ ♔ ♗ ♘ ♖ ♖ ♘ ♗ ♕ ♔ ♗ ♘ ♖";
    let x_axis = (frame.size().width / 2) - (BOARD_WIDTH / 2) + 2;
    let y_axis = (frame.size().height / 2) + 13;
    let area = Rect::new(x_axis, y_axis, 32, 1);
    let text = Paragraph::new(taken_pieces).alignment(Alignment::Left);
    frame.render_widget(text, area);
}

#[inline(always)]
fn white_player_moves<B: Backend>(chess: &mut Chess<B>, frame: &mut Frame<'_, B>) {
    let v = vec![
        "♜ E2->E4",
        "♜ E2->E4",
        "♜ E2->E4",
        "♜ E2->E4",
        "♜ E2->E4",
        "♜ E2->E4",
        "♜ E2->E4",
        "♜ E2->E4",
        "♜ E2->E4",
        "♜ E2->E4",
    ];

    let items: Vec<ListItem> = v.iter().map(|item| ListItem::new(*item)).collect();
    let x_axis = (frame.size().width / 2) + 22;
    let y_axis = (frame.size().height / 2) - 5;
    let area = Rect::new(x_axis, y_axis, 8, items.len() as u16);
    let moves = List::new(items);
    frame.render_widget(moves, area);
}
