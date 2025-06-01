{ pkgs ? import <nixpkgs> {} }:

pkgs.rustPlatform.buildRustPackage rec {
  pname = "dollpublish";
  version = "0.1.0";

  src = ./.;

  cargoHash = "sha256-ZmoCNz/jxgDShvgDQFbqf3z6P+ocDgV0qLurqNogGeQ=";

  nativeBuildInputs = with pkgs; [
    rustfmt
    clippy
  ];
}

