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
                "I'm sad" => String::from(""),
                "What's that thing you say about Joan or Arc?" => String::from("And now I know how Joan of Arc felt
                    Now I know how Joan of Arc felt
                    As the flames rose to her Roman nose
                    And her Walkman started to melt"),
                "What do you think about The Cure?" => String::from("Robert Smith is a whingebag"),
                "Is music addicting?" => String::from("Music is like a drug, but there are no rehabilitation centers"),
                "What is it like to be passionate?" => String::from("The fire in the belly is essential, otherwise you become Michael Buble
                    – famous and meaningless."),
                "Do you like long hair?" => String::from("Long hair is an unpardonable offense which should be punishable by death"),
                _ => String::from("I dont' have much to say about this.")
            }
        }
    }
}
