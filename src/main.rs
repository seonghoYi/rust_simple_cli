use components::tty::CharInterface;

mod components;

fn main() {
    let mut term = components::tty::TermTTY::get_term().unwrap();
    let mut buf = [0; 128];
    term.init();

    loop {
        if term.read(&mut buf).unwrap() > 0 {
            term.write(&buf).unwrap();
        }
    }

    term.de_init();
}
