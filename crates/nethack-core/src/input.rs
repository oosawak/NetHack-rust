//! Input handling and command mapping
//!
//! Converts keyboard input to NetHack game commands.

/// Keyboard key representation (abstract from winit)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Key {
    ArrowUp,
    ArrowDown,
    ArrowLeft,
    ArrowRight,
    Space,
    Escape,
    Enter,
    /// Alphanumeric keys
    Char(char),
    /// Digit keys
    Digit(u32),
}

impl Key {
    /// Create a Key from a char (for simplicity with testing)
    pub fn from_char(c: char) -> Option<Self> {
        match c {
            ' ' => Some(Key::Space),
            '\n' => Some(Key::Enter),
            c if c.is_ascii_digit() => {
                Some(Key::Digit(c.to_digit(10)?))
            }
            c if c.is_ascii_alphabetic() => {
                Some(Key::Char(c.to_ascii_lowercase()))
            }
            _ => None,
        }
    }
}

/// Game commands that can be executed
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameCommand {
    /// Move up
    MoveUp,
    /// Move down
    MoveDown,
    /// Move left
    MoveLeft,
    /// Move right
    MoveRight,
    /// Wait / no action
    Wait,
    /// Toggle search mode
    Search,
    /// Open doors / interact
    Open,
    /// Close doors
    Close,
    /// Rest
    Rest,
    /// Quit game
    Quit,
    /// View inventory
    Inventory,
    /// Cast a spell
    Cast,
    /// Look around
    Look,
}

impl GameCommand {
    /// Convert a keyboard key to a game command
    pub fn from_key(key: Key) -> Option<Self> {
        match key {
            Key::ArrowUp => Some(GameCommand::MoveUp),
            Key::ArrowDown => Some(GameCommand::MoveDown),
            Key::ArrowLeft => Some(GameCommand::MoveLeft),
            Key::ArrowRight => Some(GameCommand::MoveRight),
            Key::Space => Some(GameCommand::Wait),
            Key::Char('s') => Some(GameCommand::Search),
            Key::Char('o') => Some(GameCommand::Open),
            Key::Char('c') => Some(GameCommand::Close),
            Key::Char('r') => Some(GameCommand::Rest),
            Key::Char('q') => Some(GameCommand::Quit),
            Key::Char('i') => Some(GameCommand::Inventory),
            Key::Char('z') => Some(GameCommand::Cast),
            Key::Char('l') => Some(GameCommand::Look),
            _ => None,
        }
    }

    /// Get the NetHack character command for this game command
    pub fn to_nh_char(self) -> Option<char> {
        match self {
            GameCommand::MoveUp => Some('k'),
            GameCommand::MoveDown => Some('j'),
            GameCommand::MoveLeft => Some('h'),
            GameCommand::MoveRight => Some('l'),
            GameCommand::Wait => Some('.'),
            GameCommand::Search => Some('s'),
            GameCommand::Open => Some('o'),
            GameCommand::Close => Some('c'),
            GameCommand::Rest => Some('R'),
            GameCommand::Quit => Some('Q'),
            GameCommand::Inventory => Some('i'),
            GameCommand::Cast => Some('z'),
            GameCommand::Look => Some(';'),
        }
    }
}

/// Input state manager
pub struct InputManager {
    last_command: Option<GameCommand>,
    command_queue: Vec<GameCommand>,
}

impl InputManager {
    /// Create a new input manager
    pub fn new() -> Self {
        Self {
            last_command: None,
            command_queue: Vec::new(),
        }
    }

    /// Queue a command from keyboard input
    pub fn queue_command(&mut self, key: Key) {
        if let Some(cmd) = GameCommand::from_key(key) {
            self.command_queue.push(cmd);
            self.last_command = Some(cmd);
            tracing::debug!("Queued command: {:?}", cmd);
        }
    }

    /// Get the next command from the queue
    pub fn next_command(&mut self) -> Option<GameCommand> {
        if !self.command_queue.is_empty() {
            Some(self.command_queue.remove(0))
        } else {
            None
        }
    }

    /// Peek at the next command without removing it
    pub fn peek_command(&self) -> Option<GameCommand> {
        self.command_queue.first().copied()
    }

    /// Get the last command that was queued
    pub fn last_command(&self) -> Option<GameCommand> {
        self.last_command
    }

    /// Clear all queued commands
    pub fn clear(&mut self) {
        self.command_queue.clear();
    }
}

impl Default for InputManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_game_command_from_keys() {
        assert_eq!(GameCommand::from_key(Key::ArrowUp), Some(GameCommand::MoveUp));
        assert_eq!(GameCommand::from_key(Key::ArrowDown), Some(GameCommand::MoveDown));
        assert_eq!(GameCommand::from_key(Key::ArrowLeft), Some(GameCommand::MoveLeft));
        assert_eq!(GameCommand::from_key(Key::ArrowRight), Some(GameCommand::MoveRight));
    }

    #[test]
    fn test_command_to_nh_char() {
        assert_eq!(GameCommand::MoveUp.to_nh_char(), Some('k'));
        assert_eq!(GameCommand::MoveDown.to_nh_char(), Some('j'));
        assert_eq!(GameCommand::MoveLeft.to_nh_char(), Some('h'));
        assert_eq!(GameCommand::MoveRight.to_nh_char(), Some('l'));
        assert_eq!(GameCommand::Wait.to_nh_char(), Some('.'));
    }

    #[test]
    fn test_input_manager_queue() {
        let mut mgr = InputManager::new();
        mgr.queue_command(Key::ArrowUp);
        mgr.queue_command(Key::ArrowLeft);

        assert_eq!(mgr.next_command(), Some(GameCommand::MoveUp));
        assert_eq!(mgr.next_command(), Some(GameCommand::MoveLeft));
        assert_eq!(mgr.next_command(), None);
    }

    #[test]
    fn test_input_manager_peek() {
        let mut mgr = InputManager::new();
        mgr.queue_command(Key::ArrowUp);
        assert_eq!(mgr.peek_command(), Some(GameCommand::MoveUp));
        assert_eq!(mgr.peek_command(), Some(GameCommand::MoveUp));
    }

    #[test]
    fn test_key_from_char() {
        assert_eq!(Key::from_char('a'), Some(Key::Char('a')));
        assert_eq!(Key::from_char('Q'), Some(Key::Char('q')));
        assert_eq!(Key::from_char('1'), Some(Key::Digit(1)));
        assert_eq!(Key::from_char(' '), Some(Key::Space));
    }
}

