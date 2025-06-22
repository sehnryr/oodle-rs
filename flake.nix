{
  description = "oodle-rs development environment";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    {
      nixpkgs,
      flake-utils,
      rust-overlay,
      ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [
            (import rust-overlay)
          ];
        };

        stableToolchain = pkgs.lib.hiPrio (
          pkgs.rust-bin.stable.latest.minimal.override {
            extensions = [
              "rust-docs"
              "clippy"
              "rust-src"
            ];
          }
        );

        nightlyFmt = pkgs.rust-bin.selectLatestNightlyWith (
          toolchain:
          toolchain.minimal.override {
            extensions = [ "rustfmt" ];
          }
        );

        oodle = rec {
          version = "2.9.13";
          repo = "https://github.com/WorkingRobot/OodleUE";
          commit = "5b9aff507e34b5f69dba3bf12912deb551253606";
          path = "Engine/Source/Runtime/OodleDataCompression/Sdks/${version}";
          help = pkgs.fetchurl {
            url = "${repo}/raw/${commit}/${path}/help/oodle2.html";
            sha256 = "sha256-TcpTOSB9eXZoCYuj4H3op1gFuRTJnC1e6t9fdXQUI2o=";
          };
          lib = pkgs.stdenv.mkDerivation {
            pname = "oodle-lib";
            version = version;

            src = pkgs.fetchurl {
              url = "${repo}/raw/${commit}/${path}/lib/Linux/liboo2corelinux64.so.9";
              sha256 = "sha256-Gxl/YpNODYS+5TnJngFhUQJNCYrb3Oes4NSTusYAF/4=";
            };

            dontUnpack = true;

            installPhase = ''
              mkdir -p $out
              cp -r $src $out/liboo2corelinux64.so
            '';
          };
          src = pkgs.stdenv.mkDerivation {
            pname = "oodle-src";
            version = version;

            src = pkgs.fetchzip {
              url = "${repo}/raw/${commit}/${path}/src/oodle2_src_for_unreal_data_${version}.zip";
              sha256 = "sha256-qIbF92NsPAAmUDOGgU+8GVjsCPxvUwgoJciXzY8krcA=";
            };

            buildPhase = ''
              sed -i -e '/^static [^=]*(/ s/static //' \
                     -e '/^RADINLINE [^=]*(/ s/RADINLINE //' \
                     core/newlz.cpp
            '';

            installPhase = ''
              mkdir -p $out
              cp -r ./* $out
            '';
          };
        };
      in
      {
        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
            # Dependencies for bindgen
            llvmPackages.clang
            llvmPackages.libclang

            # Rust toolchain
            stableToolchain
            nightlyFmt
          ];

          LIBCLANG_PATH = "${pkgs.llvmPackages.libclang.lib}/lib";
          LIBRARY_PATH = "${oodle.lib}";

          shellHook = ''
            mkdir -p oodle

            rm oodle/help.html;
            rm oodle/src;

            ln -s ${oodle.help} oodle/help.html
            ln -s ${oodle.src} oodle/src
          '';
        };
      }
    );
}
