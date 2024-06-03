pub mod prelude {
    use std::collections::HashMap;

    use crate::cli::prelude::CommandPkgMan;

    // pub struct FindResult<'a> {
    //     name: String,
    //     version: Option<String>,
    //     description: Option<&'a str>,
    // }
    pub struct FilterForFind {
        pub name: String,
        pub version: Option<String>,
    }
    pub struct ArgForInstall {
        name: String,
        version: Option<String>,
    }
    pub type OptionsForInstall = HashMap<String, String>;

    pub trait Install {
        async fn install(&self, arg: ArgForInstall, options: OptionsForInstall) -> bool;
    }
    pub trait Find {
        async fn find(&self, filter: FilterForFind) -> CommandPkgMan;
    }

    pub trait PackageManager: Default + Send + Sync + Install + Find {}
}
