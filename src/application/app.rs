#[cfg(target_os = "macos")]
use super::macos::macos_register_open_files_handler;
use crate::{
    AppWin, MainWin, MainWinMsg, MenuEvent, MenuEventReplyMsg, TreeView, TreeViewMsg,
    main_win_settings, menu_events, parse_newick, prepare_app_menu,
};
use iced::Element;
use iced::Subscription;
use iced::Task;
// use iced::event::Event as RuntimeEvent;
// use iced::event::{Status as EventStatus, listen, listen_raw, listen_url};
use iced::exit;
use iced::futures::channel::mpsc::Sender;
use iced::widget;
use iced::window::Event as WindowEvent;
use iced::window::Id as WinId;
use iced::window::close as close_window;
use iced::window::gain_focus;
use iced::window::open as open_window;
use iced::window::{close_events, close_requests, events, open_events};
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Default)]
pub struct App {
    main_win_id: Option<WinId>,
    windows: HashMap<WinId, AppWin>,
    menu_events_sender: Option<Sender<MenuEventReplyMsg>>,
    menu: Option<muda::Menu>,
}

#[derive(Debug, Clone)]
pub enum AppMsg {
    AppInitialized,
    MainWinMsg(MainWinMsg),
    MenuEvent(MenuEvent),
    MenuEventsSender(Sender<MenuEventReplyMsg>),
    OpenMainWin,
    Path(PathBuf),
    // RawEvent(RuntimeEvent, EventStatus, WinId),
    // RuntimeEvent(RuntimeEvent),
    TerminateApp,
    TerminationConfirmed,
    // Url(String),
    Win(WinId, WindowEvent),
    WinCloseRequested(WinId),
    WinClosed(WinId),
    WinOpened(WinId),
}

impl App {
    pub fn view(&self, id: WinId) -> Element<AppMsg> {
        match &self.windows.get(&id) {
            Some(AppWin::MainWin(x)) => x.view().map(AppMsg::MainWinMsg),
            None => widget::horizontal_space().into(),
        }
    }

    pub fn update(&mut self, app_msg: AppMsg) -> Task<AppMsg> {
        match app_msg {
            // AppMsg::RawEvent(e, _status, _id) => match e {
            //     RuntimeEvent::Keyboard(_event) => Task::none(),
            //     RuntimeEvent::Mouse(_event) => Task::none(),
            //     RuntimeEvent::Window(event) => match event {
            //         WindowEvent::Opened {
            //             position: _,
            //             size: _,
            //         } => Task::none(),
            //         WindowEvent::Closed => Task::none(),
            //         WindowEvent::Moved(_point) => Task::none(),
            //         WindowEvent::Resized(_size) => Task::none(),
            //         WindowEvent::RedrawRequested(_instant) => Task::none(),
            //         WindowEvent::CloseRequested => Task::none(),
            //         WindowEvent::Focused => Task::none(),
            //         WindowEvent::Unfocused => Task::none(),
            //         WindowEvent::FileHovered(_path_buf) => Task::none(),
            //         WindowEvent::FileDropped(_path_buf) => Task::none(),
            //         WindowEvent::FilesHoveredLeft => Task::none(),
            //         // _ => {
            //         //     println!("{event:?} {status:?} {id:?}");
            //         //     Task::none()
            //         // }
            //     },
            //     RuntimeEvent::Touch(_event) => Task::none(),
            //     RuntimeEvent::InputMethod(_event) => Task::none(),
            //     // _ => {
            //     //     println!("{e:?} {status:?} {id:?}");
            //     //     Task::none()
            //     // }
            // },
            // AppMsg::RuntimeEvent(_e) => {
            //     // println!("{e:?}");
            //     Task::none()
            // }
            // AppMsg::Url(_url) => {
            //     // println!("{url}");
            //     Task::none()
            // }
            AppMsg::Win(_id, e) => match e {
                // WindowEvent::Opened { position: _, size: _, } => Task::none(),
                // WindowEvent::Closed => Task::none(),
                // WindowEvent::Moved(_point) => Task::none(),
                // WindowEvent::Resized(_size) => Task::none(),
                // WindowEvent::RedrawRequested(_instant) => Task::none(),
                // WindowEvent::CloseRequested => Task::none(),
                // WindowEvent::Focused => Task::none(),
                // WindowEvent::Unfocused => Task::none(),
                WindowEvent::FileHovered(path_buf) => {
                    println!("{path_buf:?}");
                    Task::none()
                }
                WindowEvent::FileDropped(path_buf) => Task::done(AppMsg::Path(path_buf)),
                // WindowEvent::FilesHoveredLeft => Task::none(),
                _ => {
                    // println!("{id:?} {e:?}");
                    Task::none()
                }
            },
            AppMsg::Path(path_buf) => match path_buf.extension() {
                Some(ext) => match ext.to_str() {
                    Some("newick" | "tre") => Task::perform(read_text_file(path_buf.clone()), {
                        |data| {
                            let tree_data = parse_newick(data);
                            AppMsg::MainWinMsg(MainWinMsg::TreeViewMsg(
                                TreeViewMsg::TreeDataUpdated(tree_data),
                            ))
                        }
                    })
                    .chain(Task::done(AppMsg::MainWinMsg(MainWinMsg::Title(
                        String::from(
                            path_buf
                                .file_name()
                                .unwrap_or_default()
                                .to_str()
                                .unwrap_or_default(),
                        ),
                    )))),
                    _ => Task::none(),
                },
                None => Task::none(),
            },
            AppMsg::TerminationConfirmed => exit(),
            AppMsg::MenuEventsSender(mut sender) => {
                let _ = sender.try_send(MenuEventReplyMsg::Ack);
                self.menu_events_sender = Some(sender);
                Task::none()
            }
            AppMsg::MenuEvent(menu_event) => {
                let _ = menu_event_reply(self, MenuEventReplyMsg::Ack);
                match menu_event {
                    MenuEvent::OpenFile => Task::future(choose_file()),
                    MenuEvent::Save => Task::none(),
                    MenuEvent::CloseWindow => close_last_window(self),
                    MenuEvent::Quit => close_last_window(self),
                    MenuEvent::QuitInternal => Task::none(),
                    MenuEvent::Undefined(s) => Task::done(AppMsg::Path(s.into())),
                }
            }
            AppMsg::TerminateApp => match menu_event_reply(self, MenuEventReplyMsg::Terminate) {
                Ok(_) => Task::none(),
                Err(_) => Task::done(AppMsg::TerminationConfirmed),
            },

            AppMsg::AppInitialized => {
                let menu = prepare_app_menu();
                #[cfg(target_os = "macos")]
                menu.init_for_nsapp();
                self.menu = Some(menu);
                Task::done(AppMsg::OpenMainWin)
            }
            AppMsg::OpenMainWin => open_main_window(self),
            AppMsg::MainWinMsg(main_win_msg) => {
                let app_task = match main_win_msg {
                    MainWinMsg::OpenFile => Task::future(choose_file()),
                    _ => Task::none(),
                };
                let main_win_task = match self.main_win_id {
                    Some(id) => match self.windows.get_mut(&id) {
                        Some(AppWin::MainWin(w)) => w.update(main_win_msg).map(AppMsg::MainWinMsg),
                        None => Task::none(),
                    },
                    None => Task::none(),
                };

                app_task.chain(main_win_task)
            }
            AppMsg::WinOpened(id) => match self.windows.get(&id) {
                Some(AppWin::MainWin(_)) => {
                    // Task::done(AppMsg::Path(PathBuf::from(
                    //     "/Users/karolis/Desktop/tmp3.newick",
                    // )))
                    Task::none()
                }
                None => Task::none(),
            },
            AppMsg::WinCloseRequested(id) => match self.windows.get(&id) {
                Some(AppWin::MainWin(_)) => {
                    muda::MenuEvent::send(muda::MenuEvent {
                        id: muda::MenuId(MenuEvent::QuitInternal.to_string()),
                    });
                    close_window(id)
                }
                None => Task::none(),
            },
            AppMsg::WinClosed(id) => {
                if self.main_win_id == Some(id) {
                    self.main_win_id = None
                };
                match self.windows.remove(&id) {
                    Some(AppWin::MainWin(_)) => Task::done(AppMsg::TerminateApp),
                    None => Task::none(),
                }
            }
        }
    }

    pub fn title(&self, id: WinId) -> String {
        match self.windows.get(&id) {
            Some(AppWin::MainWin(w)) => w.title(),
            None => format!("{id:?}"),
        }
    }

    pub fn subscription(&self) -> Subscription<AppMsg> {
        subscriptions()
    }

    pub fn new() -> (Self, Task<AppMsg>) {
        let app = Self {
            ..Default::default()
        };
        #[cfg(target_os = "macos")]
        macos_register_open_files_handler();
        (app, Task::done(AppMsg::AppInitialized))
    }
}

fn menu_event_reply(app: &mut App, msg: MenuEventReplyMsg) -> Result<(), ()> {
    match app.menu_events_sender.take() {
        Some(mut sender) => {
            let _ = sender.try_send(msg);
            app.menu_events_sender = Some(sender);
            Ok(())
        }
        None => Err(()),
    }
}

fn subscriptions() -> Subscription<AppMsg> {
    let open_events = open_events().map(AppMsg::WinOpened);
    let close_requests = close_requests().map(AppMsg::WinCloseRequested);
    let close_events = close_events().map(AppMsg::WinClosed);
    let all_window_events = events().map(|(id, e)| AppMsg::Win(id, e));
    // let url_events = listen_url().map(AppMsg::Url);
    // let runtime_events = listen().map(AppMsg::RuntimeEvent);
    // let raw_events = listen_raw(|e, status, id| Some(AppMsg::RawEvent(e, status, id)));
    let menu_events = menu_events();
    Subscription::batch([
        open_events,
        close_requests,
        close_events,
        menu_events,
        all_window_events,
        // url_events,
        // runtime_events,
        // raw_events,
    ])
}

fn open_main_window(app: &mut App) -> Task<AppMsg> {
    let (win_id, task) = open_window(main_win_settings());
    let main_win = Box::new(MainWin {
        tree_view: TreeView::new(),
        ..Default::default()
    });
    let win: AppWin = AppWin::MainWin(main_win);
    app.windows.insert(win_id, win);
    app.main_win_id = Some(win_id);
    task.discard().chain(gain_focus(win_id))
}

fn close_last_window(app: &App) -> Task<AppMsg> {
    {
        let nwin = app.windows.len();
        if nwin == 1 {
            match app.windows.keys().last() {
                Some(id) => Task::done(AppMsg::WinCloseRequested(*id)),
                None => Task::none(),
            }
        } else if nwin == 0 {
            Task::done(AppMsg::TerminateApp)
        } else {
            Task::none()
        }
    }
}

async fn choose_file() -> AppMsg {
    let chosen = rfd::AsyncFileDialog::new()
        .add_filter("newick", &["newick"])
        .add_filter("tre", &["tre"])
        // .add_filter("tree", &["tree"])
        // .add_filter("trees", &["trees"])
        // .add_filter("text", &["txt"])
        // .add_filter("fasta", &["fasta", "fsa", "mfa", "fa"])
        // .set_directory("/")
        .pick_file()
        .await;

    AppMsg::Path(match chosen {
        Some(pb) => pb.path().into(),
        None => PathBuf::new(),
    })
}

pub async fn read_text_file(path_buf: PathBuf) -> String {
    use tokio::fs::read;
    let data = read(path_buf)
        .await
        .map_err(|e| {
            eprintln!("IO error: {:?}", e);
        })
        .ok()
        .unwrap();
    String::from_utf8(data).unwrap()
}
