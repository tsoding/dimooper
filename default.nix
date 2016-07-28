let
  pkgs = import <nixpkgs> {};
  stdenv = pkgs.stdenv;
  funs = pkgs.callPackage ./rust-nightly.nix { };
  rustNightly = funs.rust {
    date = "2016-06-12";
    hash = "07674ikgc51d6kmbarb2z2izbh63jlxhc9078jk3f11v3s73n73q";
  };
in rec {
  dimooperEnv = stdenv.mkDerivation rec {
    name = "dimooper-env";
    version = "0.0.1";
    src = ./.;
    buildInputs = [ pkgs.portmidi pkgs.SDL2 pkgs.SDL2_ttf rustNightly ];
  };
}
