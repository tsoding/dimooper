let
  pkgs = import <nixpkgs> {};
  stdenv = pkgs.stdenv;
in rec {
  midiLooperEnv = stdenv.mkDerivation rec {
    name = "midi-looper-env";
    version = "0.0.1";
    src = ./.;
    buildInputs = [ pkgs.rustc pkgs.cargo pkgs.portmidi ];
  };
}
