{
  description = "Arrow ADBC Rust";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs";
    flake-utils.url = "github:numtide/flake-utils";
    arrow-adbc.url = "/home/aljaz/Projects/arrow-adbc-nix";
    arrow-nanoarrow.url = "/home/aljaz/Projects/arrow-nanoarrow-nix";
  };

  outputs = { self, nixpkgs, flake-utils, arrow-adbc, arrow-nanoarrow }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
      in {
        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
            pkgs.cmake
            arrow-nanoarrow.packages.${system}.default
            arrow-adbc.packages.${system}.driver_postgresql
            arrow-adbc.packages.${system}.driver_sqlite
            arrow-adbc.packages.${system}.driver_manager
            pkgs.postgresql.lib

            pkgs.rustup
            pkgs.rust-bindgen
          ];

          ADBC_DRIVER_POSTGRESQL_LIB_DIR = arrow-adbc.packages.${system}.driver_postgresql + "/lib";
          POSTGRESQL_LIB_DIR = pkgs.postgresql.lib + "/lib";
        };
      });
}
