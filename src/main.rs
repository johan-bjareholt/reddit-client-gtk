extern crate gtk;
extern crate rawr;

use gtk::prelude::*;
use rawr::prelude::*;

use rawr::client::RedditClient;
use rawr::auth::AnonymousAuthenticator;
use rawr::options::ListingOptions;
use rawr::structures::submission::Submission;
use rawr::structures::subreddit::Subreddit;

fn create_comments_container(post: String) -> gtk::Box {
//fn create_comments_container(post: Submission) -> gtk::Box {
    let container : gtk::Box = gtk::Box::new(gtk::Orientation::Vertical, 0);
    /*
    let replies = post.replies().expect("Could not get comments");
    for comment in replies.take(100) {
        println!("{}: {:?}", comment.author().name, comment.body());
    }
    */
    return container
}

fn create_link_container(subreddit: Subreddit) -> gtk::Box {
    let container : gtk::Box = gtk::Box::new(gtk::Orientation::Vertical, 0);

    let listing = subreddit.hot(ListingOptions::default()).expect("Could not fetch posts");

    for post in listing.take(25) {
        let entry = gtk::Box::new(gtk::Orientation::Horizontal, 0);

        let label = gtk::Label::new(None);

        let label_str = format!("{} - {}\n<small>r/{}, {} comments</small>", post.score(), post.title(), post.subreddit().name, post.reply_count());
        label.set_markup(&label_str);
        label.set_line_wrap(true);
        entry.pack_start(&label, false, false, 0);

        let linkbtn = gtk::Button::new_with_label("Link");
        entry.pack_end(&linkbtn, false, false, 5);
        let linkurl = post.link_url().unwrap();

        let commentsbtn = gtk::Button::new_with_label("Comments");
        commentsbtn.connect_clicked(move |b| {
            println!("Pressed commentsbtn");
            //let comments_view = create_comments_container(teststr.clone());
            let test = linkurl.clone();
        });
        entry.pack_end(&commentsbtn, false, false, 0);

        label.show();
        commentsbtn.show();
        linkbtn.show();

        let separator = gtk::Separator::new(gtk::Orientation::Horizontal);
        separator.show();

        container.pack_start(&separator, false, false, 1);
        container.pack_start(&entry, false, false, 0);

        entry.show();
    }

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

fn main() {
	// Init GTK
    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }

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
    let button: gtk::Button = builder.get_object("PreferencesPopoverButton").unwrap();
    let popover : gtk::PopoverMenu = builder.get_object("PreferencesPopoverMenu").unwrap();
	button.connect_clicked(move |_| {
		println!("Showing popover");
		popover.popup();
	});


    // Load frontpage by default
    let client = RedditClient::new("linux:reddit-client-gtk-rs:0.0.0", AnonymousAuthenticator::new());
    let frontpage = client.subreddit("all");
    let frontpage_view = create_link_container(frontpage);
    replace_view_with(&builder, &frontpage_view);

    // Setup subreddit selection
    let subreddit_entry : gtk::Entry = builder.get_object("SubredditTextEntry").unwrap();
    subreddit_entry.connect_activate(move |entry| {
        let subreddit_name = entry.get_buffer().get_text();
        println!("Swtiching to subreddit '{}'", subreddit_name);
        let new_subreddit = client.subreddit(&subreddit_name);
        let view = create_link_container(new_subreddit);
        replace_view_with(&builder, &view);
        entry.set_buffer(&gtk::EntryBuffer::new(None));
    });

    gtk::main();
}
