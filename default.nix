
with import <nixpkgs> {};

stdenv.mkDerivation {
  name = "discord-bot";
  buildInputs = [
    pkgs.cargo
  ];
}
