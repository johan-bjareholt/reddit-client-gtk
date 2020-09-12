//use crate::widgets::SVEntry;
use gtk::{self, prelude::*};

#[derive(Clone, Debug)]
pub struct UI {
    pub builder: gtk::Builder,
    //pub sventry: SVEntry,
}

impl UI {
    pub fn new() -> UI {
        let builder = gtk::Builder::new();

        builder
            .add_from_resource("/org/johan/RedditClient/ui/main_window.ui")
            .expect("Can't load ui file: main_window.ui");

        builder
            .add_from_resource("/org/johan/RedditClient/ui/comments.ui")
            .expect("Can't load ui file: comments.ui");

        builder
            .add_from_resource("/org/johan/RedditClient/ui/subreddit.ui")
            .expect("Can't load ui file: subreddit.ui");

        builder
            .add_from_resource("/org/johan/RedditClient/ui/webview.ui")
            .expect("Can't load ui file: webview.ui");

        UI { builder }
    }
}
