use gtk;
use gtk::prelude::*;

use crate::appop::AppOp;
use redditor::types::{Comment};

static PADDING: i32 = 5;

pub struct CommentBox<'a> {
    op: &'a AppOp,
    comment: &'a Comment,
    depth: &'a u8,
}

impl<'a> CommentBox<'a> {
    pub fn new(op: &'a AppOp, comment: &'a Comment, depth: &'a u8) -> CommentBox<'a> {
        CommentBox {
            op,
            comment,
            depth,
        }
    }

    pub fn widget(&self) -> gtk::ListBoxRow {
        let comment_row = gtk::ListBoxRow::new();
        comment_row.set_activatable(true);
        comment_row.show();
        comment_row.get_style_context().add_class(&format!("comment--depth-{}", self.depth)); // Part of bad hack in comment_list

        let comment_container = gtk::Box::new(gtk::Orientation::Vertical, PADDING);
        comment_container.show();
        
        let comment_container_inner = gtk::Box::new(gtk::Orientation::Vertical, 0);
        let depth = self.depth % 5;
        let class_name = format!("comment-depth-{}", depth);
        comment_container_inner.get_style_context().add_class(&class_name);
        comment_container_inner.get_style_context().add_class("comment-container-inner");
        comment_container_inner.set_margin_start((self.depth * 5) as i32);

        let comment_header_container = self.create_comment_header_container();
        let comment_body_container = self.create_comment_body_container();

        comment_container_inner.pack_start(&comment_header_container, false, true, 0);
        comment_container_inner.pack_start(&comment_body_container, false, true, 0);  
        comment_container_inner.show_all();

        let collapsed_notice_container = self.create_collapsed_notice_container();
        collapsed_notice_container.hide();

        comment_container.pack_start(&comment_container_inner, true, true, 0);
        comment_container.pack_start(&collapsed_notice_container, false, true, 0); 
        comment_row.add(&comment_container);

        comment_row
    }

    fn create_comment_header_container(&self) -> gtk::Box {
        let comment_header_container = gtk::Box::new(gtk::Orientation::Horizontal, 0);

        let label_str = format!("<small>{} - u/{}</small>", self.comment.score(), self.comment.author());

        let label = gtk::Label::new(None);
        label.set_markup(&label_str);
        label.set_selectable(true);

        comment_header_container.pack_start(&label, false, true, 0);

        comment_header_container
    }

    fn create_comment_body_container(&self) -> gtk::Box {
        let comment_body_container = gtk::Box::new(gtk::Orientation::Horizontal, 0);

        let label = gtk::Label::new(None);
        label.set_markup(&self.comment.body());
        label.set_line_wrap(true);
        label.set_line_wrap_mode(pango::WrapMode::WordChar);
        label.set_selectable(true);

        comment_body_container.pack_start(&label, false, false, 0);

        comment_body_container
    }

    fn create_collapsed_notice_container(&self) -> gtk::Box {
        let collapsed_notice_container = gtk::Box::new(gtk::Orientation::Horizontal, 0);
        collapsed_notice_container.get_style_context().add_class("collapsed-notice-container");

        let label = gtk::Label::new(None);
        label.set_markup("<span style=\"italic\"><small>Comments hidden - click to reveal</small></span>");
        label.set_justify(gtk::Justification::Center);
        label.set_hexpand(true);
        label.get_style_context().add_class("collapsed-comment-label");
        label.set_selectable(false);
        label.show();

        collapsed_notice_container.pack_start(&label, false, false, 0);

        collapsed_notice_container
    }
}
