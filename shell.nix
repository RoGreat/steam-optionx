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
      fontconfig
      libglvnd
      libxkbcommon
      wayland
    ];

  RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
}
