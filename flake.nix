{
  description = "A library for ReSet applications.";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-parts = {
      url = "github:hercules-ci/flake-parts";
      inputs.nixpkgs-lib.follows = "nixpkgs";
    };
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = inputs @ {
    self,
    flake-parts,
    ...
  }:
    flake-parts.lib.mkFlake {inherit inputs;} {
      systems = [
        "x86_64-linux"
        "aarch64-linux"
      ];

      perSystem = {
        pkgs,
        system,
        ...
      }: {
        _module.args.pkgs = import self.inputs.nixpkgs {
          inherit system;
          overlays = [
            (
              import
              inputs.rust-overlay
            )
          ];
        };
        devShells.default = pkgs.mkShell {
          nativeBuildInputs = [
            pkgs.pkg-config
            pkgs.dbus
          ];

          buildInputs = [
            pkgs.dbus
            pkgs.libadwaita
            pkgs.pulseaudio
            (pkgs.rust-bin.selectLatestNightlyWith
              (toolchain: toolchain.default))
            pkgs.rust-analyzer
            pkgs.clippy
          ];
        };
      };
    };
}
