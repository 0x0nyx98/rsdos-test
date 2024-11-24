{ pkgs ? import <nixpkgs> { } }:
pkgs.mkShell {
  packages = with pkgs; [
    dosbox
    nasm
    open-watcom-bin
  ];

  WATCOM=pkgs.open-watcom-bin;
}

