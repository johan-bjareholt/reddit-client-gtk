extern crate glib;
extern crate gtk;
extern crate gdk;
extern crate webkit2gtk;
extern crate redditor;

mod backend_loop;
mod connect_ui;
pub mod command;

use gio::ApplicationExt;
use gtk::prelude::CssProviderExt;
use gtk::prelude::*;

use backend_loop::backend_loop;
use command::ViewChangeCommand;

use std::ops;
use std::rc::{Rc, /*Weak*/};
use std::sync::mpsc::{channel};
use std::sync::{Arc, Mutex, Weak as SyncWeak};

use crate::appop::AppOp;
use crate::ui_builder;

static mut OP: Option<SyncWeak<Mutex<AppOp>>> = None;

pub struct App(Rc<AppInner>);
pub struct AppInner {
    main_window: gtk::ApplicationWindow,
    ui: ui_builder::UI,
    op: Arc<Mutex<AppOp>>
}

// Deref into the contained struct to make usage a bit more ergonomic
impl ops::Deref for App {
    type Target = AppInner;

    fn deref(&self) -> &AppInner {
        &*self.0
    }
}

// // Weak reference to our application struct
// pub struct AppWeak(Weak<AppInner>);

// impl AppWeak {
//     // Upgrade to a strong reference if it still exists
//     pub fn upgrade(&self) -> Option<App> {
//         self.0.upgrade().map(App)
//     }
// }

impl App {
    pub fn new(gtk_app: &gtk::Application) -> App {
        let (tx, rx) = channel::<ViewChangeCommand>();

        glib::set_application_name("redditclient");
        glib::set_prgname(Some("redditclient"));

        let ui = ui_builder::UI::new();

        // Add style provider
        let provider = gtk::CssProvider::new();
        provider.load_from_resource("/org/johan/RedditClient/css/app.css");
        gtk::StyleContext::add_provider_for_screen(
            &gdk::Screen::get_default().expect("Error initializing gtk css provider."),
            &provider,
            gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );

        // Create Window
        let window: gtk::ApplicationWindow = ui
            .builder
            .get_object("main_window")
            .expect("Couldn't find main_window in ui file.");

        window.set_application(Some(gtk_app));

        let op = Arc::new(Mutex::new(AppOp::new(ui.clone(), tx.clone())));

        unsafe {
            OP = Some(Arc::downgrade(&op));
        }

        backend_loop(tx.clone(), rx);
        tx.send(ViewChangeCommand::SubredditView(String::from("all"))).unwrap();

        let app = App(Rc::new(AppInner {
            main_window: window,
            ui,
            op,
        }));

        app.connect_ui();

        app
    }

    pub fn on_startup(application: &gtk::Application) {
        let app = App::new(application);

        application.connect_activate(move |_| {
            app.on_activate();
        });
    }

    fn on_activate(&self) {
        self.main_window.show();

        // FIXME: present() dosen't work currently on wayland because of https://gitlab.gnome.org/GNOME/gtk/issues/624
        self.main_window
            .present_with_time((glib::get_monotonic_time() / 1000) as u32)
    }

    // fn on_shutdown(self) {
    //     self.op.lock().unwrap().quit();
    // }
    
    // Legacy function to get AppOp. This shouldn't be used in new code
    pub fn get_op() -> Option<Arc<Mutex<AppOp>>> {
        unsafe { OP.as_ref().and_then(|x| x.upgrade()) }
    }
}
