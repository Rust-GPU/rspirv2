let
  pkgs = import <nixpkgs> {};
in with pkgs; stdenv.mkDerivation rec {
  name = "rspirv2";
  nativeBuildInputs = [ rustup spirv-tools ];
}
