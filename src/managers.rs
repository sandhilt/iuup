pub mod prelude {
    use crate::actions::prelude::{FilterForFind, Find, FindResult, PackageManager};


    // Language Specific
    struct Corepack {}
    struct PiP {}
    struct Cargo {}

    struct BrewLinux {}

    struct BrewMacOS {}
    struct Winget {}
    struct NixOS {}
    struct Pacman {}

    struct DebianApt {}

    struct UbuntuApt {}
    struct UbuntuPPA {}

    impl Find for NixOS {
        async fn find(&self, filter: FilterForFind) -> FindResult {
            todo!()
        }
    }
}
