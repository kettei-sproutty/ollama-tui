use crossterm::{execute, terminal::*};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::io::{stdout, Stdout};

use crate::{
  models::{self, Model},
  state::State,
};

type Tui = Terminal<CrosstermBackend<Stdout>>;

#[derive(Debug)]
pub struct App {
  state: State,
  models: Vec<Model>,
}

impl App {
  pub fn new() -> Self {
    Self {
      state: State::new(),
      models: models::fetch_models(),
    }
  }

  pub fn setup(&mut self) -> anyhow::Result<Tui> {
    enable_raw_mode()?;
    execute!(stdout(), EnterAlternateScreen)?;
    Terminal::new(CrosstermBackend::new(stdout())).map_err(anyhow::Error::from)
  }

  pub fn restore() -> anyhow::Result<()> {
    execute!(stdout(), LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
  }
}
