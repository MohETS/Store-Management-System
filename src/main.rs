use cursive::views::{Dialog, TextView};

#[cfg(test)]
mod main_test;

fn main() {
    // Creates the cursive root - required for every application.
    let mut siv = cursive::default();

    siv.add_layer(Dialog::around(TextView::new("Hello Dialog!"))
        .title("Lab - LOG430")
        .button("Quit", |s| s.quit()));

    siv.run();
}