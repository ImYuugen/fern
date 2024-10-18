{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/release-24.05";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    { self
    , nixpkgs
    , rust-overlay
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


            libxkbcommon
            wayland
            xorg.libX11
            xorg.libXcursor
            xorg.libXrandr
            xorg.libXi
  
            shaderc
            directx-shader-compiler
            ocl-icd
            libGL
            vulkan-headers
            vulkan-loader
            vulkan-tools
            vulkan-tools-lunarg
            vulkan-validation-layers

            rust-analyzer
            rust-bin.stable.latest.default
          ];
          shellHook = ''
            export LD_LIBRARY_PATH="$LD_LIBRARY_PATH:${builtins.toString (pkgs.lib.makeLibraryPath buildInputs)}";
          '';         nativeBuildInputs = with pkgs; [];
      };
    };
}
