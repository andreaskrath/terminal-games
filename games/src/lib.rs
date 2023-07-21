use std::error::Error;

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event::Key, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Layout},
    style::{Modifier, Style},
    widgets::{Block, BorderType, Borders, List, ListItem, ListState},
    Frame, Terminal,
};

struct GamesMenuList {
    state: ListState,
    items: Box<[&'static str]>,
}

impl GamesMenuList {
    fn new() -> Self {
        let mut new = Self {
            state: ListState::default(),
            items: Box::new(["Chess", "Minesweeper"]),
        };
        new.state.select(Some(0));
        new
    }

    fn move_up(&mut self) {
        let selected = match self.state.selected() {
            Some(val) => {
                if val == 0 {
                    Some(val)
                } else {
                    Some(val - 1)
                }
            }
            None => Some(0),
        };
        self.state.select(selected);
    }

    fn move_down(&mut self) {
        let selected = match self.state.selected() {
            Some(val) => {
                if val == self.items.len() - 1 {
                    Some(val)
                } else {
                    Some(val + 1)
                }
            }
            None => Some(0),
        };
        self.state.select(selected);
    }
}

struct GamesMenu {
    menu_state: MainMenuStates,
    games_list: GamesMenuList,
}

impl GamesMenu {
    fn new() -> Self {
        Self {
            menu_state: MainMenuStates::Menu,
            games_list: GamesMenuList::new(),
        }
    }

    fn change_menu_state(&mut self) {
        if let Some(val) = self.games_list.state.selected() {
            match val {
                0 => self.menu_state = MainMenuStates::Chess,
                1 => self.menu_state = MainMenuStates::Minesweeper,
                _ => unreachable!("should not be possible to go out of bounds of main menu"),
            }
        }
    }
}

enum MainMenuStates {
    Menu,
    Chess,
    Minesweeper,
}

pub fn run() -> Result<(), Box<dyn Error>> {
    enable_raw_mode()?;
    execute!(std::io::stdout(), EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(std::io::stdout());
    let mut terminal = Terminal::new(backend)?;

    let result = run_games_menu(&mut terminal);

    if let Err(err) = result {
        println!("{err}");
    }

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;

    Ok(())
}

fn run_games_menu<B: Backend>(terminal: &mut Terminal<B>) -> Result<(), Box<dyn Error>> {
    let mut state = GamesMenu::new();
    loop {
        terminal.draw(|f| render_games_menu(f, &mut state))?;
        if let Key(key) = event::read()? {
            match state.menu_state {
                MainMenuStates::Menu => match key.code {
                    KeyCode::Char('q') => return Ok(()),
                    KeyCode::Up => state.games_list.move_up(),
                    KeyCode::Down => state.games_list.move_down(),
                    KeyCode::Enter => state.change_menu_state(),
                    _ => {}
                },
                MainMenuStates::Chess => todo!(),
                MainMenuStates::Minesweeper => todo!(),
            }
        }
    }
}

fn render_games_menu<B: Backend>(f: &mut Frame<B>, state: &mut GamesMenu) {
    let full_screen = Layout::default()
        .direction(tui::layout::Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage(33),
                Constraint::Percentage(34),
                Constraint::Percentage(33),
            ]
            .as_ref(),
        )
        .split(f.size());

    let full_height_middle_block = Block::default();
    f.render_widget(full_height_middle_block, full_screen[1]);

    let center_area = Layout::default()
        .direction(tui::layout::Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage(33),
                Constraint::Percentage(34),
                Constraint::Percentage(33),
            ]
            .as_ref(),
        )
        .split(full_screen[1]);

    let menu_block = Block::default()
        .title("Games Menu")
        .title_alignment(tui::layout::Alignment::Center)
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded);
    f.render_widget(menu_block, center_area[1]);

    let list_layout = Layout::default()
        .margin(2)
        .direction(tui::layout::Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(1)].as_ref())
        .split(center_area[1]);

    let items: Vec<ListItem<'_>> = state
        .games_list
        .items
        .iter()
        .map(|item| ListItem::new(*item))
        .collect();
    let games_list = List::new(items)
        .block(Block::default())
        .highlight_symbol("->")
        .highlight_style(Style::default().add_modifier(Modifier::BOLD));
    f.render_stateful_widget(games_list, list_layout[1], &mut state.games_list.state);
}
