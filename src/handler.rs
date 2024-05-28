use crate::api::{get_url, query_anime};
use crate::app::{App, AppResult, ResultList};
use crate::handler::KeyCode::*;

use crossterm::event::*;
use tui_input::backend::crossterm::EventHandler;

use std::process::Command;

/// Handles the key events and updates the state of [`App`].
pub async fn handle_key_events(key_event: KeyEvent, app: &mut App) -> AppResult<()> {
    match key_event {
        KeyEvent {
            code: KeyCode::Char('c'),
            modifiers: KeyModifiers::CONTROL,
            ..
        } => {
            app.quit();
        }
        KeyEvent {
            code: KeyCode::Esc, ..
        } => {
            app.quit();
        }
        KeyEvent {
            code: KeyCode::Enter,
            ..
        } => {
            if app.editing {
                app.list = ResultList {
                    state: app.list.state.clone(),
                    items: query_anime(app.input.to_string()).await.unwrap().results,
                    last_selected: app.list.last_selected,
                };
                app.chunk = 2;
                app.editing = false;
                app.input.reset();
            }
        }
        _ => {
            if app.editing {
                app.input.handle_event(&Event::Key(key_event));
            }
        }
    }
    if !app.editing {
        match key_event.code {
            Char('j') | Down => app.list.next(),
            Char('k') | Up => app.list.previous(),
            Char('g') => app.go_top(),
            Char('G') => app.go_bottom(),
            KeyCode::Enter => {
                if let Some(i) = app.list.state.selected() {
                    let mut url = String::new();
                    for source in get_url(format!("{}-episode-1", app.list.items[i].id.clone()))
                        .await
                        .unwrap()
                        .sources
                    {
                        if source.quality == "1080p" {
                            url = source.url
                        }
                    }

                    Command::new("mpv")
                        .arg(url)
                        .output()
                        .expect("Failed to execute `mpv`");
                }
            }
            _ => {}
        }
    }
    Ok(())
}
