{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-25.11";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    git-hooks = {
      url = "github:cachix/git-hooks.nix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    {
      self,
      nixpkgs,
      rust-overlay,
      git-hooks,
      ...
    }:
    let
      system = "x86_64-linux";
      overlays = [ (import rust-overlay) ];
      pkgs = import nixpkgs { inherit system overlays; };
    in
    {
      checks.${system} = {
        pre-commit-check = git-hooks.lib.${system}.run {
          src = ./.;
          hooks = {
            # Rust
            rustfmt.enable = true;
            cargo-check.enable = true;
            check-toml.enable = true;

            # Misc
            nixfmt-rfc-style.enable = true;
            trim-trailing-whitespace.enable = true;
          };
        };
      };

      devShells.${system}.default = pkgs.mkShell rec {
        inherit (self.checks.${system}.pre-commit-check) shellHook;

        buildInputs = with pkgs; [
          # Rust stuff
          rust-analyzer
          rust-bin.stable.latest.default

          # Networking/Libs stuff
          pkg-config
          openssl
          glib-networking

          # Utils
          jq
        ];
        LD_LIBRARY_PATH = "$LD_LIBRARY_PATH:${builtins.toString (pkgs.lib.makeLibraryPath buildInputs)}";
        GIO_MODULE_DIR = "${pkgs.glib-networking}/lib/gio/modules/";
      };
    };
}
