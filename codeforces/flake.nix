{
  description = "yaaaaaaaaaaaaaaaaaaaaa";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-23.05";
    nixpkgs-unstable.url = "github:nixos/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = inputs @ {...}:
    inputs.flake-utils.lib.eachDefaultSystem (system: let
      pkgs = import inputs.nixpkgs {
        inherit system;
      };
      unstable = import inputs.nixpkgs-unstable {
        inherit system;
      };
    in {
      devShells.default = pkgs.mkShell {
        nativeBuildInputs = with pkgs; [
          # rustup
          unstable.cargo
          unstable.rustc
          unstable.rust-analyzer
          unstable.clippy

          (python39.withPackages (ps: []))
          python310Packages.python-lsp-server
          python310Packages.ruff-lsp # python linter
          python310Packages.black # python formatter
        ];
        shellHook = ''
          export RUST_BACKTRACE="1"
        '';
      };
    });
}
