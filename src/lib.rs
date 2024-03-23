use std::io::{self, stdout};

use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};

type RatatuiTerminal = Terminal<CrosstermBackend<io::Stdout>>;

/// Sets up the terminal
pub fn setup_terminal() -> RatatuiTerminal {
    enable_raw_mode().expect("Enabling raw mode failed");
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture).expect("Execute failed");

    let backend = CrosstermBackend::new(stdout);
    let terminal = Terminal::new(backend).expect("Terminal setup failed");

    terminal
}

/// Shuts the terminal down gracefully
pub fn shutdown_terminal(mut terminal: RatatuiTerminal) -> io::Result<()> {
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}
