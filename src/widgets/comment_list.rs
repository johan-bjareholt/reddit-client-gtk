use gtk;
use gtk::prelude::*;
use gtk::WidgetExt;

use crate::appop::AppOp;
use crate::widgets;
use redditor::types::{Comment, CommentList};

pub struct CommentListBox<'a> {
    op: &'a AppOp,
    comment_list: &'a CommentList,
}

impl<'a> CommentListBox<'a> {
    pub fn new(op: &'a AppOp, comment_list: &'a CommentList) -> CommentListBox<'a> {
        CommentListBox {
            op,
            comment_list,
        }
    }
    
    pub fn widget(&self) -> gtk::ListBox {
        let comments_listbox = gtk::ListBox::new();
        comments_listbox.set_activate_on_single_click(true);
        comments_listbox.set_selection_mode(gtk::SelectionMode::None);
        comments_listbox.show();

        self.connect_comments_listbox(&comments_listbox);
        self.pack_comments_listbox_loop(&comments_listbox, self.comment_list.comments(), 0);

        comments_listbox
    }

    pub fn connect_comments_listbox(&self, comments_listbox: &gtk::ListBox) {
        comments_listbox.connect_row_activated(move |comments_listbox, comment_row| {
            // TODO: This is all a bad hack, looking for ListBoxRows
            // with class containing depth, and their children
            // recursively, and showing/hiding them on click based
            // on depth beign greater than the clicked row's depth.

            let mut clicked_comment_depth = 0;

            let class_list = comment_row.get_style_context().list_classes();
            for class_name in class_list {
                let class_name = class_name.as_str();
                if class_name.contains("comment--depth") {
                    clicked_comment_depth = class_name.replace("comment--depth-", "").parse::<i32>().unwrap();
                    break;
                }
            }

            let comment_row_index = comment_row.get_index();
            let mut is_hiding = false;

            for (index, row) in comments_listbox.get_children().iter().enumerate() {
                let index = index as i32;

                if index == comment_row_index { // If this is the clicked row, toggle the collapsed_notice_container
                    is_hiding = comments_listbox.get_row_at_index(index + 1).unwrap().upcast::<gtk::Widget>().is_visible();
                    let comment_container = comment_row.get_children()[0].clone().downcast::<gtk::Container>().unwrap();
                    let collapsed_notice_container = comment_container.get_children()[1].clone();
                    if is_hiding {
                        collapsed_notice_container.show();
                    }
                    else {
                        collapsed_notice_container.hide();
                    }
                }
                else if index > comment_row_index { // else, toggle the children comment rows
                    let class_list = row.get_style_context().list_classes();
                    for class_name in class_list {
                        let class_name = class_name.as_str();
                        if class_name.contains("comment--depth") {
                            let cur_comment_depth = class_name.replace("comment--depth-", "").parse::<i32>().unwrap();

                            if cur_comment_depth > clicked_comment_depth {
                                if is_hiding {
                                    row.hide();
                                }
                                else {
                                    row.show();
                                }
                            }

                            if cur_comment_depth == clicked_comment_depth {
                                return
                            }

                            break;
                        }
                    }
                }
            }
        });
    }

    fn pack_comments_listbox_loop(&self, comments_listbox: &gtk::ListBox, comment_list: Vec<&Comment>, depth: u8) {
        for comment in comment_list {
            let comment_widget = widgets::comment::CommentBox::new(&self.op, &comment, &depth).widget();
            comments_listbox.add(&comment_widget);

            let replies = comment.replies();
            if replies.len().clone() > 0 {
                self.pack_comments_listbox_loop(&comments_listbox, replies, depth + 1);
            }
        }
    }
}
