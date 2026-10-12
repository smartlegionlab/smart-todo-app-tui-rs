use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::storage;

pub const APP_NAME: &str = "smart-todo-app-tui";
pub const APP_VERSION: &str = env!("CARGO_PKG_VERSION");
pub const APP_AUTHOR: &str = "Alexander Suvorov";
pub const APP_REPO_URL: &str = "https://github.com/smartlegionlab/smart-todo-app-tui-rs";
pub const APP_SITE_URL: &str = "https://smartlegionlab.github.io";
pub const APP_LICENSE: &str = "BSD-3-Clause";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Todo {
    pub title: String,
    pub done: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Mode {
    Normal,
    Adding,
    Editing,
    About,
}

#[derive(Debug)]
pub struct App {
    pub todos: Vec<Todo>,
    pub selected: usize,
    pub mode: Mode,
    pub input: String,
    pub input_cursor: usize,
    pub should_quit: bool,
    pub status: Option<String>,
    pub pending_delete: Option<usize>,
}

impl App {
    pub fn new() -> Result<Self> {
        let todos = storage::load().unwrap_or_default();
        Ok(Self {
            todos,
            selected: 0,
            mode: Mode::Normal,
            input: String::new(),
            input_cursor: 0,
            should_quit: false,
            status: None,
            pending_delete: None,
        })
    }

    pub fn save(&self) -> Result<()> {
        storage::save(&self.todos)
    }

    pub fn next(&mut self) {
        if self.todos.is_empty() {
            return;
        }
        self.clear_pending_delete();
        self.selected = (self.selected + 1).min(self.todos.len() - 1);
    }

    pub fn prev(&mut self) {
        self.clear_pending_delete();
        if self.selected > 0 {
            self.selected -= 1;
        }
    }

    pub fn toggle_done(&mut self) {
        self.clear_pending_delete();
        if let Some(t) = self.todos.get_mut(self.selected) {
            t.done = !t.done;
            let _ = self.save();
        }
    }

    pub fn request_delete(&mut self) {
        if self.todos.is_empty() {
            return;
        }

        if self.pending_delete == Some(self.selected) {
            self.todos.remove(self.selected);
            if self.selected >= self.todos.len() && !self.todos.is_empty() {
                self.selected = self.todos.len() - 1;
            }
            self.pending_delete = None;
            let _ = self.save();
        } else {
            self.pending_delete = Some(self.selected);
        }
    }

    pub fn clear_pending_delete(&mut self) {
        self.pending_delete = None;
    }

    pub fn open_about(&mut self) {
        self.clear_pending_delete();
        self.mode = Mode::About;
    }

    pub fn close_about(&mut self) {
        self.mode = Mode::Normal;
    }

    pub fn start_adding(&mut self) {
        self.clear_pending_delete();
        self.mode = Mode::Adding;
        self.input.clear();
        self.input_cursor = 0;
        self.status = None;
    }

    pub fn start_editing(&mut self) {
        self.clear_pending_delete();
        if let Some(t) = self.todos.get(self.selected) {
            self.mode = Mode::Editing;
            self.input = t.title.clone();
            self.input_cursor = self.input.chars().count();
            self.status = None;
        }
    }

    pub fn commit_input(&mut self) {
        let title = self.input.trim().to_string();
        if title.is_empty() {
            self.cancel_input();
            return;
        }

        match self.mode {
            Mode::Adding => {
                self.todos.push(Todo { title, done: false });
                self.selected = self.todos.len() - 1;
            }
            Mode::Editing => {
                if let Some(t) = self.todos.get_mut(self.selected) {
                    t.title = title;
                }
            }
            _ => {}
        }

        self.mode = Mode::Normal;
        self.input.clear();
        self.input_cursor = 0;
        let _ = self.save();
    }

    pub fn cancel_input(&mut self) {
        self.mode = Mode::Normal;
        self.input.clear();
        self.input_cursor = 0;
    }

    pub fn input_push(&mut self, c: char) {
        let byte_idx = self
            .input
            .char_indices()
            .nth(self.input_cursor)
            .map(|(i, _)| i)
            .unwrap_or(self.input.len());
        self.input.insert(byte_idx, c);
        self.input_cursor += 1;
    }

    pub fn input_backspace(&mut self) {
        if self.input_cursor == 0 {
            return;
        }
        let byte_idx = self
            .input
            .char_indices()
            .nth(self.input_cursor - 1)
            .map(|(i, _)| i)
            .unwrap();
        self.input.remove(byte_idx);
        self.input_cursor -= 1;
    }

    pub fn input_left(&mut self) {
        if self.input_cursor > 0 {
            self.input_cursor -= 1;
        }
    }

    pub fn input_right(&mut self) {
        if self.input_cursor < self.input.chars().count() {
            self.input_cursor += 1;
        }
    }

    pub fn counts(&self) -> (usize, usize) {
        let done = self.todos.iter().filter(|t| t.done).count();
        (done, self.todos.len())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Mutex;

    static ENV_LOCK: Mutex<()> = Mutex::new(());

    struct TempEnv {
        _guard: std::sync::MutexGuard<'static, ()>,
        path: std::path::PathBuf,
    }

    impl TempEnv {
        fn new(name: &str) -> Self {
            let guard = ENV_LOCK.lock().unwrap_or_else(|e| e.into_inner());
            let mut path = std::env::temp_dir();
            path.push(format!(
                "smart-todo-test-{}-{}-{:?}.json",
                name,
                std::process::id(),
                std::thread::current().id()
            ));
            let _ = std::fs::remove_file(&path);
            unsafe {
                std::env::set_var("SMART_TODO_FILE", &path);
            }
            TempEnv { _guard: guard, path }
        }
    }

    impl Drop for TempEnv {
        fn drop(&mut self) {
            let _ = std::fs::remove_file(&self.path);
            unsafe {
                std::env::remove_var("SMART_TODO_FILE");
            }
        }
    }

    fn todo(title: &str, done: bool) -> Todo {
        Todo { title: title.into(), done }
    }

    fn app_with(todos: Vec<Todo>, selected: usize) -> App {
        App {
            todos,
            selected,
            mode: Mode::Normal,
            input: String::new(),
            input_cursor: 0,
            should_quit: false,
            status: None,
            pending_delete: None,
        }
    }

    #[test]
    fn counts_empty() {
        let _env = TempEnv::new("counts_empty");
        let app = app_with(vec![], 0);
        assert_eq!(app.counts(), (0, 0));
    }

    #[test]
    fn counts_mixed() {
        let _env = TempEnv::new("counts_mixed");
        let app = app_with(
            vec![todo("a", true), todo("b", false), todo("c", true)],
            0,
        );
        assert_eq!(app.counts(), (2, 3));
    }

    #[test]
    fn next_stops_at_last() {
        let _env = TempEnv::new("next_stops_at_last");
        let mut app = app_with(vec![todo("a", false), todo("b", false)], 0);
        app.next();
        assert_eq!(app.selected, 1);
        app.next();
        assert_eq!(app.selected, 1);
    }

    #[test]
    fn next_on_empty_does_nothing() {
        let _env = TempEnv::new("next_on_empty_does_nothing");
        let mut app = app_with(vec![], 0);
        app.next();
        assert_eq!(app.selected, 0);
    }

    #[test]
    fn prev_stops_at_zero() {
        let _env = TempEnv::new("prev_stops_at_zero");
        let mut app = app_with(vec![todo("a", false), todo("b", false)], 1);
        app.prev();
        assert_eq!(app.selected, 0);
        app.prev();
        assert_eq!(app.selected, 0);
    }

    #[test]
    fn next_clears_pending_delete() {
        let _env = TempEnv::new("next_clears_pending_delete");
        let mut app = app_with(vec![todo("a", false), todo("b", false)], 0);
        app.request_delete();
        assert_eq!(app.pending_delete, Some(0));
        app.next();
        assert_eq!(app.pending_delete, None);
    }

    #[test]
    fn toggle_done_flips_state() {
        let _env = TempEnv::new("toggle_done_flips_state");
        let mut app = app_with(vec![todo("a", false)], 0);
        app.toggle_done();
        assert!(app.todos[0].done);
        app.toggle_done();
        assert!(!app.todos[0].done);
    }

    #[test]
    fn toggle_done_on_empty_does_nothing() {
        let _env = TempEnv::new("toggle_done_on_empty_does_nothing");
        let mut app = app_with(vec![], 0);
        app.toggle_done();
        assert!(app.todos.is_empty());
    }

    #[test]
    fn delete_requires_two_calls() {
        let _env = TempEnv::new("delete_requires_two_calls");
        let mut app = app_with(vec![todo("a", false), todo("b", false)], 0);

        app.request_delete();
        assert_eq!(app.pending_delete, Some(0));
        assert_eq!(app.todos.len(), 2);

        app.request_delete();
        assert_eq!(app.pending_delete, None);
        assert_eq!(app.todos.len(), 1);
        assert_eq!(app.todos[0].title, "b");
    }

    #[test]
    fn delete_on_empty_does_nothing() {
        let _env = TempEnv::new("delete_on_empty_does_nothing");
        let mut app = app_with(vec![], 0);
        app.request_delete();
        assert_eq!(app.pending_delete, None);
        assert!(app.todos.is_empty());
    }

    #[test]
    fn delete_last_item_moves_selection() {
        let _env = TempEnv::new("delete_last_item_moves_selection");
        let mut app = app_with(vec![todo("a", false), todo("b", false)], 1);
        app.request_delete();
        app.request_delete();
        assert_eq!(app.todos.len(), 1);
        assert_eq!(app.selected, 0);
    }

    #[test]
    fn delete_only_item_keeps_selection_at_zero() {
        let _env = TempEnv::new("delete_only_item_keeps_selection_at_zero");
        let mut app = app_with(vec![todo("a", false)], 0);
        app.request_delete();
        app.request_delete();
        assert!(app.todos.is_empty());
        assert_eq!(app.selected, 0);
    }

    #[test]
    fn delete_second_press_after_move_is_cancelled() {
        let _env = TempEnv::new("delete_second_press_after_move_is_cancelled");
        let mut app = app_with(vec![todo("a", false), todo("b", false)], 0);
        app.request_delete();
        app.next();
        assert_eq!(app.pending_delete, None);
        app.request_delete();
        assert_eq!(app.pending_delete, Some(1));
    }

    #[test]
    fn open_about_sets_mode() {
        let _env = TempEnv::new("open_about_sets_mode");
        let mut app = app_with(vec![], 0);
        app.open_about();
        assert_eq!(app.mode, Mode::About);
    }

    #[test]
    fn open_about_clears_pending_delete() {
        let _env = TempEnv::new("open_about_clears_pending_delete");
        let mut app = app_with(vec![todo("a", false)], 0);
        app.request_delete();
        app.open_about();
        assert_eq!(app.pending_delete, None);
        assert_eq!(app.mode, Mode::About);
    }

    #[test]
    fn close_about_returns_to_normal() {
        let _env = TempEnv::new("close_about_returns_to_normal");
        let mut app = app_with(vec![], 0);
        app.open_about();
        app.close_about();
        assert_eq!(app.mode, Mode::Normal);
    }

    #[test]
    fn start_adding_clears_input() {
        let _env = TempEnv::new("start_adding_clears_input");
        let mut app = app_with(vec![], 0);
        app.input = "stale".into();
        app.input_cursor = 5;
        app.start_adding();
        assert_eq!(app.mode, Mode::Adding);
        assert!(app.input.is_empty());
        assert_eq!(app.input_cursor, 0);
    }

    #[test]
    fn start_editing_loads_title_into_input() {
        let _env = TempEnv::new("start_editing_loads_title_into_input");
        let mut app = app_with(vec![todo("hello", false)], 0);
        app.start_editing();
        assert_eq!(app.mode, Mode::Editing);
        assert_eq!(app.input, "hello");
        assert_eq!(app.input_cursor, 5);
    }

    #[test]
    fn start_editing_on_empty_does_nothing() {
        let _env = TempEnv::new("start_editing_on_empty_does_nothing");
        let mut app = app_with(vec![], 0);
        app.start_editing();
        assert_eq!(app.mode, Mode::Normal);
    }

    #[test]
    fn commit_adding_pushes_todo() {
        let _env = TempEnv::new("commit_adding_pushes_todo");
        let mut app = app_with(vec![], 0);
        app.start_adding();
        app.input = "new task".into();
        app.input_cursor = 8;
        app.commit_input();
        assert_eq!(app.todos.len(), 1);
        assert_eq!(app.todos[0].title, "new task");
        assert!(!app.todos[0].done);
        assert_eq!(app.mode, Mode::Normal);
        assert_eq!(app.selected, 0);
    }

    #[test]
    fn commit_adding_trims_whitespace() {
        let _env = TempEnv::new("commit_adding_trims_whitespace");
        let mut app = app_with(vec![], 0);
        app.start_adding();
        app.input = "   spaced   ".into();
        app.commit_input();
        assert_eq!(app.todos[0].title, "spaced");
    }

    #[test]
    fn commit_adding_empty_is_cancelled() {
        let _env = TempEnv::new("commit_adding_empty_is_cancelled");
        let mut app = app_with(vec![], 0);
        app.start_adding();
        app.input = "   ".into();
        app.commit_input();
        assert!(app.todos.is_empty());
        assert_eq!(app.mode, Mode::Normal);
    }

    #[test]
    fn commit_editing_replaces_title() {
        let _env = TempEnv::new("commit_editing_replaces_title");
        let mut app = app_with(vec![todo("old", false)], 0);
        app.start_editing();
        app.input = "new".into();
        app.input_cursor = 3;
        app.commit_input();
        assert_eq!(app.todos[0].title, "new");
        assert_eq!(app.mode, Mode::Normal);
    }

    #[test]
    fn cancel_input_resets_state() {
        let _env = TempEnv::new("cancel_input_resets_state");
        let mut app = app_with(vec![], 0);
        app.start_adding();
        app.input = "partial".into();
        app.input_cursor = 7;
        app.cancel_input();
        assert_eq!(app.mode, Mode::Normal);
        assert!(app.input.is_empty());
        assert_eq!(app.input_cursor, 0);
    }

    #[test]
    fn input_push_appends_at_end() {
        let _env = TempEnv::new("input_push_appends_at_end");
        let mut app = app_with(vec![], 0);
        app.start_adding();
        app.input_push('a');
        app.input_push('b');
        app.input_push('c');
        assert_eq!(app.input, "abc");
        assert_eq!(app.input_cursor, 3);
    }

    #[test]
    fn input_push_inserts_at_cursor() {
        let _env = TempEnv::new("input_push_inserts_at_cursor");
        let mut app = app_with(vec![], 0);
        app.start_adding();
        app.input_push('a');
        app.input_push('c');
        app.input_left();
        app.input_push('b');
        assert_eq!(app.input, "abc");
        assert_eq!(app.input_cursor, 2);
    }

    #[test]
    fn input_push_handles_multibyte() {
        let _env = TempEnv::new("input_push_handles_multibyte");
        let mut app = app_with(vec![], 0);
        app.start_adding();

        for c in ['c', 'a', 'f', 'é'] {
            app.input_push(c);
        }
        assert_eq!(app.input, "café");
        assert_eq!(app.input_cursor, 4);

        app.input_left();
        assert_eq!(app.input_cursor, 3);

        app.input_push('X');
        assert_eq!(app.input, "cafXé");
        assert_eq!(app.input_cursor, 4);
    }

    #[test]
    fn input_backspace_removes_last() {
        let _env = TempEnv::new("input_backspace_removes_last");
        let mut app = app_with(vec![], 0);
        app.start_adding();
        app.input_push('a');
        app.input_push('b');
        app.input_backspace();
        assert_eq!(app.input, "a");
        assert_eq!(app.input_cursor, 1);
    }

    #[test]
    fn input_backspace_at_start_does_nothing() {
        let _env = TempEnv::new("input_backspace_at_start_does_nothing");
        let mut app = app_with(vec![], 0);
        app.start_adding();
        app.input_push('a');
        app.input_left();
        app.input_backspace();
        assert_eq!(app.input, "a");
        assert_eq!(app.input_cursor, 0);
    }

    #[test]
    fn input_backspace_handles_multibyte() {
        let _env = TempEnv::new("input_backspace_handles_multibyte");
        let mut app = app_with(vec![], 0);
        app.start_adding();
        app.input_push('п');
        app.input_push('р');
        app.input_backspace();
        assert_eq!(app.input, "п");
    }

    #[test]
    fn input_left_right_bounds() {
        let _env = TempEnv::new("input_left_right_bounds");
        let mut app = app_with(vec![], 0);
        app.start_adding();
        app.input_push('a');
        app.input_push('b');
        app.input_left();
        app.input_left();
        app.input_left();
        assert_eq!(app.input_cursor, 0);
        app.input_right();
        app.input_right();
        app.input_right();
        assert_eq!(app.input_cursor, 2);
    }
}