# shell.nix

let
    pkgs = import <nixpkgs> {};

in
pkgs.mkShell {
    buildInputs = [
        pkgs.glibc
        # Add any additional dependencies here
        # In case you need to install Rust
        # rustc
        # cargo
    ];
}