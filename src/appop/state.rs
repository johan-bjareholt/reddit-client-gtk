// use gtk;
// use gtk::prelude::*;
// // use libhandy::LeafletExt;

// // use crate::actions::AppState;
// use crate::appop::AppOp;

// #[derive(Debug, Clone, PartialEq)]
// pub enum AppState {
//     Comments,
//     Loading,
//     Login,
//     Subreddit,
//     SubredditSidebar,
//     WebView,
// }

// impl From<String> for AppState {
//     fn from(v: String) -> AppState {
//         match v.as_str() {
//             "comments" => AppState::Comments,
//             "loading" => AppState::Loading,
//             "login" => AppState::Login,
//             "subreddit" => AppState::Subreddit,
//             "subredditsidebar" => AppState::SubredditSidebar,
//             "webview" => AppState::WebView,
//             _ => panic!("Invalid back state type"),
//         }
//     }
// }

// impl From<AppState> for glib::Variant {
//     fn from(v: AppState) -> glib::Variant {
//         match v {
//             AppState::Comments => "comments".to_variant(),
//             AppState::Loading => "loading".to_variant(),
//             AppState::Login => "login".to_variant(),
//             AppState::Subreddit => "subreddit".to_variant(),
//             AppState::SubredditSidebar => "subredditsidebar".to_variant(),
//             AppState::WebView => "webview".to_variant(),
//         }
//     }
// }

// impl AppOp {
//     pub fn set_state(&mut self, state: AppState) {
//         self.state = state;
//         // let stack = self
//         //     .ui
//         //     .builder
//         //     .get_object::<gtk::Stack>("room_view_stack")
//         //     .expect("Can't find room_view_stack in ui file.");
//         // let headerbar = self
//         //     .ui
//         //     .builder
//         //     .get_object::<gtk::HeaderBar>("room_header_bar")
//         //     .expect("Can't find room_header_bar in ui file.");

//         let widget_name = match self.state {
//             AppState::Comments => "comments",
//             AppState::Loading => "loading",
//             AppState::Login => "login",
//             AppState::Subreddit => "subreddit",
//             AppState::SubredditSidebar => "subredditsidebar",
//             AppState::WebView => "webview",
//         };

//         self.ui
//             .builder
//             .get_object::<gtk::Stack>("main_content_stack")
//             .expect("Can't find main_content_stack in ui file.")
//             .set_visible_child_name(widget_name);

//         // Setting headerbar
//         // let bar_name = match self.state {
//         //     AppState::Login => "login",
//         //     AppState::Directory => "back",
//         //     AppState::Loading => "login",
//         //     AppState::AccountSettings => "account-settings",
//         //     AppState::RoomSettings => "room-settings",
//         //     AppState::MediaViewer => "media-viewer",
//         //     _ => "normal",
//         // };

//         self.ui
//             .builder
//             .get_object::<gtk::Stack>("headerbar_stack")
//             .expect("Can't find headerbar_stack in ui file.")
//             .set_visible_child_name(widget_name);
//     }
// }
