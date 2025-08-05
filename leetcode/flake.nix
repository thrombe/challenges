{
  description = "yaaaaaaaaaaaaaaaaaaaaa";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = inputs @ {...}:
    inputs.flake-utils.lib.eachDefaultSystem (system: let
      pkgs = import inputs.nixpkgs {
        inherit system;
      };
    in {
      devShells.default = pkgs.mkShell {
        nativeBuildInputs = with pkgs; [
          # rustup
          cargo
          rustc
          rust-analyzer
          clippy
          rustfmt

          python312
          pyright # lsp (js)
          # basedpyright # lsp (js) (fork of pyright)
          ruff # python linter, formatter (rust)
          uv # python package manager, virtualenv (rust)

          # - [use clangd C/C++ LSP in any project](https://www.reddit.com/r/neovim/comments/17rhvtl/guide_how_to_use_clangd_cc_lsp_in_any_project/)
          cmake
          # Clangd from clang-tools must come first.
          # (hiPrio clang-tools.override {
          #   llvmPackages = llvmPackages_16;
          #   enableLibcxx = false;
          # })
          # Do not use the clangd from this package as it does not work correctly with
          # stdlib headers.
          # llvmPackages_16.libstdcxxClang

          clang
        ];
        shellHook = ''
          export RUST_BACKTRACE="1"
        '';
        LD_LIBRARY_PATH = "${pkgs.stdenv.cc.cc.lib}/lib";
      };
    });
}
