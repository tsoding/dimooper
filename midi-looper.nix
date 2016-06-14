let
  pkgs = import <nixpkgs> {};
  stdenv = pkgs.stdenv;
  funs = pkgs.callPackage ./rust-nightly.nix { };
in rec {

  rustNightly = funs.rust {
    date = "2016-06-12";
    hash = "07674ikgc51d6kmbarb2z2izbh63jlxhc9078jk3f11v3s73n73q";
  };

  midiLooperEnv = stdenv.mkDerivation rec {
    name = "midi-looper-env";
    version = "0.0.1";
    src = ./.;
    buildInputs = [ pkgs.portmidi pkgs.SDL2 rustNightly ];
  };
}
