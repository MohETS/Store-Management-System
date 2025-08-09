use cursive::event::Event::Key;
use cursive::event::Key::Esc;
use register::Register;
use shared::model::*;

mod register;


fn main() {
    let mut siv = cursive::default();
    let mut register = Register::new();

    register.setup_ui(&mut siv);
    siv.add_global_callback(Key(Esc), |s| s.quit());
    siv.run();
}








