pub mod prelude {
    use crate::{actions::prelude::{FilterForFind, Find}, cli::prelude::CommandPkgMan};

    // Language Specific
    pub struct Corepack {}
    pub struct PiP {}
    pub struct Cargo {}

    pub struct BrewLinux {}

    pub struct BrewMacOS {}
    pub struct Winget {}
    pub struct NixOS {}
    pub struct Pacman {}

    pub struct DebianApt {}

    pub struct DPKG {}
    pub struct UbuntuPPA {}

    impl Find for NixOS {
        async fn find(&self, filter: FilterForFind) -> CommandPkgMan {
            todo!()
        }
    }
}
