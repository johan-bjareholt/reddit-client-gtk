extern crate glib;
extern crate gtk;
extern crate webkit2gtk;
extern crate redditor;

use gtk::prelude::*;
use webkit2gtk::WebViewExt;

use std::thread;
use std::collections::LinkedList;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::mpsc::{channel, Sender, Receiver};

use redditor::Client;
use redditor::types::{Listing, Post, CommentList, Comment};

#[derive(Clone,Debug)]
pub enum ViewChangeCommand {
    SubredditView(String),
    CommentsView(String),
    WebView(String),
    PreviousView(),
}

pub struct State {
    builder: gtk::Builder,
    state_tx: Sender<ViewChangeCommand>
}

static mut STATE : Option<Arc<Mutex<State>>> = None;

pub fn get_state() -> Arc<Mutex<State>> {
    unsafe {
        match STATE {
            Some(ref s) => s.clone(),
            None => panic!()
        }
    }
}

fn create_comments_container_loop(comment: &Comment) -> gtk::Box {
    static PADDING : i32 = 5;
    let root_container: gtk::Box = gtk::Box::new(gtk::Orientation::Vertical, PADDING*2);

    let comment_container: gtk::Box = gtk::Box::new(gtk::Orientation::Horizontal, PADDING);
    let rlabel = gtk::Label::new(None);
    rlabel.set_line_wrap(true);
    let label_str = format!("<small>{} - u/{}</small>\n{}", comment.score(), comment.author(), comment.body());
    rlabel.set_markup(&label_str);
    comment_container.pack_start(&rlabel, false, false, 0);

    let reply_container_root: gtk::Box = gtk::Box::new(gtk::Orientation::Horizontal, PADDING);
    let reply_container_separator = gtk::Separator::new(gtk::Orientation::Horizontal);
    reply_container_root.pack_start(&reply_container_separator, false, false, 0);
    let reply_container: gtk::Box = gtk::Box::new(gtk::Orientation::Vertical, PADDING);
    reply_container_root.pack_start(&reply_container, false, false, 0);
    for reply in comment.replies() {
        let reply_container_v = create_comments_container_loop(reply);
        reply_container.pack_start(&reply_container_v, false, false, 0);
    }

    root_container.pack_start(&comment_container, false, false, 0);
    root_container.pack_start(&reply_container_root, false, false, 0);
    return root_container
}

fn create_comments_container(commentlist: CommentList) -> gtk::Box {
    let post = commentlist.post();
    let container : gtk::Box = gtk::Box::new(gtk::Orientation::Vertical, 0);
    let post_container = create_link_widget(&post, false, true);
    container.pack_start(&post_container, false, false, 0);

    let comments_container = gtk::Box::new(gtk::Orientation::Vertical, 0);
    for comment in commentlist.comments() {
        let comment_container = create_comments_container_loop(comment);
        comments_container.pack_start(&comment_container, false, false, 0);
    }
    container.pack_end(&comments_container, false, false, 0);

    return container
}

fn create_link_widget(post: &Post, show_comments_btn: bool, show_body: bool) -> gtk::Box {
    let entry = gtk::Box::new(gtk::Orientation::Vertical, 0);
    let entry_info = gtk::Box::new(gtk::Orientation::Horizontal, 0);

    let label = gtk::Label::new(None);

    let label_str = format!("{} - {}\n<small>r/{}, {} comments</small>", post.score(), post.title(), post.subreddit(), post.num_comments());
    label.set_markup(&label_str);
    label.set_line_wrap(true);
    entry_info.pack_start(&label, false, false, 0);

    let permalink = post.permalink();
    let linkurl = post.url();

    let permalink_comment = permalink.clone();
    if show_comments_btn {
        let commentsbtn = gtk::Button::new_with_label("Comments");
        commentsbtn.connect_clicked(move |_b| {
            let sg = get_state();
            let s = sg.lock().unwrap();
            s.state_tx.send(ViewChangeCommand::CommentsView(String::from(permalink_comment.clone()))).unwrap();
        });
        entry_info.pack_end(&commentsbtn, false, false, 0);
    }

    // If permalink_url and linkurl are the same it is a selfpost, no linkbtn is needed
    let permalink_url = format!("https://www.reddit.com{}", permalink);
    if permalink_url != linkurl {
        let linkbtn = gtk::Button::new_with_label("Link");
        linkbtn.connect_clicked(move |_b| {
            let sg = get_state();
            let s = sg.lock().unwrap();
            s.state_tx.send(ViewChangeCommand::WebView(linkurl.clone())).unwrap();
        });
        entry_info.pack_end(&linkbtn, false, false, 5);
    }

    entry.pack_start(&entry_info, false, false, 0);

    if show_body {
        let body_label = gtk::Label::new(None);
        let label_str = format!("{}", post.body());
        body_label.set_markup(&label_str);
        body_label.set_line_wrap(true);
        entry.pack_start(&body_label, false, false, 0);
    }

    return entry;
}

fn create_link_container(posts: Listing<Post>) -> gtk::Box {
    let container : gtk::Box = gtk::Box::new(gtk::Orientation::Vertical, 0);

    for post in posts {
        let entry = create_link_widget(&post, true, false);

        let separator = gtk::Separator::new(gtk::Orientation::Horizontal);

        container.pack_start(&separator, false, false, 1);
        container.pack_start(&entry, false, false, 0);
    }

    return container;
}

fn replace_view_with(builder: &gtk::Builder, view: &gtk::Widget) {
    let root_container: gtk::Container = builder.get_object("ContentViewport").unwrap();
    for child in root_container.get_children() {
        root_container.remove(&child);
    }
    root_container.add(view);
    view.show_all();
}

fn set_loadingspinner(status: bool) -> () {
    let ctx = glib::MainContext::default();
    ctx.invoke(move || {
        let sg = get_state();
        let s = sg.lock().unwrap();

        let spinner : gtk::Spinner = s.builder.get_object("LoadingSpinner").unwrap();

        if status {
            spinner.start();
        } else {
            spinner.stop();
        }
    });
}

fn statechange_loop (rx: Receiver<ViewChangeCommand>, tx: Sender<ViewChangeCommand>) {
    thread::spawn(move || {
        let mut client = Client::new();
        let ctx = glib::MainContext::default();
        let mut prev_view_stack : LinkedList<ViewChangeCommand> = LinkedList::new();
        loop {
            set_loadingspinner(false);
            let new_view = match rx.recv() {
                Ok(c) => c,
                Err(_e) => continue
            };
            set_loadingspinner(true);
            match new_view.clone() {
                ViewChangeCommand::SubredditView(subreddit_name) => {
                    println!("Switching to subreddit view {}", subreddit_name);
                    let posts = client.get_subreddit_posts(&subreddit_name);

                    ctx.invoke(move || {
                        let sg = get_state();
                        let s = sg.lock().unwrap();
                        let frontpage_view = create_link_container(posts);
                        replace_view_with(&s.builder, &frontpage_view.upcast::<gtk::Widget>());
                    });
                },
                ViewChangeCommand::CommentsView(post_id) => {
                    println!("Switching to comments view with id: {}", post_id);
                    let commentlist = client.get_comments(&post_id).unwrap();
                    ctx.invoke(move || {
                        let sg = get_state();
                        let s = sg.lock().unwrap();
                        let comments_view = create_comments_container(commentlist);
                        replace_view_with(&s.builder, &comments_view.upcast::<gtk::Widget>());
                    });
                },
                ViewChangeCommand::WebView(url) => {
                    ctx.invoke(move || {
                        let sg = get_state();
                        let s = sg.lock().unwrap();

                        let webview = webkit2gtk::WebView::new();
                        webview.load_uri(&url);
                        replace_view_with(&s.builder, &webview.upcast::<gtk::Widget>());
                    });
                }
                ViewChangeCommand::PreviousView() => {
                    if prev_view_stack.len() <= 1 {
                        continue
                    }
                    let _current_view = prev_view_stack.pop_front();
                    let prev_view = prev_view_stack.pop_front();
                    match prev_view {
                        None => (),
                        Some(prev_view) => {
                            println!("Going back to previous view: {:?}", prev_view);
                            tx.send(prev_view).unwrap();
                        }
                    }
                }
            }
            match new_view {
                ViewChangeCommand::PreviousView() => (),
                new_view => prev_view_stack.push_front(new_view)
            }
        }
    });
}

fn main() {
	// Init GTK
    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }

    let (tx, rx) = channel::<ViewChangeCommand>();
    // Load layout and builder
    let glade_src = include_str!("../resources/layout.glade");
    let builder = gtk::Builder::new_from_string(glade_src);

    // Create Window
	let window: gtk::Window = builder.get_object("RedditClientWindow").unwrap();
    window.connect_destroy(|_| {
        gtk::main_quit();
    });
    window.show_all();

    // Setup popover
    let button : gtk::Button = builder.get_object("PreferencesPopoverButton").unwrap();
    let popover : gtk::PopoverMenu = builder.get_object("PreferencesPopoverMenu").unwrap();
    button.connect_clicked(move |_| {
        println!("Showing popover");
        popover.popup();
    });

    let back_button : gtk::Button = builder.get_object("BackButton").unwrap();
    let backbutton_tx = tx.clone();
    back_button.connect_clicked(move |_| {
        println!("Pressed back button");
        match backbutton_tx.send(ViewChangeCommand::PreviousView()) {
            Ok(_) => (),
            Err(_) => ()
        }
    });

    // Setup subreddit selection
    let subreddit_entry : gtk::Entry = builder.get_object("SubredditTextEntry").unwrap();
    let tx2 = tx.clone();
    subreddit_entry.connect_activate(move |entry| {
        let subreddit_name = entry.get_buffer().get_text();
        tx2.send(ViewChangeCommand::SubredditView(String::from(subreddit_name))).unwrap();
        entry.set_buffer(&gtk::EntryBuffer::new(None));
    });

    unsafe {
        STATE = Some(Arc::new(Mutex::new(State {
            builder: builder,
            state_tx: tx.clone()
        })));
    }

    statechange_loop(rx, tx.clone());
    tx.send(ViewChangeCommand::SubredditView(String::from("all"))).unwrap();
    // Load frontpage by default

    gtk::main();
}
