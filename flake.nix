{
  inputs = { nixpkgs.url = "github:nixos/nixpkgs/nixpkgs-unstable"; };

  outputs = { self, nixpkgs, ... }:
    let
      system = "x86_64-linux";
      pkgs = import nixpkgs { inherit system; };
      dc = pkgs.diesel-cli.override {
        sqliteSupport = false;
        mysqlSupport = false;
      };
    in {
      devShells.${system}.default = nixpkgs.legacyPackages.${system}.mkShell {
        nativeBuildInputs = [ dc ];
      };
    };
}