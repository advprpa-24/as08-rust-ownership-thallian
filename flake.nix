{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    fenix.url = "github:nix-community/fenix";
  };

  outputs =
    {
      nixpkgs,
      fenix,
      flake-utils,
      ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs { inherit system; };
        fx = fenix.packages.${system};
        rust = fx.combine [
          fx.stable.toolchain
        ];
        buildInputs = [
          rust
          pkgs.cargo-deny
          pkgs.rust-analyzer
        ];
      in
      {
        devShells.default = pkgs.mkShell {
          buildInputs = buildInputs;
        };
      }
    );
}
