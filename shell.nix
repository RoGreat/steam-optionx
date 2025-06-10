{
  pkgs ? import <nixpkgs> { },
}:

pkgs.mkShell {
  nativeBuildInputs = with pkgs; [
    cargo
    pkg-config
    rustc
  ];

  buildInputs = with pkgs; [
    openssl
    qt6.full
  ];

  LD_LIBRARY_PATH =
    with pkgs;
    lib.makeLibraryPath [
      ffmpeg
      fontconfig
      libglvnd
      libxkbcommon
      skia
      wayland
      xorg.libxcb
    ];
}
