{ pkgs ? import <nixpkgs> {}}:

pkgs.mkShell {
  buildInputs = with pkgs; [
    rustc
    cmake
    git
    gcc
    xorg.libXext
    xorg.libXft
    xorg.libXinerama
    xorg.libXcursor
    xorg.libXrender
    xorg.libXfixes
    libcerf
    pango
    cairo
    libGL
    mesa
    pkg-config
  ];

  RUST_BACKTRACE = 1;
}

