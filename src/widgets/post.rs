use redditor::types::{Post};
use gtk;
use gtk::prelude::*;

use crate::app::command::ViewChangeCommand;
use crate::appop::AppOp;

pub struct PostBox<'a> {
    op: &'a AppOp,
    post: &'a Post,
}

impl<'a> PostBox<'a> {
    pub fn new(op: &'a AppOp, post: &'a Post) -> PostBox<'a> {
        PostBox {
            op,
            post,
        }
    }
    
    pub fn widget(&self, show_comments_btn: bool, show_post_body: bool) -> gtk::Box {
        let post_container = gtk::Box::new(gtk::Orientation::Vertical, 0);
        let post_inner = gtk::Box::new(gtk::Orientation::Horizontal, 0);
        let post_info_box = gtk::Box::new(gtk::Orientation::Vertical, 0);

        let score = format!("{}", self.post.score());
        let title_info = format!("{}\n<small>r/{}, {} comments, by {}</small>", self.post.title(), self.post.subreddit(), self.post.num_comments(), self.post.author());
        let permalink_url = format!("https://www.reddit.com{}", self.post.permalink());
        let link_url = self.post.url();

        let score_label = gtk::Label::new(None);
        score_label.set_property_width_request(50);
        score_label.set_markup(&score);
        post_inner.pack_start(&score_label, false, true, 5);
        
        let title_label = gtk::Label::new(None);
        title_label.set_xalign(0.0);
        title_label.set_justify(gtk::Justification::Left);
        title_label.set_line_wrap(true);
        title_label.set_halign(gtk::Align::Start);
        title_label.set_markup(&title_info);
        post_info_box.pack_start(&title_label, false, true, 0);
        post_inner.pack_start(&post_info_box, true, true, 5);

        if show_comments_btn {
            let comments_btn = gtk::Button::new();
            let comments_icon: gtk::Image = gtk::Image::from_icon_name(Some("chat-icon"), gtk::IconSize::Button);
            comments_icon.set_size_request(32, 32);
            comments_btn.set_image(Some(&comments_icon));
            self.connect_comments_btn(&comments_btn);
            post_inner.pack_end(&comments_btn, false, true, 0);
        }

        if permalink_url != link_url { // if these URLs are equal, then this was a text post not a link post to somewhere on the web
            let link_btn = gtk::Button::new();
            let link_icon: gtk::Image = gtk::Image::from_icon_name(Some("globe-icon"), gtk::IconSize::Button);
            link_icon.set_size_request(32, 32);
            link_btn.set_image(Some(&link_icon));

            self.connect_link_btn(&link_btn);
            post_inner.pack_end(&link_btn, false, true, 5);
        }

        if show_post_body && !self.post.body().clone().is_empty() {
            let body_text = format!("{}", self.post.body());

            let body_label = gtk::Label::new(None);
            body_label.set_markup(&body_text);
            body_label.set_selectable(true);
            body_label.set_line_wrap(true);
            body_label.get_style_context().add_class("post-body");
            let separator = gtk::Separator::new(gtk::Orientation::Horizontal);

            post_container.pack_start(&separator, false, true, 1);
            post_container.pack_end(&body_label, false, true, 5);
        }

        post_container.pack_start(&post_inner, false, true, 0);

        post_container
    }

    fn connect_comments_btn(&self, comments_btn: &gtk::Button) {
        let url = self.post.permalink();
        let backend = self.op.backend.clone();
        comments_btn.connect_clicked(move |_| {
            backend.clone().send(ViewChangeCommand::CommentsView(url.clone())).unwrap();
        });
    }

    fn connect_link_btn(&self, link_btn: &gtk::Button) {
        let post_id = self.post.url();
        let backend = self.op.backend.clone();
        link_btn.connect_clicked(move |_| {
            backend.clone().send(ViewChangeCommand::WebView(post_id.clone())).unwrap();
        });
    }
}