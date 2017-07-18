let
  pkgs = import <nixpkgs> {};
  stdenv = pkgs.stdenv;
in rec {
  dimooperEnv = stdenv.mkDerivation rec {
    name = "dimooper-env";
    version = "0.0.1";
    src = ./.;
    buildInputs = [ pkgs.portmidi pkgs.SDL2 pkgs.SDL2_ttf pkgs.rustChannels.nightly.rust ];
    # RUST_SRC_PATH = "${rustSrcNightly}/usr/src/rust/src/";
  };
}
