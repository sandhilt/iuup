pub mod prelude {

    #[derive(Debug)]
    struct Command {
        content: String,
        options: Vec<String>,
    }

    impl Command {
        pub fn new(content: String, options: Vec<String>) -> Self {
            Self { content, options }
        }

        pub fn add_option(&mut self, option: String) {
            self.options.push(option);
        }
    }
}