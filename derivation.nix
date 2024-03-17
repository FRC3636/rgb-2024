{ naersk, ... }:

naersk.buildPackage {
  name = "rgb-2024";
  version = "0.0.1";
  src = ./.;
}
