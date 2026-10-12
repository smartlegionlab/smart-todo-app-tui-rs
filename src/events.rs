use anyhow::Result;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::app::{App, Mode};

pub fn handle_key(app: &mut App, key: KeyEvent) -> Result<()> {
    if key.modifiers.contains(KeyModifiers::CONTROL) && key.code == KeyCode::Char('c') {
        app.should_quit = true;
        return Ok(());
    }

    match app.mode {
        Mode::Normal => handle_normal(app, key)?,
        Mode::Adding | Mode::Editing => handle_input(app, key)?,
        Mode::About => handle_about(app, key)?,
    }

    Ok(())
}

fn handle_normal(app: &mut App, key: KeyEvent) -> Result<()> {
    match key.code {
        KeyCode::Char('q') | KeyCode::Esc => app.should_quit = true,
        KeyCode::Down | KeyCode::Char('j') => app.next(),
        KeyCode::Up | KeyCode::Char('k') => app.prev(),
        KeyCode::Char(' ') | KeyCode::Enter => app.toggle_done(),
        KeyCode::Char('a') => app.start_adding(),
        KeyCode::Char('e') => app.start_editing(),
        KeyCode::Char('d') => app.request_delete(),
        KeyCode::Char('i') => app.open_about(),
        _ => app.clear_pending_delete(),
    }
    Ok(())
}

fn handle_input(app: &mut App, key: KeyEvent) -> Result<()> {
    match key.code {
        KeyCode::Enter => app.commit_input(),
        KeyCode::Esc => app.cancel_input(),
        KeyCode::Backspace => app.input_backspace(),
        KeyCode::Left => app.input_left(),
        KeyCode::Right => app.input_right(),
        KeyCode::Char(c) => app.input_push(c),
        _ => {}
    }
    Ok(())
}

fn handle_about(app: &mut App, key: KeyEvent) -> Result<()> {
    match key.code {
        KeyCode::Esc | KeyCode::Char('q') | KeyCode::Char('i') | KeyCode::Enter => {
            app.close_about()
        }
        _ => {}
    }
    Ok(())
}
