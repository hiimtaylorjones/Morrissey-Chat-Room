#[macro_use]
extern crate helix;

ruby! {
    class MorriseyBot {
        def hello() {
            println!("Hello from morrisey_bot!");
        }
    }
}
