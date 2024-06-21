use crossterm::event::{self, KeyCode, KeyEventKind};
use ratatui::prelude::{Backend, Terminal};

use crate::client::Client;
use crate::constants::{Action, Screen};
use crate::filesystem;
use crate::state::State;
use crate::view::View;

pub struct Controller {
    pub state: State,
    client: Client,
}

impl Controller {
    pub fn new() -> Self {
        Self {
            state: State::default(),
            client: Client::default(),
        }
    }

    pub fn handle_action(&mut self, action: Action) -> Result<(), Box<dyn std::error::Error>> {
        match self.state.screen {
            Screen::Main => match action {
                Action::Exit => self.state.running = false,
                Action::GetTasks => {
                    self.state.task_list = self.client.get_tasks()?;
                }
                Action::OpenAddScreen => self.state.screen = Screen::Add,
                _ => println!("rest"),
            },
            _ => println!("yay"),
        }
        Ok(())
    }

    pub fn handle_events(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if event::poll(std::time::Duration::from_millis(16))? {
            if let event::Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    let action = match key.code {
                        KeyCode::Char('q') => Action::Exit,
                        _ => Action::Input,
                    };
                    self.handle_action(action)?;
                }
            }
        }
        Ok(())
    }

    pub fn init_controller(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        filesystem::create_config_folder()?;
        self.client.open_connection()?;
        self.state.running = true;
        let tasks = self.client.get_tasks()?;
        self.state.task_list = tasks;
        Ok(())
    }

    pub fn exit(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.client.close_connection()?;
        Ok(())
    }

    pub fn run<B: Backend>(
        &mut self,
        terminal: &mut Terminal<B>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.init_controller()?;

        while self.state.running {
            self.handle_events()?;
            View::draw(terminal, &self.state)?;
        }
        Ok(())
    }
}
