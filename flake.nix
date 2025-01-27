{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/release-24.11";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    rustacean = {
      url = "github:mrcjkb/rustaceanvim";
    };
  };

  outputs =
    { self
    , nixpkgs
    , rust-overlay
    , rustacean
    , ...
    } @ inputs:
    let
      system = "x86_64-linux";
      overlays = [ (import rust-overlay) ];
      pkgs = import nixpkgs { inherit system overlays; };
    in
    {
      devShells.${system}.default = pkgs.mkShell rec {
        buildInputs = with pkgs; [
          pre-commit
          rustacean.packages.${system}.codelldb
          rust-analyzer
          rust-bin.stable.latest.default
          jq
        ];
        LD_LIBRARY_PATH="$LD_LIBRARY_PATH:${builtins.toString (pkgs.lib.makeLibraryPath buildInputs)}";
        GIO_MODULE_DIR="${pkgs.glib-networking}/lib/gio/modules/";
      };
    };
}
