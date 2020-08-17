use gtk;
use gtk::prelude::*;

use crate::app::command::ViewChangeCommand;
use crate::appop::AppOp;
use crate::widgets;
use redditor::Client;
use redditor::types::{Listing, Post};

impl AppOp {
    pub fn load_subreddit_view(&self, subreddit_name: String) {        
        let mut client = Client::new();
        let posts = client.get_subreddit_posts(&subreddit_name);

        let subreddit_headerbar = self.create_subreddit_headerbar(subreddit_name.clone());
        let subreddit_view = self.create_subreddit_view(posts);

        self.update_headerbar(subreddit_headerbar);
        self.update_view(subreddit_view);
    }

    pub fn connect_subreddit_headerbar(&self) {
        // Back button
        let back_button: gtk::Button = self.ui.builder.get_object("subreddit_back_button").unwrap();        
        let backend = self.backend.clone();
        back_button.connect_clicked(move |_| {
            backend.clone().send(ViewChangeCommand::PreviousView()).unwrap();
        });

        // Go to subreddit entry
        let subreddit_goto_entry: gtk::Entry = self.ui.builder.get_object("subreddit_goto_entry").unwrap();        
        let backend = self.backend.clone();
        subreddit_goto_entry.connect_activate(move |subreddit_goto_entry| {
            let subreddit_name = subreddit_goto_entry.get_buffer().get_text();
            backend.clone().send(ViewChangeCommand::SubredditView(String::from(subreddit_name))).unwrap();
            subreddit_goto_entry.set_buffer(&gtk::EntryBuffer::new(None));
        });

        // Preferences popover
        let preferences_popover_button : gtk::Button = self.ui.builder.get_object("preferences_popover_button").unwrap();
        let preferences_popover_menu : gtk::PopoverMenu = self.ui.builder.get_object("preferences_popover_menu").unwrap();
        preferences_popover_button.connect_clicked(move |_| {
            preferences_popover_menu.show();
        });
    }

    fn create_subreddit_headerbar(&self, subreddit_name: String) -> gtk::Widget {
        let subreddit_headerbar: gtk::HeaderBar = self
            .ui
            .builder
            .get_object("subreddit_headerbar")
            .expect("Couldn't find subreddit_headerbar in ui file.");

        subreddit_headerbar.set_subtitle(Some(&subreddit_name));

        subreddit_headerbar.upcast::<gtk::Widget>()
    }

    fn create_subreddit_view(&self, posts: Listing<Post>) -> gtk::Widget {
        let scroll_window = gtk::ScrolledWindow::new(gtk::NONE_ADJUSTMENT, gtk::NONE_ADJUSTMENT);
        scroll_window.set_hexpand(false);
        scroll_window.set_vexpand(true);
        
        let posts_container = gtk::Box::new(gtk::Orientation::Vertical, 0);

        for post in posts {
            let post_widget = widgets::post::PostBox::new(&self, &post).widget(true, false);
            let separator = gtk::Separator::new(gtk::Orientation::Horizontal);
            posts_container.pack_start(&separator, false, true, 1);
            posts_container.pack_start(&post_widget, false, true, 0);
        }

        scroll_window.add(&posts_container);

        scroll_window.upcast::<gtk::Widget>()
    }
}