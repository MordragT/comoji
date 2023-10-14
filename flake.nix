{
  description = "Create beautiful git commit messages";

  inputs = {
    utils.url = "github:numtide/flake-utils";
  };

  outputs = {
    self,
    nixpkgs,
    utils,
  }:
    utils.lib.eachDefaultSystem
    (system: let
      pkgs = import nixpkgs {inherit system;};
    in rec {
      packages.default = pkgs.rustPlatform.buildRustPackage {
        pname = "comoji";
        version = "1.3.0";
        src = ./.;
        cargoLock.lockFile = ./Cargo.lock;

        meta = with pkgs.lib; {
          description = "Create beautiful git commit messages";
          homepage = "https://github.com/MordragT/comoji";
          license = licenses.mit;
          maintainers = with maintainers; [mordrag];
          mainProgram = "comoji";
        };
      };

      apps.default = utils.lib.mkApp {
        drv = packages.default;
      };

      devShells.default = pkgs.mkShell {
        nativeBuildInputs = with pkgs; [
          (with rustPlatform; [
            rust.cargo
            rust.rustc
            rustLibSrc
          ])
          clippy
          rustfmt
          openssl
          pkg-config
        ];

        RUST_SRC_PATH = "${pkgs.rustPlatform.rustLibSrc}";
      };
    })
    // {
      overlays.default = this: pkgs: {
        comoji = self.packages."${pkgs.system}".default;
      };
    };
}
