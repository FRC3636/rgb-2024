{ pkgs, lib, naersk, stdenv, ... }:

naersk.buildPackage {
  name = "rgb-2024";
  version = "0.0.1";
  src = ./.;

  preBuild = ''
    export BINDGEN_EXTRA_CLANG_ARGS="$(< ${stdenv.cc}/nix-support/libc-crt1-cflags) \
      $(< ${stdenv.cc}/nix-support/libc-cflags) \
      $(< ${stdenv.cc}/nix-support/cc-cflags) \
      $(< ${stdenv.cc}/nix-support/libcxx-cxxflags) \
      ${lib.optionalString stdenv.cc.isClang "-idirafter ${stdenv.cc.cc}/lib/clang/${lib.getVersion stdenv.cc.cc}/include"} \
      ${lib.optionalString stdenv.cc.isGNU "-isystem ${stdenv.cc.cc}/include/c++/${lib.getVersion stdenv.cc.cc} -isystem ${stdenv.cc.cc}/include/c++/${lib.getVersion stdenv.cc.cc}/${stdenv.hostPlatform.config}"}
    "
  '';

  # cargoLock = {
  #   lockFile = ./Cargo.lock;
  #   outputHashes = {
  #     "frclib-nt4-0.1.5" =
  #       "sha256-mr4gl1bcmOKHu2TJIC3nrsTzbQhFidZdtppFq5+2T1M=";
  #     "shark-0.1.0" = "sha256-w2wGGbWCDvzR6ZmcrEqiEOXakg2Xfu9WaU895ZQe1Xs=";
  #   };
  # };

  buildInputs = [ pkgs.clang ];

  LIBCLANG_PATH = "${pkgs.libclang.lib}/lib";
}
