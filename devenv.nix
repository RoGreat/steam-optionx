{
  pkgs,
  lib,
  ...
}:

rec {
  env = {
    LD_LIBRARY_PATH = lib.makeLibraryPath packages;
  };

  packages = with pkgs; [
    libglvnd # libGL.so libEGL.so
    libx11 # libX11.so libX11-xcb.so
    libxcb # libxcb
    libxcursor # libXcursor.so
    libxi # libXi.so
    libxinerama # libXinerama.so
    libxkbcommon # libxkbcommon.so libxkbcommon-x11.so
    libxrender # libXrender.so
    libxt # libXt.so
    vulkan-loader # libvulkan.so
    wayland # libwayland-egl.so libwayland-client.so
  ];

  languages.rust = {
    enable = true;
    channel = "nightly";
    components = [
      "rustc"
      "cargo"
      "clippy"
      "rustfmt"
      "rust-analyzer"
    ];
  };
}
