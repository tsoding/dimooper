let
  pkgs = import <nixpkgs> {};
  stdenv = pkgs.stdenv;
  funs = pkgs.callPackage ./rust-nightly.nix { };
in rec {

  rustNightly = funs.rust {
    date = "2016-04-09";
    hash = "07w2fs1c4jwzsdphcr6215py7f3nid8qf920hswfn9l3fy5x9jfz";
  };

  midiLooperEnv = stdenv.mkDerivation rec {
    name = "midi-looper-env";
    version = "0.0.1";
    src = ./.;
    buildInputs = [ pkgs.portmidi pkgs.SDL2 rustNightly ];
  };
}
