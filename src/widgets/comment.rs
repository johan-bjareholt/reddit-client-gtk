use gtk;
use gtk::prelude::*;
use pango;

use redditor::types::{Comment};

static PADDING: i32 = 5;

pub struct CommentBox<'a> {
    comment: &'a Comment,
    depth: &'a u8,
}

impl<'a> CommentBox<'a> {
    pub fn new(comment: &'a Comment, depth: &'a u8) -> CommentBox<'a> {
        CommentBox {
            comment,
            depth,
        }
    }
    
    pub fn widget(&self) -> gtk::Box {
        let comment_container = gtk::Box::new(gtk::Orientation::Horizontal, PADDING);
        
        let comment_container_inner = gtk::Box::new(gtk::Orientation::Vertical, PADDING);
        let depth = self.depth % 5;
        let class_name = format!("comment-depth-{}", depth);
        comment_container_inner.get_style_context().add_class(&class_name);
        comment_container_inner.get_style_context().add_class("comment-container-inner");
        comment_container_inner.set_margin_start((depth * 5) as i32);

        let comment_header_container = self.create_comment_header_container();
        let comment_body_container = self.create_comment_body_container();
        comment_container_inner.pack_start(&comment_header_container, false, true, 0);
        comment_container_inner.pack_start(&comment_body_container, false, true, 0);

        comment_container.pack_start(&comment_container_inner, true, true, 0);
        
        comment_container
    }

    fn create_comment_header_container(&self) -> gtk::Box {
        let comment_header_container = gtk::Box::new(gtk::Orientation::Horizontal, 0);

        let comment_header_label = gtk::Label::new(None);
        let label_str = format!("<small>{} - u/{}</small>", self.comment.score(), self.comment.author());
        comment_header_label.set_markup(&label_str);
        comment_header_label.set_selectable(true);

        comment_header_container.pack_start(&comment_header_label, false, true, 0);

        comment_header_container
    }

    fn create_comment_body_container(&self) -> gtk::Box {
        let comment_body_container = gtk::Box::new(gtk::Orientation::Horizontal, 0);

        let comment_body_label = gtk::Label::new(None);
        comment_body_label.set_markup(&self.comment.body());
        comment_body_label.set_line_wrap(true);
        comment_body_label.set_line_wrap_mode(pango::WrapMode::WordChar);
        comment_body_label.set_selectable(true);

        comment_body_container.pack_start(&comment_body_label, false, false, 0);

        comment_body_container
    }
}