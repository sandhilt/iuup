pub mod prelude {
    use std::fmt::{self, Formatter};

    pub trait PackageManagerMinimal {
        fn get_name(&self) -> String;
        fn get_version(&self) -> Option<String>;
    }

    pub struct Structure {
        platform: String,
        arch: String,
        package_manager: Option<Box<dyn PackageManagerMinimal>>,
    }

    impl fmt::Debug for Structure {
        fn fmt(&self, f: &mut Formatter) -> fmt::Result {
            let package_manager = self
                .package_manager
                .as_ref()
                .map(|pm| pm.get_name())
                .unwrap_or("None".to_string());

            f.debug_struct("Structure")
                .field("platform", &self.platform)
                .field("arch", &self.arch)
                .field("package_manager", &package_manager)
                .finish()
        }
    }

    impl Structure {
        pub fn new(platform: String, arch: String) -> Self {
            Self {
                platform,
                arch,
                package_manager: None,
            }
        }

        pub fn is_supported(&self) -> bool {
            self.platform == "linux" && self.arch == "x86_64"
        }
    }
}
