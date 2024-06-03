pub mod prelude {
    use crate::{
        actions::prelude::{
            ArgForInstall, FilterForFind, Find, Install, OptionsForInstall, PackageManager,
        },
        cli::prelude::CommandPkgMan,
    };

    // Language Specific
    pub struct Corepack {}
    pub struct PiP {}
    pub struct Cargo {}

    pub struct BrewLinux {}

    pub struct BrewMacOS {}
    pub struct Winget {}

    #[derive(Default)]
    pub struct NixOS {}
    pub struct Pacman {}

    pub struct DebianApt {}

    pub struct DPKG {}
    pub struct UbuntuPPA {}

    impl Find for NixOS {
        async fn find(&self, filter: FilterForFind) -> CommandPkgMan {
            let mut command = CommandPkgMan::new(
                "nix-env".to_string(),
                vec!["-qaP".to_string(), "--description".to_string()],
            );

            command.add_option(filter.name);

            command
        }
    }

    impl Install for NixOS {
        async fn install(&self, arg: ArgForInstall, options: OptionsForInstall) -> bool {
            todo!()
        }
    }

    impl PackageManager for NixOS {}
}
