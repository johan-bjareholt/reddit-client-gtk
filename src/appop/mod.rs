use std::sync::mpsc::Sender;

use gtk;
use gtk::prelude::*;

// use crate::backend;
use crate::app::command::ViewChangeCommand;
// use crate::globals;

// use crate::actions::AppState;
use crate::ui_builder;
// use crate::widgets;

mod comments;
mod state;
mod subreddit;
mod webview;

// use state::AppState;

pub struct AppOp {
    pub ui: ui_builder::UI,
    pub backend: Sender<ViewChangeCommand>,
    // pub state: AppState,
    // pub subreddit: String,
    pub is_logged_in: bool,
}

impl AppOp {
    pub fn new(ui: ui_builder::UI, tx: Sender<ViewChangeCommand>) -> AppOp {
        AppOp {
            ui: ui,
            backend: tx,
            // state: AppState::Subreddit,
            // subreddit: "all".to_string(),
            is_logged_in: false,
        }
    }

    // pub fn init(&mut self) {
    //     self.set_state(AppState::Loading);
    // }

    pub fn activate(&self) {
        let window: gtk::Window = self
            .ui
            .builder
            .get_object("main_window")
            .expect("Couldn't find main_window in ui file.");
        window.show();
        window.present();
    }

    pub fn quit(&self) {
        // self.cache_rooms();
        // self.disconnect();
    }

    fn update_headerbar(&self, widget: gtk::Widget) {
        let headerbar_stack = self.ui
            .builder
            .get_object::<gtk::Stack>("headerbar_stack")
            .expect("Can't find headerbar_stack in ui file.");

        for child in headerbar_stack.get_children() {
            headerbar_stack.remove(&child);
        }

        headerbar_stack.add(&widget);
        headerbar_stack.show();
    }

    fn update_view(&self, widget: gtk::Widget) {
        let main_content_stack = self.ui
            .builder
            .get_object::<gtk::Stack>("main_content_stack")
            .expect("Can't find main_content_stack in ui file.");

        for child in main_content_stack.get_children() {
            main_content_stack.remove(&child);
        }

        main_content_stack.add(&widget);
        main_content_stack.show();
    }
}
