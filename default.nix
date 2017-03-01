let
  pkgs = import <nixpkgs> {};
  stdenv = pkgs.stdenv;
  funs = pkgs.callPackage ./rust-nightly.nix { };

  # Archive is here: https://static.rust-lang.org/dist/index.html
  rustNightly = funs.rust {
    date = "2017-03-01";
    hash = "1jrh2j9mlj13z52qz6w99h60azayqbx4bicimy49jx7zrlq0p9lr";
  };

  rustSrcNightly = funs.rustSrc {
    date = "2017-03-01";
    hash = "0i8p0463469x7138i4aslqs7yfh852p31pfp7rlc6kyw75pzvby1";
  };
in rec {
  dimooperEnv = stdenv.mkDerivation rec {
    name = "dimooper-env";
    version = "0.0.1";
    src = ./.;
    buildInputs = [ pkgs.portmidi pkgs.SDL2 pkgs.SDL2_ttf rustNightly rustSrcNightly ];
    RUST_SRC_PATH = "${rustSrcNightly}/usr/src/rust/src/";
  };
}
