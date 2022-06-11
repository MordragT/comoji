{ system
, lib
, pkgs
, naersk
, fenix
}:

let
  toolchain = with fenix.packages.${system};
    combine [
      minimal.rustc
      minimal.cargo
    ];
  naersk-lib = naersk.lib.${system}.override {
    cargo = toolchain;
    rustc = toolchain;
  };
in
naersk-lib.buildPackage {
  pname = "comoji";
  root = ./.;
  buildInputs = with pkgs; [
    openssl
    pkgconfig
  ];
  meta = with lib; {
    description = "comoji cli to create beautiful git commit messages";
  };
}
