use gtk;
use gtk::prelude::*;

use redditor::types::{Comment};

pub struct CommentBox<'a> {
    comment: &'a Comment,
}

impl<'a> CommentBox<'a> {
    pub fn new(comment: &'a Comment) -> CommentBox<'a> {
        CommentBox {
            comment,
        }
    }
    
    pub fn widget(&self) -> gtk::Box {
        static PADDING: i32 = 5;

        let comment_container = gtk::Box::new(gtk::Orientation::Horizontal, PADDING);
        let comment_container_inner = gtk::Box::new(gtk::Orientation::Vertical, PADDING);

        comment_container.get_style_context().add_class("comment-container");
    
        let header_label = gtk::Label::new(None);
        let header_label_box = gtk::Box::new(gtk::Orientation::Horizontal, 0);
        let label_str = format!("<small>{} - u/{}</small>", self.comment.score(), self.comment.author());
        header_label.set_markup(&label_str);
    
        let body_label = gtk::Label::new(None);
        let body_label_box = gtk::Box::new(gtk::Orientation::Horizontal, PADDING);
        body_label.set_selectable(true);
        body_label.set_line_wrap(true);
        body_label.set_markup(&self.comment.body());
    
        header_label_box.pack_start(&header_label, false, true, 0);
        body_label_box.pack_start(&body_label, false, true, 0);
        comment_container_inner.pack_start(&header_label_box, false, true, 0);
        comment_container_inner.pack_start(&body_label_box, false, true, 0);
        comment_container.pack_start(&comment_container_inner, true, true, 0);

        comment_container
    }
}