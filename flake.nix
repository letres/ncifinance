{
  inputs.nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
  inputs.nci.url = "github:yusdacra/nix-cargo-integration";
  inputs.nci.inputs.nixpkgs.follows = "nixpkgs";
  inputs.parts.url = "github:hercules-ci/flake-parts";
  inputs.parts.inputs.nixpkgs-lib.follows = "nixpkgs";
  inputs.devshell.url = "github:numtide/devshell";

  outputs = inputs @ {
    parts,
    nci,
    nixpkgs,
    ...
  }:
    parts.lib.mkFlake {inherit inputs;} {

      systems = ["x86_64-linux"];
      imports = [nci.flakeModule ];
      perSystem = {config,pkgs, ...}: let
        # TODO: change this to your crate's name
        crateName = "my-crate";
        # shorthand for accessing this crate's outputs
        # you can access crate outputs under `config.nci.outputs.<crate name>` (see documentation)
        crateOutputs = config.nci.outputs.${crateName};
      in {
        # declare projects
        # relPath is the relative path of a project to the flake root
        # TODO: change this to your crate's path
        #devshells.default = {packages=[pkgs.openssl crateOutputs];};
        nci.projects.${crateName}.relPath = "";
        # configure crates
        nci.crates.${crateName} = {
          # export crate (packages and devshell) in flake outputs
          # alternatively you can access the outputs and export them yourself (see below)
          export = true;
          overrides.add-inputs.overrideAttrs = old: {
              buildInputs = (old.buildInputs or []) ++ [pkgs.openssl pkgs.pkg-config];
          };
          # look at documentation for more options
        };
        # export the crate devshell as the default devshell
        devShells.default = crateOutputs.devShell;
        # export the release package of the crate as default package
        packages.default = crateOutputs.packages.release;

      };
    };
}