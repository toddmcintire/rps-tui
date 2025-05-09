use std::{error::Error, io};

use ratatui::{
    backend::{Backend, CrosstermBackend},
    crossterm::{
        event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
        execute,
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    },
    Terminal,
};

mod app;
mod ui;

use crate::{
    app::{App, GameChoice, GameScreen},
    ui::ui,
};

fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> io::Result<bool> {
    loop {
        terminal.draw(|f| ui(f, app))?;

        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Release {
                // Skip events that are not KeyEventKind::Press
                continue;
            }
            match app.current_game_screen {
                GameScreen::Choice => match key.code { 
                    KeyCode::Left => {
                        if app.selected_tab > 0 {
                            app.selected_tab -= 1;
                        }
                    }

                    KeyCode::Right => {
                        if app.selected_tab < 2 {
                            app.selected_tab += 1;
                        } 
                    }

                    KeyCode::Char('q') => {
                        return Ok(true);
                    }

                    KeyCode::Enter => {
                        match app.selected_tab {
                            0 => {
                                app.current_game_choice = Some(GameChoice::Rock);
                            }
                            1 => {
                                app.current_game_choice = Some(GameChoice::Paper);
                            }
                            2 => {
                                app.current_game_choice = Some(GameChoice::Scissor);
                            }
                            _ => {}
                        }
                        app.current_game_screen = GameScreen::Playing;
                        app.play_game();
                        //somehow show screen
                        app.current_game_screen = GameScreen::Choice;
                    }

                    _ => {}
                },

                // CurrentScreen::Exiting => match key.code {
                //     KeyCode::Char('y') => {
                //         return Ok(true);
                //     }

                //     KeyCode::Char('n') | KeyCode::Char('q') => {
                //         return Ok(false);
                //     }

                //     _ => {}
                // },

                // CurrentScreen::Editing if key.kind == KeyEventKind::Press => {
                //     match key.code {
                //         KeyCode::Enter => {
                //             if let Some(editing) = &app.currently_editing {
                //                 match editing {
                //                     CurrentlyEditing::Key => {
                //                         app.currently_editing = Some(CurrentlyEditing::Value);
                //                     }

                //                     CurrentlyEditing::Value => {
                //                         app.save_key_value();
                //                         app.current_screen = CurrentScreen::Main;
                //                     }
                //                 }
                //             }
                //         }

                //         KeyCode::Backspace => {
                //             if let Some(editing) = &app.currently_editing {
                //                 match editing {
                //                     CurrentlyEditing::Key => {
                //                         app.key_input.pop();
                //                     }
                //                     CurrentlyEditing::Value => {
                //                         app.value_input.pop();
                //                     }
                //                 }
                //             }
                //         }

                //         KeyCode::Esc => {
                //             app.current_screen = CurrentScreen::Main;
                //             app.currently_editing = None;
                //         }

                //         KeyCode::Tab => {
                //             app.toggle_editing();
                //         }

                //         KeyCode::Char(value) => {
                //             if let Some(editing) = &app.currently_editing {
                //                 match editing { 
                //                     CurrentlyEditing::Key => {
                //                         app.key_input.push(value);
                //                     }

                //                     CurrentlyEditing::Value => {
                //                         app.value_input.push(value);
                //                     }
                //                 }
                //             }
                //         }

                //         _ => {}
                //     }
                // }

                _ => {}
            }
        }
        
    }
}

fn main() -> Result<(), Box<dyn Error>>{
    enable_raw_mode()?;
    let mut stderr = io::stderr();
    execute!(stderr, EnterAlternateScreen, EnableMouseCapture)?;

    let backend = CrosstermBackend::new(stderr);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new();
    let res = run_app(&mut terminal, &mut app);

    //restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Ok(do_print) = res {
        if do_print {
            app.print_score();
        }
    } else if let Err(err) = res {
        println!("{err:?}");
    }

    Ok(())
}
