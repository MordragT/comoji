{
  inputs = {
    utils.url = "github:numtide/flake-utils";
    naersk.url = "github:nmattia/naersk";
  };

  outputs = { self, nixpkgs, utils, naersk }:
    utils.lib.eachDefaultSystem (system: let
      pkgs = nixpkgs.legacyPackages."${system}";
      naersk-lib = naersk.lib."${system}";
    in rec {
      
      overlay = import ./overlay.nix;
      
      # `nix build`
      packages.gitmoji = naersk-lib.buildPackage {
        pname = "gitmoji";
        root = ./.;
        buildInputs = with pkgs; [
          openssl
          pkgconfig
        ];
      };
      defaultPackage = packages.gitmoji;

      # `nix run`
      apps.gitmoji = utils.lib.mkApp {
        drv = packages.gitmoji;
      };
      defaultApp = apps.gitmoji;

      # `nix develop`
      devShell = pkgs.mkShell {
        nativeBuildInputs = with pkgs; [ rustc cargo ];
      };
    });
}
