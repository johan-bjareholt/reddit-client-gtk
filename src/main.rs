extern crate glib;
extern crate gtk;
extern crate rawr;

use gtk::prelude::*;
use rawr::prelude::*;

use std::thread;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::mpsc::{channel, Sender, Receiver};

use rawr::client::RedditClient;
use rawr::auth::AnonymousAuthenticator;
use rawr::options::ListingOptions;
use rawr::structures::submission::Submission;
use rawr::structures::subreddit::Subreddit;

pub enum ViewChangeCommand {
    SubredditView(String),
    CommentsView(String)
}

pub struct State {
    builder: gtk::Builder,
    client: RedditClient,
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

fn create_comments_container(post: Submission) -> gtk::Box {
    let container : gtk::Box = gtk::Box::new(gtk::Orientation::Vertical, 0);
    let plabel = gtk::Label::new(None);
    let plabel_str = format!("{} - {}\n<small>r/{}, {} comments</small>", post.score(), post.title(), post.subreddit().name, post.reply_count());
    plabel.set_markup(&plabel_str);
    plabel.set_line_wrap(true);
    container.pack_start(&plabel, false, false, 0);

    let replies = post.replies().expect("Could not get comments");
    let comments_container = gtk::Box::new(gtk::Orientation::Vertical, 0);
    for comment in replies.take(10) {
        let comment_container: gtk::Box = gtk::Box::new(gtk::Orientation::Horizontal, 0);
        let rlabel = gtk::Label::new(None);
        rlabel.set_line_wrap(true);
        let author = "u/username"; // TODO: Fix username
        let label_str = format!("<small>{} - {}</small>\n{}", comment.score(), author, comment.body().unwrap());
        rlabel.set_markup(&label_str);

        comment_container.pack_start(&rlabel, false, false, 0);
        comments_container.pack_start(&comment_container, false, false, 0);
    }
    let separator = gtk::Separator::new(gtk::Orientation::Horizontal);
    container.pack_end(&separator, false, false, 0);
    container.pack_end(&comments_container, false, false, 0);

    container.show_all();
    return container
}

fn create_link_container(subreddit: Subreddit) -> gtk::Box {
    let container : gtk::Box = gtk::Box::new(gtk::Orientation::Vertical, 0);

    let listing = subreddit.hot(ListingOptions::default()).expect("Could not fetch posts");

    for post in listing.take(5) {
        let entry = gtk::Box::new(gtk::Orientation::Horizontal, 0);

        let label = gtk::Label::new(None);

        let label_str = format!("{} - {}\n<small>r/{}, {} comments</small>", post.score(), post.title(), post.subreddit().name, post.reply_count());
        label.set_markup(&label_str);
        label.set_line_wrap(true);
        entry.pack_start(&label, false, false, 0);

        let linkbtn = gtk::Button::new_with_label("Link");
        entry.pack_end(&linkbtn, false, false, 5);
        let submission_id = post.name().to_string();

        let commentsbtn = gtk::Button::new_with_label("Comments");
        commentsbtn.connect_clicked(move |_b| {
            let sg = get_state();
            let s = sg.lock().unwrap();
            s.state_tx.send(ViewChangeCommand::CommentsView(String::from(submission_id.clone()))).unwrap();
        });
        entry.pack_end(&commentsbtn, false, false, 0);

        let separator = gtk::Separator::new(gtk::Orientation::Horizontal);

        container.pack_start(&separator, false, false, 1);
        container.pack_start(&entry, false, false, 0);
    }

    container.show_all();
    return container;
}

fn replace_view_with(builder: &gtk::Builder, view: &gtk::Box) {
    let root_container: gtk::Container = builder.get_object("ContentViewport").unwrap();
    for child in root_container.get_children() {
        root_container.remove(&child);
    }
    root_container.add(view);
    view.show();
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

fn statechange_loop (rx: Receiver<ViewChangeCommand>) {
    thread::spawn(move || {
        let ctx = glib::MainContext::default();
        loop {
            let new_view = match rx.recv() {
                Ok(c) => c,
                Err(_e) => continue
            };
            set_loadingspinner(true);
            match new_view {
                ViewChangeCommand::SubredditView(subreddit_name) => {
                    println!("Switching to subreddit view {}", subreddit_name);
                    ctx.invoke(move || {
                        let sg = get_state();
                        let s = sg.lock().unwrap();

                        let frontpage = s.client.subreddit(&subreddit_name);
                        let frontpage_view = create_link_container(frontpage);
                        replace_view_with(&s.builder, &frontpage_view);
                    });
                },
                ViewChangeCommand::CommentsView(post_id) => {
                    println!("Switching to comments view with id: {}", post_id);
                    ctx.invoke(move || {
                        let sg = get_state();
                        let s = sg.lock().unwrap();

                        let post = s.client.get_by_id(&post_id).get().unwrap();
                        let comments_view = create_comments_container(post);
                        replace_view_with(&s.builder, &comments_view);
                    });
                }
            }
            set_loadingspinner(false);
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

    // Setup popover
    let button: gtk::Button = builder.get_object("PreferencesPopoverButton").unwrap();
    let popover : gtk::PopoverMenu = builder.get_object("PreferencesPopoverMenu").unwrap();
    button.connect_clicked(move |_| {
        println!("Showing popover");
        popover.popup();
    });

    // Setup subreddit selection
    let subreddit_entry : gtk::Entry = builder.get_object("SubredditTextEntry").unwrap();
    let tx2 = tx.clone();
    subreddit_entry.connect_activate(move |entry| {
        let subreddit_name = entry.get_buffer().get_text();
        tx2.send(ViewChangeCommand::SubredditView(String::from(subreddit_name))).unwrap();
        entry.set_buffer(&gtk::EntryBuffer::new(None));
    });

    window.show_all();

    let client = RedditClient::new("linux:reddit-client-gtk-rs:0.0.0", AnonymousAuthenticator::new());

    unsafe {
        STATE = Some(Arc::new(Mutex::new(State {
            builder: builder,
            client: client,
            state_tx: tx.clone()
        })));
    }

    statechange_loop(rx);
    tx.send(ViewChangeCommand::SubredditView(String::from("all"))).unwrap();
    // Load frontpage by default

    gtk::main();
}
