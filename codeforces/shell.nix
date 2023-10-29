{ pkgs ? import <nixpkgs> {}, unstable ? import <nixos-unstable> {} }:

pkgs.mkShell {
    packages = with pkgs; [
        # rustup
        unstable.cargo
        unstable.rustc
        unstable.rust-analyzer
        unstable.clippy

        (python39.withPackages(ps: []))
        python310Packages.python-lsp-server
        python310Packages.ruff-lsp # python linter
        python310Packages.black # python formatter
    ];
}
