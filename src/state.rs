#[derive(Default, Debug)]
pub enum Screen {
  #[default]
  /// The user is in the chat screen
  Chat,
  /// The user is in the model screen
  Models,
  /// The user is in the options screen
  Options,
  /// The user is in the exit screen
  Exit,
}

#[derive(Debug)]
pub struct History {
  question: String,
  answer: String,
}

#[derive(Debug)]
pub struct State {
  /// The value of the input in the chat screen
  pub chat_input: String,
  /// The current screen user is rendering
  pub screen: Screen,
  /// The chat history, stored inside $HOME/ollama-tui/history.ron
  pub chat_history: Vec<History>,
}

impl State {
  pub fn new() -> Self {
    State {
      chat_input: String::new(),
      screen: Screen::default(),
      chat_history: vec![],
    }
  }
}
