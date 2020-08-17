use crate::app::App;

impl App {
    pub fn connect_ui(&self) {
        self.op.lock().unwrap().connect_comments_headerbar();
        self.op.lock().unwrap().connect_subreddit_headerbar();
        self.op.lock().unwrap().connect_webview_headerbar();
    }
}