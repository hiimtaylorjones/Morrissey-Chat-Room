#[macro_use]
extern crate helix;

ruby! {
    class MorrisseyBot {
        def hello() -> String {
            return String::from("Hello from morrissey_bot!");
        }

        def translate(text: String) -> String {
            match &*text {
                "I don't like some people" => String::from("In my life, why do I smile At people who I'd much rather kick in the eye?"),
                _ => String::from("I dont' have much to say about this.")
            }
        }
    }
}
