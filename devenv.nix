{ pkgs, lib, config, inputs, ... }:

{
  languages.rust.enable = true;
  packages = [
    pkgs.aoc-cli
    pkgs.cargo-generate
    pkgs.cargo-watch
    pkgs.cargo-outdated
    pkgs.hyperfine
    pkgs.z3
    pkgs.clang
    pkgs.llvmPackages.libclang
  ];
  env.LIBCLANG_PATH = "${pkgs.llvmPackages.libclang.lib}/lib";
  env.LD_LIBRARY_PATH = "${pkgs.stdenv.cc.cc.lib}/lib";
}
