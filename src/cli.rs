pub mod prelude {

    #[derive(Debug)]
    pub struct CommandPkgMan {
        content: String,
        options: Vec<String>,
    }

    impl CommandPkgMan {
        pub fn new(content: String, options: Vec<String>) -> Self {
            Self { content, options }
        }

        pub fn add_option(&mut self, option: String) {
            self.options.push(option);
        }
    }
}