{pkgs ? import <nixpkgs> {}}: let
  getLibFolder = pkg: "${pkg}/lib";
in
  pkgs.stdenv.mkDerivation {
    name = "bot";

    nativeBuildInputs = with pkgs; [
      # LLVM & GCC
      gcc
      cmake
      gnumake
      pkg-config
      llvmPackages.llvm
      llvmPackages.clang

      # Hail the Nix
      nixd
      nixpkgs-fmt
      nixpkgs-lint

      # Launch scripts
      just

      # Rust
      rustc
      cargo
      clippy
      rustfmt
      cargo-watch
      rust-analyzer
    ];

    buildInputs = with pkgs; [
      openssl
    ];

    # Set Environment Variables
    RUST_BACKTRACE = 1;
    NIX_LDFLAGS = "-L${(getLibFolder pkgs.libiconv)}";
    RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
    LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath [
      pkgs.gcc
      pkgs.libiconv
      pkgs.llvmPackages.llvm
    ];

    shellHook = ''
      # Load the environment variables from the .env file
      if [ ! -f .env ]; then
        echo "Please enter your telegram bot token: ";
        read -r TELOXIDE_TOKEN;
        echo "TELOXIDE_TOKEN=$TELOXIDE_TOKEN" > .env;
      else
        source .env;
      fi

      # Set the environment variable
      # export TELOXIDE_TOKEN=$TELOXIDE_TOKEN;

      # Start watching for changes
      # Start watching for changes in the background
      # cargo watch -x "run --bin bot" &

      # Store the PID of the background process
      # CARGO_WATCH_PID=$!

      # Function to clean up the background process on exit
      # cleanup() {
      #   kill $CARGO_WATCH_PID
      # }

      # Trap EXIT signal to run cleanup function
      # trap cleanup EXIT
    '';
  }
