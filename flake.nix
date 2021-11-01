{
  inputs = {
    utils.url = "github:numtide/flake-utils";
    naersk.url = "github:nmattia/naersk";
    fenix.url = "github:nix-community/fenix";
  };

  outputs = { self, nixpkgs, utils, naersk, fenix }: let
    overlay = (final: prev: {
      gitmoji = prev.callPackage (import ./default.nix) {
        inherit naersk fenix;
      };
    });
  in { overlay = overlay; } // utils.lib.eachDefaultSystem (system: let
    pkgs = nixpkgs.legacyPackages."${system}";
    naersk-lib = naersk.lib."${system}";
  in rec {
    # `nix build`
    packages.gitmoji = import ./default.nix {
      inherit system;
      inherit (nixpkgs) lib;
      inherit pkgs;
      inherit naersk fenix;
    };
      
    defaultPackage = packages.gitmoji;

    # `nix run`
    apps.gitmoji = utils.lib.mkApp {
      drv = packages.gitmoji;
    };
    defaultApp = apps.gitmoji;

    # `nix develop`
    devShell = pkgs.mkShell {
      nativeBuildInputs = with pkgs; [
        rustc
        cargo
        openssl
        pkgconfig
      ];
    };
  });
}
