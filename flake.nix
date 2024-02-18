{
  inputs = {
    flake-utils.url = "github:numtide/flake-utils";
    naersk.url = "github:nix-community/naersk";
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
  };
  outputs = { self, nixpkgs, flake-utils, naersk, }:
    (flake-utils.lib.eachSystem flake-utils.lib.allSystems (system:
      let
        pkgs = (import nixpkgs) { inherit system; };
        naersk' = pkgs.callPackage naersk { };
      in {
        packages = rec {
          rgb-2024 = pkgs.callPackage ./derivation.nix { naersk = naersk'; };
          default = rgb-2024;
        };
        devShells.default =
          (pkgs.mkShell.override { stdenv = pkgs.clangStdenv; }) {
            LIBCLANG_PATH = "${pkgs.libclang.lib}/lib";
          };
      }));
}
