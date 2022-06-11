{
  inputs = {
    utils.url = "github:numtide/flake-utils";
    naersk.url = "github:nmattia/naersk";
    fenix.url = "github:nix-community/fenix";
  };

  outputs = { self, nixpkgs, utils, naersk, fenix }: let
    overlay = (final: prev: {
      comoji = prev.callPackage (import ./default.nix) {
        inherit naersk fenix;
      };
    });
  in { overlay = overlay; } // utils.lib.eachDefaultSystem (system: let
    pkgs = nixpkgs.legacyPackages."${system}";
    naersk-lib = naersk.lib."${system}";
    toolchain = fenix.packages.${system}.complete;
  in rec {
    # `nix build`
    packages.comoji = import ./default.nix {
      inherit system;
      inherit (nixpkgs) lib;
      inherit pkgs;
      inherit naersk fenix;
    };
    packages.default = packages.comoji;

    # `nix run`
    apps.comoji = utils.lib.mkApp {
      drv = packages.comoji;
    };
    apps.default = apps.comoji;

    # `nix develop`
    devShell = pkgs.mkShell {
      nativeBuildInputs = with pkgs; [
        (toolchain.withComponents [
          "cargo" "rustc" "rust-src" "rustfmt" "clippy"    
        ])
        openssl
        pkgconfig
      ];
    };
  });
}
