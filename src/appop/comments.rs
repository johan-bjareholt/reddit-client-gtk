use gtk;
use gtk::prelude::*;

use crate::app::command::ViewChangeCommand;
use crate::appop::AppOp;
use crate::widgets;
use redditor::Client;
use redditor::types::{Comment, CommentList};

impl AppOp {
    pub fn load_comments_view(&self, post_id: String) {let mut client = Client::new();
        let comment_list = client.get_comments(&post_id).unwrap();

        let comments_headerbar = self.create_comments_headerbar();
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

    fn create_comments_headerbar(&self) -> gtk::Widget {
        let comments_headerbar: gtk::HeaderBar = self
            .ui
            .builder
            .get_object("comments_headerbar")
            .expect("Couldn't find comments_headerbar in ui file.");

        comments_headerbar.set_subtitle(Some(""));

        comments_headerbar.upcast::<gtk::Widget>()
    }

    fn create_comments_view(&self, comment_list: CommentList) -> gtk::Widget {
        let post = comment_list.post().clone();

        let scroll_window = gtk::ScrolledWindow::new(gtk::NONE_ADJUSTMENT, gtk::NONE_ADJUSTMENT);
        scroll_window.set_hexpand(false);
        scroll_window.set_vexpand(true);

        let comments_view_container : gtk::Box = gtk::Box::new(gtk::Orientation::Vertical, 0);
        let post_widget = widgets::post::PostBox::new(&self, post).widget(false, true);
        let separator = gtk::Separator::new(gtk::Orientation::Horizontal);
        comments_view_container.pack_start(&post_widget, false, true, 0);
        comments_view_container.pack_start(&separator, false, true, 0);

        let comments_container = gtk::Box::new(gtk::Orientation::Vertical, 0);
        for comment in comment_list.comments() {
            let comment_group_container = self.create_comment_group_loop(comment, 0);
            comments_container.pack_start(&comment_group_container, false, true, 0);
        }

        comments_view_container.pack_end(&comments_container, false, true, 0);
        scroll_window.add(&comments_view_container);

        scroll_window.upcast::<gtk::Widget>()
    }

    fn create_comment_group_loop(&self, comment: &Comment, depth: u8) -> gtk::Box {
        static PADDING: i32 = 5;
        
        let comment_group_container = gtk::Box::new(gtk::Orientation::Vertical, PADDING * 2);
    
        let comment_container = widgets::comment::CommentBox::new(&comment).widget();
        comment_group_container.pack_start(&comment_container, false, true, 0);
    
        let replies = comment.replies();
        if replies.len() > 0 {
            let reply_container_root = gtk::Box::new(gtk::Orientation::Horizontal, PADDING);
    
            let reply_container_sep_box = gtk::Box::new(gtk::Orientation::Horizontal, 0);
            let reply_container_sep_widget = reply_container_sep_box.upcast::<gtk::Widget>();
            let class_name = format!("comment-reply-{}", depth % 5);
            reply_container_sep_widget.get_style_context().add_class(&class_name);
            reply_container_root.pack_start(&reply_container_sep_widget, false, true, 0);
    
            let reply_container: gtk::Box = gtk::Box::new(gtk::Orientation::Vertical, PADDING);
            reply_container_root.pack_start(&reply_container, false, true, 0);
            for reply in comment.replies() {
                let reply_container_v = self.create_comment_group_loop(reply, depth + 1);
                reply_container.pack_start(&reply_container_v, false, true, 0);
            }
            comment_group_container.pack_start(&reply_container_root, false, true, 0);
        }
    
        comment_group_container
    }
}