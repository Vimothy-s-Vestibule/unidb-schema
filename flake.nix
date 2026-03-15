{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
        dc = pkgs.diesel-cli.override {
          sqliteSupport = false;
          mysqlSupport = false;
        };
      in {
        devShells.default = pkgs.mkShell {
          nativeBuildInputs = [ dc ];
        };
      });
}