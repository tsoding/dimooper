let
  pkgs = import <nixpkgs> {};
  stdenv = pkgs.stdenv;
  funs = pkgs.callPackage ./rust-nightly.nix { };

  # Archive is here: https://static.rust-lang.org/dist/index.html
  rustNightly = funs.rust {
    date = "2016-09-08";
    hash = "19m8xlhq608f8rw4dy78d9s1gsfwpj9b5pma7i3ivdkh9ap751h8";
  };

  rustSrcNightly = funs.rustSrc {
    date = "2016-09-08";
    hash = "1g1mw6m1w9yp2lppmy37cz9rq5wffsgy6f00rmjy60iqb77ff25y";
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
