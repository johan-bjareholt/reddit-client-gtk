use gtk;
use gtk::prelude::*;

use crate::app::command::ViewChangeCommand;
use crate::appop::AppOp;
use crate::widgets;
use redditor::Client;
use redditor::types::{Comment, CommentList};

impl AppOp {
    pub fn load_comments_view(&self, post_id: String, subreddit_name: String) {let mut client = Client::new();
        let comment_list = client.get_comments(&post_id).unwrap();

        let comments_headerbar = self.create_comments_headerbar(&subreddit_name);
        let comments_view = self.create_comments_view(comment_list);

        self.update_headerbar(comments_headerbar);
        self.update_view(comments_view);
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

        comments_headerbar.upcast::<gtk::Widget>()
    }

    fn create_comments_view(&self, comment_list: CommentList) -> gtk::Widget {
        let scroll_window = gtk::ScrolledWindow::new(gtk::NONE_ADJUSTMENT, gtk::NONE_ADJUSTMENT);
        scroll_window.set_hexpand(false);
        scroll_window.set_vexpand(true);

        let comments_view_container : gtk::Box = gtk::Box::new(gtk::Orientation::Vertical, 0);
        let post_widget = widgets::post::PostBox::new(&self, comment_list.post()).widget(false, true);
        let separator = gtk::Separator::new(gtk::Orientation::Horizontal);
        comments_view_container.pack_start(&post_widget, false, true, 0);
        comments_view_container.pack_start(&separator, true, true, 0);

        let comments_container = self.create_comments_container(comment_list);

        comments_view_container.pack_end(&comments_container, false, true, 0);
        scroll_window.add(&comments_view_container);

        scroll_window.upcast::<gtk::Widget>()
    }

    fn create_comments_container(&self, comment_list: CommentList) -> gtk::Box {
        let comments_container = gtk::Box::new(gtk::Orientation::Vertical, 0);

        for comment_root in comment_list.comments() {
            let comment_group_container = self.create_comment_group_loop(comment_root, 0);
            comments_container.pack_start(&comment_group_container, false, true, 0);
        }

        comments_container
    }

    fn create_comment_group_loop(&self, comment: &Comment, depth: u8) -> gtk::Box {
        let comment_group_container = gtk::Box::new(gtk::Orientation::Vertical, 0);

        let comment_widget = widgets::comment::CommentBox::new(&comment, &depth).widget();
        comment_group_container.pack_start(&comment_widget, false, true, 0);

        let replies = comment.replies();
        if replies.len() > 0 {
            for reply in comment.replies() {
                let reply_container = self.create_comment_group_loop(&reply, depth + 1);
                comment_group_container.pack_start(&reply_container, false, true, 0);
            }
        }

        comment_group_container
    }
}