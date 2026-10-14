{
  pkgs,
  lib,
  config,
  inputs,
  ...
}:

rec {
  # https://devenv.sh/basics/
  env = {
    LD_LIBRARY_PATH = lib.makeLibraryPath packages;
  };

  # https://devenv.sh/packages/
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

  # https://devenv.sh/languages/
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

  # https://devenv.sh/processes/
  # processes.dev.exec = "${lib.getExe pkgs.watchexec} -n -- ls -la";

  # https://devenv.sh/services/
  # services.postgres.enable = true;

  # https://devenv.sh/scripts/
  # scripts.hello.exec = ''
  #   echo hello from $GREET
  # '';

  # https://devenv.sh/basics/
  # enterShell = ''
  #   hello         # Run scripts directly
  #   git --version # Use packages
  # '';

  # https://devenv.sh/tasks/
  # tasks = {
  #   "myproj:setup".exec = "mytool build";
  #   "devenv:enterShell".after = [ "myproj:setup" ];
  # };

  # https://devenv.sh/tests/
  # enterTest = ''
  #   echo "Running tests"
  #   git --version | grep --color=auto "${pkgs.git.version}"
  # '';

  # https://devenv.sh/git-hooks/
  # git-hooks.hooks.shellcheck.enable = true;

  # See full reference at https://devenv.sh/reference/options/
}
