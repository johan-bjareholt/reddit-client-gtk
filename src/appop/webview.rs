use gtk;
use gtk::prelude::*;
use webkit2gtk::WebViewExt;

use crate::app::command::ViewChangeCommand;
use crate::appop::AppOp;

impl AppOp {
    pub fn load_webview_view(&self, url: String) {        
        let webview_headerbar = self.create_webview_headerbar(&url);
        let webview_view = self.create_webview_view(&url);
        
        self.update_headerbar(webview_headerbar);
        self.update_view(webview_view);
    }

    pub fn connect_webview_headerbar(&self) {
        // Back button
        let back_button: gtk::Button = self.ui.builder.get_object("webview_back_button").unwrap();        
        let backend = self.backend.clone();
        back_button.connect_clicked(move |_| {
            backend.clone().send(ViewChangeCommand::PreviousView()).unwrap();
        });
    }

    fn create_webview_headerbar(&self, url: &String) -> gtk::Widget {
        let webview_headerbar: gtk::HeaderBar = self
            .ui
            .builder
            .get_object("webview_headerbar")
            .expect("Couldn't find webview_headerbar in ui file.");

        webview_headerbar.set_subtitle(Some(url));
        webview_headerbar.show_all();

        webview_headerbar.upcast::<gtk::Widget>()
    }

    fn create_webview_view(&self, url: &String) -> gtk::Widget {
        let webview = webkit2gtk::WebView::new();
        webview.load_uri(url);
        webview.show_all();

        webview.upcast::<gtk::Widget>()
    }
}