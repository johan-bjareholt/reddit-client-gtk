#[derive(Clone,Debug)]
pub enum ViewChangeCommand {
    SubredditView(String),
    CommentsView(String),
    WebView(String),
    PreviousView(),
}
