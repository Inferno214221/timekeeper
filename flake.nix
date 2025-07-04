{
  description = "TimeKeeper Nix Flake";

  inputs = {
    nixpkgs.url = "nixpkgs/nixos-24.11";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
        rustVersion = "2025-04-21";
        buildInputs = with pkgs; [
          glibc
          cairo
          libsoup_3
          webkitgtk_4_1
          xdotool
        ];
        nativeBuildInputs = with pkgs; [
          (rust-bin.nightly."${rustVersion}".default.override {
            extensions = [ "rust-src" ];
          })
          pkg-config
          gcc
          git
          cmake
          dioxus-cli
        ] ++ buildInputs;
      in
      {
        devShells.default = with pkgs; mkShell {
          inherit nativeBuildInputs;
        };

        packages.default = with pkgs; rustPlatform.buildRustPackage rec {
          pname = "timekeeper";
          version = "0.2.0";
          
          src = ./.;

          cargoHash = "sha256-pTufUdJoz8ql2m5bxQtIF3yQw4EMeY57WXNeuRZ2W9I=";

          inherit buildInputs nativeBuildInputs;

          postInstall = ''
            mkdir -p $out/share/applications
            cp $src/timekeeper.desktop $out/share/applications/timekeeper.desktop
          '';

          meta = {
            description = "A simple stopwatch / timer with a GUI written in Rust using Dioxus";
            homepage = "https://github.com/Inferno214221/timekeeper";
            license = lib.licenses.gpl3;
          };
        };
      }
    );
}
