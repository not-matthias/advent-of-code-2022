{ pkgs ? import <nixpkgs> {} }:
pkgs.mkShell {
    packages = with pkgs; [openssl.dev pkg-config];
    shellHook = ''
        # Install cargo-aoc if not already
        if ! cargo aoc --help > /dev/null 2>&1; then
            cargo install cargo-aoc
        fi
    '';
}