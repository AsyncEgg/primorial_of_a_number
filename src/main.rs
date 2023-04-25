use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{error::Error, io, thread};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout},
    style::{Color, Style, Modifier},
    widgets::{Block, Borders, Paragraph},
    Frame, Terminal,
};
use unicode_width::UnicodeWidthStr;

use primorial_of_a_number::{primorial, WriteMode, write_biguint_to_file, read_file_to_biguint, ReadMode};

#[derive(Debug)]
enum OutputSwitch {
    Enabled,
    Disabled
}

enum FileWindowMode {
    Read,
    None,
    Write
}

enum OutputMode {
    Message,
    Number,
}

struct App {
    input: String,
    output: String,
    primes_status: String,
    primorial_status: String,
    file_status: String,
    scroll_position: u16,
    write_mode: WriteMode,
    output_mode: OutputMode, 
    file_window_mode: FileWindowMode,
    output_switch: OutputSwitch
}

impl Default for App {
    fn default() -> App {
        App {
            input: String::new(),
            output: String::new(),
            primes_status: String::new(),
            primorial_status: String::new(),
            file_status: String::new(),
            scroll_position: 0,
            write_mode: WriteMode::None,
            output_mode: OutputMode::Message,
            file_window_mode: FileWindowMode::None,
            output_switch: OutputSwitch::Enabled
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let app = App::default();
    let res = run_app(&mut terminal, app);

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui(f, &app))?;

        if let Event::Key(key) = event::read()? {

                match key.code {

                    KeyCode::Down => {
                        app.scroll_position += 1
                    }
                    KeyCode::Up => {
                        if app.scroll_position != 0 {
                            app.scroll_position -= 1
                        }
                    }

                    KeyCode::Right => {
                        app.scroll_position += 10
                    }
                    KeyCode::Left => {
                        if app.scroll_position != 0 && !((app.scroll_position as i32 - 9) <= 0) {
                            app.scroll_position -= 10
                        }
                    }

                    KeyCode::Enter => {
                        app.file_window_mode = FileWindowMode::None;
                        app.scroll_position = 0;
                        
                        let drain: String = app.input.drain(..).collect();

                        if drain == String::from("quit") || drain == String::from("q") {
                            return Ok(());
                        }

                        if !drain.chars().all(|ch| ch.is_digit(10)) || drain == String::new() {
                            app.output_mode = OutputMode::Message;
                            app.output = String::from("Error: Value is not a number!")
                        } else {
                            app.output_mode = OutputMode::Number;
                            let (output,prime_duration,primorial_duration)  = primorial(drain.parse::<usize>().unwrap());

                            match app.output_switch {
                                OutputSwitch::Enabled => {
                                    app.output = output.to_str_radix(10);
                                    
                                }
                                OutputSwitch::Disabled => app.output = "Output disabled; Type output enable to enable output".to_string()
                            }
                            
                            app.primes_status = format!("{:?}",prime_duration);
                            app.primorial_status = format!("{:?}",primorial_duration);

                            app.file_status = thread::spawn(move ||{
                                return write_biguint_to_file(output, &app.write_mode)
                            }).join().unwrap().unwrap();

                        }

                        if drain.contains("output") {
                            app.output_mode = OutputMode::Message;
                            let option = drain.split_once(" ");

                            match option {
                                Some(("output", "enable")) => {
                                    app.output_switch = OutputSwitch::Enabled;
                                    app.output = String::from("Output enabled")
                                }
                                Some(("output", "disable")) => {
                                    app.output_switch = OutputSwitch::Disabled;
                                    app.output = String::from("Output disabled")

                                }
                                Some((_,_)) => {app.output = String::from("Error: Invalid write option; Valid options: \"enable\" \"disable\"")}
                                None => {app.output = String::from("Error: Invalid write option; Valid options: \"enable\" \"disable\"")}
                            }
                        }

                        if drain.contains("write") {
                            app.file_window_mode = FileWindowMode::Write;
                            app.output_mode = OutputMode::Message;
                            let option = drain.split_once(" ");

                            match option {
                                Some(("write", "bin")) => {
                                    app.write_mode = WriteMode::Bin;
                                    app.output = String::from("Write mode bin selected, enter value; value will be saved")
                                }
                                Some(("write", "txt")) => {
                                    app.write_mode = WriteMode::Txt;
                                    app.output = String::from("Write mode txt selected, enter value; value will be saved")

                                }
                                Some(("write", "none")) => {
                                    app.write_mode = WriteMode::None;
                                    app.output = String::from("Write mode none selected, enter value; value will be saved")
                                }
                                Some((_,_)) => {app.output = String::from("Error: Invalid write option; Valid options: \"bin\" \"txt\" \"none\"")}
                                None => {
                                    app.output = String::from("Error: Invalid write option; Valid options: \"bin\" \"txt\" \"none\"")
                                }
                            }
                        }

                        if drain.contains("read") {
                            app.file_window_mode = FileWindowMode::Read;
                            app.output_mode = OutputMode::Message;
                            app.primes_status = String::from("No number generated");
                            app.primorial_status = String::from("No number generated");
                            let option = drain.split_once(" ");

                            match option {
                                Some(("read", "bin")) => {
                                    app.output_mode = OutputMode::Number;
                                    (app.file_status, app.output, _) = read_file_to_biguint(&ReadMode::Bin).unwrap()
                                }
                                Some(("read", "txt")) => {
                                    app.output_mode = OutputMode::Number;
                                    (app.file_status, app.output, _) = read_file_to_biguint(&ReadMode::Txt).unwrap()

                                }
                                Some((_,_)) => {app.output = String::from("Error: Invalid write option; Valid options: \"bin\" \"txt\"")}

                                None => {
                                    app.output = String::from("Error: Invalid write option; Valid options: \"bin\" \"txt\"")
                                }
                            }
                        }

                    }
                    KeyCode::Char(c) => {
                        app.input.push(c);
                    }
                    KeyCode::Backspace => {
                        app.input.pop();
                    }
                    _ => {}
            }
        }
    }
}

fn ui<B: Backend>(f: &mut Frame<B>, app: &App) {
    let outer_chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(0)
        .constraints(
            [
                Constraint::Min(0),
                Constraint::Length(4)
            ]
            .as_ref(),
        )
        .split(f.size());

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(0)
        .constraints(
            [
                Constraint::Length(1),
                Constraint::Length(1),
                Constraint::Length(3),
                Constraint::Min(1),
            ]
            .as_ref(),
        )
        .split(outer_chunks[0]);
    
    let bottom_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Length(21),
                Constraint::Min(0),
            ]
            .as_ref(),
        )
        .split(chunks[3]);

    let input = Paragraph::new(app.input.as_ref())
        .style(Style::default().add_modifier(Modifier::BOLD).fg(Color::Yellow))
        .block(Block::default().borders(Borders::ALL).title("Primorial Number Generator"));
    f.render_widget(input, chunks[2]);
    f.set_cursor(
        chunks[1].x + app.input.width() as u16 + 1,
        chunks[1].y + 2,
    );

    let left_chunks = Layout::default()
    .direction(Direction::Vertical)
    .constraints(
        [
            Constraint::Percentage(34),
            Constraint::Percentage(34),
            Constraint::Percentage(31),
        ]
        .as_ref(),
    )
    .split(bottom_chunks[0]);

    let status = Paragraph::new(format!("{}",app.primes_status))
        .style(Style::default().add_modifier(Modifier::ITALIC))
        .block(Block::default().borders(Borders::ALL).title("Prime Duration"));

    f.render_widget(status, left_chunks[0]);

    let status = Paragraph::new(format!("{}",app.primorial_status))
        .style(Style::default().add_modifier(Modifier::ITALIC))
        .block(Block::default().borders(Borders::ALL).title("Primorial Duration"));

    f.render_widget(status, left_chunks[1]);

    match app.file_window_mode {
        FileWindowMode::Write => {
            let status = Paragraph::new(format!("{}\nWrite: {:?}",app.file_status, app.write_mode))
                .style(Style::default().add_modifier(Modifier::ITALIC))
                .block(Block::default().borders(Borders::ALL).title("File Duration"));

            f.render_widget(status, left_chunks[2]);
        }
        FileWindowMode::Read => {
            let status = Paragraph::new(format!("{}\nRead",app.file_status))
                .style(Style::default().add_modifier(Modifier::ITALIC))
                .block(Block::default().borders(Borders::ALL).title("File Duration"));

            f.render_widget(status, left_chunks[2]);
        }
        FileWindowMode::None => {
            let status = Paragraph::new(format!("{}",app.file_status))
                .style(Style::default().add_modifier(Modifier::ITALIC))
                .block(Block::default().borders(Borders::ALL).title("File Duration"));

            f.render_widget(status, left_chunks[2]);
        }
    }

    let chunk_width = bottom_chunks[1].width -  9;

    let output;

    match app.output_switch {
        OutputSwitch::Enabled => {
            output = string_wrap(&app.output, chunk_width, &app.output_mode);
        }
        OutputSwitch::Disabled => {
            output = "Output disabled; Type output enable to enable output".to_string();
        }
    }
    

    let output = Paragraph::new(output)
        .style(Style::default().fg(Color::White))
        .block(Block::default().title(format!("Output: {:?}", app.output_switch)).borders(Borders::ALL))
        .scroll((app.scroll_position, 0));

    f.render_widget(output, bottom_chunks[1]);

    let instructions = String::from("type \"q\" or \"quit\" to quit | press ENTER to submit number | use arrow keys to scroll | type \"write\" to write data to file | type \"read\" to Read Data from file | type \"output\" to enable or disable output");

    let instructions = string_wrap(&instructions, outer_chunks[1].width - 3, &OutputMode::Message);

    let instructions = Paragraph::new(instructions)
        .style(Style::default().add_modifier(Modifier::BOLD).fg(Color::Yellow))
        .block(Block::default().borders(Borders::ALL));

    f.render_widget(instructions, outer_chunks[1]);
}

fn string_wrap(string: &String, chunk_width: u16, output_mode: &OutputMode) -> String {
    string
    .chars()
    .collect::<Vec<_>>()
    .chunks(chunk_width.into())
    .into_iter()
    .map(|chunk| chunk.into_iter().collect::<String>())
    .enumerate()
    .map(|(i,s)|{
        return match output_mode {
            OutputMode::Number => format!("{}~ {}",i+1,s ).to_string(),
            OutputMode::Message => s
        }
    })
    .collect::<Vec<String>>()
    .join("\n")
}