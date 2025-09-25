{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    crane.url = "github:ipetkov/crane";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs =
    {
      nixpkgs,
      crane,
      flake-utils,
      ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
        craneLib = crane.mkLib pkgs;
        gdscript-formatter = craneLib.buildPackage {
          src = ./.;

          cargoExtraArgs = "--bin=gdscript-formatter";
        };
      in
      {
        packages.default = gdscript-formatter;

        devShells.default = craneLib.devShell {
          inputsFrom = [ gdscript-formatter ];
        };

        checks = {
          inherit gdscript-formatter;
        };
      }
    );
}
