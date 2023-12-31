{ pkgs ? import <nixpkgs> { } }:

# This file contains a development shell for working on rustc.
let
  # Build configuration for rust-lang/rust. Based on `config.example.toml` (then called
  # `config.toml.example`) from `1bd30ce2aac40c7698aa4a1b9520aa649ff2d1c5`
  config = pkgs.writeText "rustc-config" ''
    profile = "compiler" # you may want to choose a different profile, like `library` or `tools`

    [build]
    patch-binaries-for-nix = true
    # The path to (or name of) the GDB executable to use. This is only used for
    # executing the debuginfo test suite.
    gdb = "${pkgs.gdb}/bin/gdb"
    python = "${pkgs.python3Full}/bin/python"

    [rust]
    debug = true
    incremental = true
    deny-warnings = false

    # Indicates whether some LLVM tools, like llvm-objdump, will be made available in the
    # sysroot.
    llvm-tools = true

    # Print backtrace on internal compiler errors during bootstrap
    backtrace-on-ice = true
  '';

  ripgrepConfig =
    let
      # Files that are ignored by ripgrep when searching.
      ignoreFile = pkgs.writeText "rustc-rgignore" ''
        configure
        config.example.toml
        x.py
        LICENSE-MIT
        LICENSE-APACHE
        COPYRIGHT
        **/*.txt
        **/*.toml
        **/*.yml
        **/*.nix
        *.md
        src/ci
        src/etc/
        src/llvm-emscripten/
        src/llvm-project/
        src/rtstartup/
        src/rustllvm/
        src/stdsimd/
        src/tools/rls/rls-analysis/test_data/
      '';
    in
    pkgs.writeText "rustc-ripgreprc" "--ignore-file=${ignoreFile}";
in
pkgs.mkShell {
  name = "rustc";
  nativeBuildInputs = with pkgs; [
    gcc_multi
    binutils
    cmake
    ninja
    openssl
    pkgconfig
    python39
    git
    curl
    cacert
    patchelf
    nix
    psutils
    rustup
  ];
  RIPGREP_CONFIG_PATH = ripgrepConfig;
  RUST_BOOTSTRAP_CONFIG = config;
}
