let
  pkgs = import <nixpkgs> {};
  stdenv = pkgs.stdenv;
  dimooperChannel = pkgs.rustChannelOf {
    date = "2017-03-01";
    channel = "nightly";
  };
in rec {
  dimooperEnv = stdenv.mkDerivation rec {
    name = "dimooper-env";
    version = "0.0.1";
    src = ./.;
    buildInputs = [ pkgs.portmidi pkgs.SDL2 pkgs.SDL2_ttf dimooperChannel.rust dimooperChannel.rust-src ];
    RUST_SRC_PATH = "${dimooperChannel.rust-src}/lib/rustlib/src/rust/src";
  };
}
