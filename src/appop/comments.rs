use gtk;
use gtk::prelude::*;

use crate::app::command::ViewChangeCommand;
use crate::appop::AppOp;
use crate::widgets;
use widgets::comment_list::CommentListBox;
use redditor::Client;
use redditor::types::{CommentList};

impl AppOp {
    pub fn load_comments_view(&self, post_id: String, subreddit_name: String) {let mut client = Client::new();
        let comment_list = client.get_comments(&post_id).unwrap();

        let comments_headerbar = self.create_comments_headerbar(&subreddit_name);
        let comments_view = self.create_comments_view(comment_list);

        self.update_headerbar(comments_headerbar);
        self.update_view(comments_view);

        //comments_view
    }

    pub fn connect_comments_headerbar(&self) {
        // Back button
        let back_button: gtk::Button = self.ui.builder.get_object("comments_back_button").unwrap();        
        let backend = self.backend.clone();
        back_button.connect_clicked(move |_| {
            backend.clone().send(ViewChangeCommand::PreviousView()).unwrap();
        });
    }

    fn create_comments_headerbar(&self, subreddit_name: &String) -> gtk::Widget {
        let comments_headerbar: gtk::HeaderBar = self
            .ui
            .builder
            .get_object("comments_headerbar")
            .expect("Couldn't find comments_headerbar in ui file.");

        comments_headerbar.set_subtitle(Some(subreddit_name));
        comments_headerbar.show_all();

        comments_headerbar.upcast::<gtk::Widget>()
    }

    fn create_comments_view(&self, comment_list: CommentList) -> gtk::Widget {
        let scroll_window = gtk::ScrolledWindow::new(gtk::NONE_ADJUSTMENT, gtk::NONE_ADJUSTMENT);
        scroll_window.set_hexpand(false);
        scroll_window.set_vexpand(true);
        scroll_window.show();

        let comments_view_container : gtk::Box = gtk::Box::new(gtk::Orientation::Vertical, 0);
        comments_view_container.show();
        let post_widget = widgets::post::PostBox::new(&self, comment_list.post()).widget(false, true);
        post_widget.show_all();
        let separator = gtk::Separator::new(gtk::Orientation::Horizontal);
        separator.show();

        comments_view_container.pack_start(&post_widget, false, true, 0);
        comments_view_container.pack_start(&separator, true, true, 0);

        let comments_listbox = CommentListBox::new(&self, &comment_list).widget();
        comments_listbox.show();

        comments_view_container.pack_end(&comments_listbox, false, true, 0);
        scroll_window.add(&comments_view_container);

        scroll_window.upcast::<gtk::Widget>()
    }
}
