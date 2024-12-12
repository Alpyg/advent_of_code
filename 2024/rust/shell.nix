{ pkgs ? import <nixpkgs> { } }:

with pkgs;

mkShell rec {
  nativeBuildInputs = [ pkg-config ];
  buildInputs = [ openssl just ];
  LD_LIBRARY_PATH = lib.makeLibraryPath buildInputs;
}