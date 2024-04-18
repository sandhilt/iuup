pub mod prelude {
    use std::collections::HashMap;

    pub struct FindResult<'a> {
        name: String,
        version: Option<String>,
        description: Option<&'a str>,
    }
    pub struct FilterForFind {
        name: String,
        version: Option<String>,
    }
    pub struct ArgForInstall {
        name: String,
        version: Option<String>,
    }
    type OptionsForInstall = HashMap<String, String>;

    pub trait Install {
        async fn install(&self, arg: ArgForInstall, options: OptionsForInstall) -> bool;
    }
    pub trait Find {
        async fn find(&self, filter: FilterForFind) -> FindResult;
    }

    pub trait PackageManager: Default + Send + Sync + Install + Find {}
}