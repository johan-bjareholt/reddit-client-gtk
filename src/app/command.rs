#[derive(Clone,Debug)]
pub enum ViewChangeCommand {
    SubredditView(String),
    CommentsView(String, String),
    WebView(String),
    PreviousView(),
}
