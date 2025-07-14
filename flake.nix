{
  description = "TimeKeeper Nix Flake";

  inputs = {
    nixpkgs.url = "nixpkgs/nixos-25.05";
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
        rust = (pkgs.rust-bin.nightly."${rustVersion}".default.override {
          extensions = [ "rust-src" ];
        });
        buildInputs = with pkgs; [
          glibc
          cairo
          libsoup_3
          webkitgtk_4_1
          xdotool

          alsa-lib
          systemd
          xorg.libX11
          xorg.libXtst
        ];
        nativeBuildInputs = with pkgs; [
          rust
          pkg-config
          gcc

          git
          cmake
          dioxus-cli

          makeWrapper
        ] ++ buildInputs;
        rustPlatform = pkgs.makeRustPlatform {
          cargo = rust;
          rustc = rust;
        };
      in with pkgs; rec
      {
        devShells.default = mkShell {
          inherit nativeBuildInputs;
        };

        packages.default = rustPlatform.buildRustPackage rec {
          pname = "timekeeper";
          version = "0.2.0";
          
          src = ./.;

          cargoHash = "sha256-vwhpKzVbRwkITZQ6m7TFUfMalL8ri/p0W8WUVccSmH8=";

          inherit buildInputs nativeBuildInputs;

          # Dioxus doesn't support Cargo version 4, so pretend that we aren't using it.
          prePatch = ''
            sed -i -e "s/version = 4/version = 3/" ./Cargo.lock
          '';

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

        apps.default = {
          type = "app";
          program = "${packages.default}/bin/timekeeper";
        };
      }
    );
}
