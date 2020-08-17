use crate::app::App;
use crate::app::command::ViewChangeCommand;

use std::collections::LinkedList;
// use std::ops;
// use std::rc::{Rc, Weak};
use std::sync::mpsc::{Receiver, Sender};
use std::thread;

#[macro_export]
macro_rules! APPOP {
    ($fn: ident, ($($x:ident),*) ) => {{
        let ctx = glib::MainContext::default();
        ctx.invoke(move || {
            $( let $x = $x.clone(); )*
            if let Some(op) = App::get_op() {
                op.lock().unwrap().$fn($($x),*);
            }
        });
    }};
    ($fn: ident) => {{
        APPOP!($fn, ( ) );
    }}
}

pub fn backend_loop (tx: Sender<ViewChangeCommand>, rx: Receiver<ViewChangeCommand>) {
    thread::spawn(move || {
        let mut prev_view_stack : LinkedList<ViewChangeCommand> = LinkedList::new();
        // let mut shutting_down = false;
        loop {
            let view_change_command = match rx.recv() {
                Ok(c) => c,
                Err(_e) => break, // stopping this backend loop thread
            };
            // if shutting_down { // ignore this event, we're shutting down this thread
            //     continue;
            // }
            
            match view_change_command.clone() {
                ViewChangeCommand::SubredditView(subreddit_name) => {
                    println!("Switching to subreddit view {}", subreddit_name);
                    APPOP!(load_subreddit_view, (subreddit_name));
                },
                ViewChangeCommand::CommentsView(post_id) => {
                    println!("Switching to comments view with id: {}", post_id);
                    APPOP!(load_comments_view, (post_id));
                },
                ViewChangeCommand::WebView(url) => {
                    println!("Switching to web view with url: {}", url);
                    APPOP!(load_webview_view, (url));
                }
                ViewChangeCommand::PreviousView() => {
                    if prev_view_stack.len() <= 1 {
                        continue
                    }
                    let _current_view = prev_view_stack.pop_front();
                    let prev_view = prev_view_stack.pop_front();
                    match prev_view {
                        Some(prev_view) => {
                            println!("Going back to previous view: {:?}", prev_view);
                            tx.send(prev_view).unwrap();
                        }
                        None => (),
                    }
                }
            }
            match view_change_command {
                ViewChangeCommand::PreviousView() => (),
                new_view => prev_view_stack.push_front(new_view)
            }
        }
    });
}