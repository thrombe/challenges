{
  description = "yaaaaaaaaaaaaaaaaaaaaa";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-23.05";
    unstable-nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
  };

  outputs = { self, nixpkgs, unstable-nixpkgs }:
  let
    system = "x86_64-linux";

    pkgs = import nixpkgs {
      inherit system;
    };
    unstable = import unstable-nixpkgs {
      inherit system;
      # - [Overlays | NixOS & Flakes Book](https://nixos-and-flakes.thiscute.world/nixpkgs/overlays)
      overlays = [
        (self: super: {})
      ];
    };
  in
  {
    devShells."${system}".default = (import ./shell.nix { inherit pkgs unstable; });
  };
}
