let
  pkgs = import <nixpkgs> {};
in
pkgs.mkShell {
    strictDeps = true;
    nativeBuildInputs = [
      pkgs.cargo
      pkgs.rustc
      pkgs.gcc
    ];
  } 