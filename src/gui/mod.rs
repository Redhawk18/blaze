use std::path::PathBuf;

use iced::widget::text_input;
use iced::widget::Column;
use iced::{theme, Application, Command, Element, Subscription};

mod elements;
mod file_dialog;

pub use elements::menu_bar;

#[derive(Debug, Clone)]
pub enum Message {
    TextUpdate(String),

    //menu bar
    NewFile,
    OpenFile,
    OpenFolder,
    Save,
    SaveAs,
    Quit,

    //tabs
    TabNew(FileTab),
    TabSelected(elements::TabId),
    TabClosed(elements::TabId),
}

pub struct Blaze {
    tabs: Tabs,
}

struct Tabs {
    tab_bar: iced_aw::TabBar<Message, elements::TabId>,
    data: Vec<Tab>,
}

impl Default for Tabs {
    fn default() -> Self {
        Self {
            tab_bar: iced_aw::TabBar::new(Message::TabSelected),
            data: Vec::new(),
        }
    }
}

pub enum Tab {
    File(FileTab),
}

#[derive(Debug, Clone)]
pub struct FileTab {
    id: elements::TabId,
    text: String,
    path: PathBuf,
}

impl Default for FileTab {
    fn default() -> Self {
        Self {
            id: elements::TabId::File,
            text: String::default(),
            path: PathBuf::default(),
        }
    }
}

impl Application for Blaze {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Theme = theme::Theme;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (
            Blaze {
                tabs: Tabs::default(),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("code editor")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::TextUpdate(text) => match self.active_tab {
                Some(index) => {
                    let tab = self.tabs.get_mut(index).unwrap();
                    tab.text = text;
                }
                None => return self.update(Message::TabNew(FileTab::default())),
            },

            Message::NewFile => return self.update(Message::TabNew(FileTab::default())),

            Message::OpenFile => {
                let (file_contents, path) = file_dialog::pick_file();
                let Ok(text) = file_contents else { return Command::none() };
                let Some(index) = self.active_tab else {
                    return self.update(Message::TabNew(FileTab::default()))
                };

                self.tabs[index].path = path;
                return self.update(Message::TextUpdate(text));
            }

            Message::OpenFolder => file_dialog::pick_folder(),

            Message::Save => match self.active_tab {
                Some(index) => {
                    let tab = self.tabs.get(index).unwrap();
                    file_dialog::save_file(tab.text.as_str(), tab.path.as_path()).unwrap();
                }
                None => return Command::none(),
            },

            Message::SaveAs => match self.active_tab {
                Some(index) => {
                    let tab = self.tabs.get(index).unwrap();
                    file_dialog::save_as(tab.text.as_str(), tab.path.as_path()).unwrap();
                }
                None => return Command::none(),
            },

            Message::Quit => return iced::window::close(),

            Message::TabNew(tab) => {
                log::info!("New tab");
                self.tabs.push(tab);
                self.active_tab = Some(self.tabs.len() - 1);
            }

            Message::TabSelected(id) => {
                //log::info!("Selected tab {}", index);
                //self.active_tab = Some(index);
            }

            Message::TabClosed(id) => {
                //log::info!("Closed tab {}", index);
            }
        }

        Command::none()
    }

    fn view(&self) -> Element<Message> {
        let mut c = Column::new().push(menu_bar());

        if !self.tabs.is_empty() {
            c = c.push(elements::tab_header(&self.tabs, self.active_tab.unwrap()));
            c = c.push(
                text_input(
                    "",
                    self.tabs
                        .get(self.active_tab.unwrap())
                        .unwrap()
                        .text
                        .as_str(),
                )
                .on_input(Message::TextUpdate),
            );
        }

        c.into()
    }

    fn subscription(&self) -> Subscription<Message> {
        Subscription::none()
    }
}
