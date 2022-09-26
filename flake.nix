{
  description = "Create beautiful git commit messages";

  inputs = {
    utils.url = "github:numtide/flake-utils";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, utils, fenix }:
    utils.lib.eachDefaultSystem
      (system:
        let
          pkgs = import nixpkgs {
            inherit system;
            overlays = [ fenix.overlay ];
          };
          toolchain = pkgs.fenix.complete;
        in
        rec {
          packages.default = (pkgs.makeRustPlatform {
            inherit (toolchain) cargo rustc;
          }).buildRustPackage {
            pname = "comoji";
            version = "1.2.0";
            src = ./.;
            cargoLock.lockFile = ./Cargo.lock;

            meta = with pkgs.lib; {
              description = "Create beautiful git commit messages";
              homepage = "https://github.com/MordragT/comoji";
              license = licenses.mit;
              maintainers = with maintainers; [ mordrag ];
              mainProgram = "comoji";
            };
          };

          apps.default = utils.lib.mkApp {
            drv = packages.default;
          };

          devShells.default = pkgs.mkShell {
            nativeBuildInputs = with pkgs; [
              (with toolchain; [
                cargo
                rustc
                rust-src
                clippy
                rustfmt
              ])
              openssl
              pkg-config
            ];
          };

          overlays.default = self: pkgs: {
            comoji = packages.default;
          };
        });
}
