{
  description = "Stoic Quotes";
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-25.05";

    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs = {
        nixpkgs.follows = "nixpkgs";
        flake-utils.follows = "flake-utils";
      };

    };

    flake-utils.url = "github:numtide/flake-utils";

    pre-commit-hooks.url = "github:cachix/pre-commit-hooks.nix";
  };

  outputs =
    {
      self,
      nixpkgs,
      rust-overlay,
      flake-utils,
      pre-commit-hooks,
      ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        overlays = [ (import rust-overlay) ];

        pkgs = import nixpkgs { inherit system overlays; };

        rust = pkgs.rust-bin.stable."1.87.0".default.override {
          targets = [ "wasm32-unknown-unknown" ];
        };

        package_version = "0.5.0";

        buildInputs = with pkgs; [
          bashInteractive
          rust
          openssl
          tailwindcss
          dioxus-cli
        ];
        nativeBuildInputs = with pkgs; [ pkg-config ];
      in
      with pkgs;
      {
        checks = {
          pre-commit-check = pre-commit-hooks.lib.${system}.run {
            src = ./.;
            hooks = {
              typos.enable = true;
              rustfmt.enable = true;
              clippy.enable = true;
              nixfmt-rfc-style.enable = true;
            };
          };
        };

        devShells.default =
          let
            # pre-commit-checks
            _shellHook = (self.checks.${system}.pre-commit-check.shellHook or "");
          in
          mkShell {
            inherit buildInputs;

            shellHook = "${_shellHook}";
          };

        packages.default = import ./build.nix {
          inherit (pkgs) lib rustPlatform;
          inherit
            rust
            buildInputs
            nativeBuildInputs
            package_version
            ;
        };

        flake.overlays.default = (
          final: prev: {
            stoicquotes = self.packages.${final.system}.default;
          }
        );
      }
    );
}
