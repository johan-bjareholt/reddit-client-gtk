extern crate glib;
extern crate gtk;
extern crate gdk;
extern crate webkit2gtk;
extern crate redditor;

mod app;
pub mod appop;
mod globals;
mod static_resources;
mod ui_builder;
mod widgets;

use crate::app::App;
use gio::ApplicationExt;
use gio::prelude::ApplicationExtManual;
use std::env::args;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    static_resources::init().expect("GResource initialization failed.");

    // let app_id = globals::APP_ID.unwrap_or("RedditClient");
    let app_id = globals::APP_ID;
    let application = gtk::Application::new(app_id, gio::ApplicationFlags::empty())?;

    application.set_resource_base_path(Some("/org/johan/RedditClient"));

    application.connect_startup(|application| {
        App::on_startup(application);
    });

    application.run(&args().collect::<Vec<_>>());

    Ok(())
}
