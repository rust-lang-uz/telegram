# For more, refer to:
# https://github.com/NixOS/nixpkgs/blob/master/doc/languages-frameworks/rust.section.md
{pkgs ? import <nixpkgs> {}}: let
  lib = pkgs.lib;
  getLibFolder = pkg: "${pkg}/lib";
  manifest = (pkgs.lib.importTOML ./Cargo.toml).package;
in
  pkgs.rustPlatform.buildRustPackage {
    pname = manifest.name;
    version = manifest.version;
    cargoLock.lockFile = ./Cargo.lock;
    src = pkgs.lib.cleanSource ./.;

    nativeBuildInputs = with pkgs; [
      # Rust
      rustc
      cargo

      # LLVM & GCC
      gcc
      cmake
      gnumake
      pkg-config
      llvmPackages.llvm
      llvmPackages.clang
    ];

    buildInputs = with pkgs; [
      openssl
      libressl
    ];

    LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath [
      pkgs.gcc
      pkgs.libiconv
      pkgs.llvmPackages.llvm
    ];

    # If you wanna get thorny
    # RUST_BACKTRACE = 1;
    NIX_LDFLAGS = "-L${(getLibFolder pkgs.libiconv)}";
    RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";

    meta = with lib; {
      homepage = manifest.homepage;
      description = manifest.description;
      license = with lib.licenses; [asl20 mit];
      platforms = with platforms; linux ++ darwin;
      maintainers = [lib.maintainers.orzklv];
    };
  }
