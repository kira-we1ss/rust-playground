{
  description = "rust-playground";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { ... }@inputs:
    inputs.flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import inputs.rust-overlay) ];
        pkgs = import inputs.nixpkgs { inherit system overlays; };
      in with pkgs; {
        devShells.default = mkShell rec {
          nativeBuildInputs = [ # Here you put in whatever you'd need while the app runs. I didn't sort this very well.
            # Rust
            cargo-generate 
            (rust-bin.fromRustupToolchainFile ./rust-toolchain.toml)

            # Misc
            pkg-config git just binaryen clang mold diesel-cli cargo-nextest
          ];

          buildInputs = [
            # Does the same as nativeBuiltInputs but generally here go dependencies you need only during build.
            # These would be the same architecture as your current system architecture.
          ];

          LD_LIBRARY_PATH = "${lib.makeLibraryPath nativeBuildInputs}:${lib.makeLibraryPath buildInputs}:$LD_LIBRARY_PATH";
        };
      }
    );
}
