# { lib, rustPlatform }:
# rustPlatform.buildRustPackage rec {
{ stdenv, lib, makeRustPlatform, fetchFromGitHub, pkgs }:
let
  mozRepo = fetchFromGitHub {
    owner = "mozilla";
    repo = "nixpkgs-mozilla";
    rev = "b5f2af80f16aa565cef33d059f27623d258fef67";
    sha256 = "0s552nwnxcn6nnzrqaazhdgx5mm42qax9wy1gh5n6mxfaqi6dvbr";
  };
  # `mozPkgs` is the package set of `mozRepo`; this differs from their README
  # where they use it as an overlay rather than a separate package set
  mozPkgs = import "${mozRepo}/package-set.nix" { inherit pkgs; };
  channel = mozPkgs.rustChannelOf { date = "2019-11-29"; channel = "nightly"; };
  nightlyRustPlatform = makeRustPlatform {
    rustc = channel.rust;
    cargo = channel.cargo;
  };
in

nightlyRustPlatform.buildRustPackage rec {
# stdenv.mkDerivation rec {
  pname = "actix_i18n";
  version = "0.6.1";
  cargoSha256 = "0p8m9xqb2rwffrn3aqwjhm19fx6q8kx4c6f4f96ilixr783f5m7d";
  src = ./.;

  # buildPhase = ''
  #   # prevent referring /homeless-shelter
  #   export HOME=$(pwd)
  #   ${channel.cargo}/bin/cargo build  
  #   '';
  # installPhase = ''
  #   ${channel.cargo}/bin/cargo install
  #   '';

  meta = {
    description = "Books management";
    maintainers = with lib.maintainers; [ mmai ];
  };
}
