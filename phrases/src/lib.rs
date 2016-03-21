pub mod english {
    pub mod greetings {
        pub fn hello() -> String {
            "Hello!".to_string()
        }
    }

    pub mod farewells {
        pub fn goodbye() -> String {
            "Goodbye.".to_string()
        }
    }
}

mod japanese {
    mod greetings {
        fn hello() -> String {
            "こんにちは".to_string()
        }
    }

    mod farewells {
        fn goodbye() -> String {
            "さようなら".to_string()
        }
    }
}