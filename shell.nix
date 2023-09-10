{ pkgs ? import <nixpkgs> { } }:
pkgs.mkShell
{
  nativeBuildInputs = with pkgs.buildPackages;
    [
      curl
      git
      tree
      unzip
      zstd
      iproute2
      jq
      clang
      llvm
      lldb
      glibc
      rustup
      musl
      cocogitto
      difftastic
      watchexec
      hexyl
      zellij
      vscodium
      #nixd
      nixpkgs-fmt
    ];
  shellHook = ''
    rustup default stable
    rustup component add rust-analyzer
    rustup component add rustfmt
    rustup component add clippy
    rustup target add x86_64-unknown-linux-musl 
    codium .
  '';
}
