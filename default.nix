{
  system ? builtins.currentSystem,
  rust-overlay ? (builtins.fetchTarball "https://github.com/oxalica/rust-overlay/archive/master.tar.gz"),
  cargo2nix ? (builtins.fetchGit "https://github.com/cargo2nix/cargo2nix"),
}:
let
  rustOverlay = import rust-overlay;
  cargo2nixOverlay = import "${cargo2nix}/overlay";

  pkgs = import <nixpkgs> {
    inherit system;
    overlays = [ cargo2nixOverlay rustOverlay ];
  };

  rustPkgs = pkgs.rustBuilder.makePackageSet' {
    rustChannel = "1.55.0";
    packageFun = import ./Cargo.nix;
    # packageOverrides = pkgs: pkgs.rustBuilder.overrides.all; # Implied, if not specified
  };
in
  rustPkgs.workspace.gitmoji {}
