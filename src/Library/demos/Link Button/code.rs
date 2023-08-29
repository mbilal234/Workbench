use crate::workbench;
use gtk::prelude::*;

pub fn main() {
    let linkbutton: gtk::LinkButton = workbench::builder().object("linkbutton").unwrap();
    linkbutton.connect_clicked(move |_| {
        println!("Link has been visited");
    });

    linkbutton.connect_activate_link(|button| {
        println!("About to activate {}", button.uri());
        // Return true if handling the link manually, or
        // false to let the default behavior continue
        false.into()
    });
}
