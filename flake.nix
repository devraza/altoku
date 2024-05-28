{
  description = "Rust development environment for altoku";

  inputs = {
    utils.url = "github:numtide/flake-utils";
    nixpkgs-unstable.url = "github:nixos/nixpkgs/nixos-unstable";
  };

  outputs = {
    self,
    nixpkgs-unstable,
    utils,
    ...
  }:
    utils.lib.eachDefaultSystem
    (
      system: let
        pkgs = import nixpkgs-unstable { inherit system; };
      in rec
      {
        # Executed by `nix build`
        packages.default = pkgs.rustPlatform.buildRustPackage {
          pname = "altoku";
          version = "0.1.0";
          src = ./.;
          cargoLock.lockFile = ./Cargo.lock;
        };

        # Executed by `nix run`
        apps.default = utils.lib.mkApp {drv = packages.default;};

        # Used by `nix develop`
        devShells.default = pkgs.mkShell rec {
          shellHook = ''export LD_LIBRARY_PATH="$LD_LIBRARY_PATH:${pkgs.lib.makeLibraryPath [
            pkgs.openssl_3_3
          ]}"'';
          buildInputs = with pkgs; [
            openssl_3_3 pkg-config
          ];
        };
      }
    );
}
