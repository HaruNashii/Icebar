{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  packages = [
    pkgs.nerd-fonts.jetbrains-mono
    pkgs.nerd-fonts.caskaydia-cove   
    pkgs.nerd-fonts.fira-code

    pkgs.rustc
    pkgs.cargo
    pkgs.clippy
    pkgs.rust-analyzer

    pkgs.pkg-config
    pkgs.cmake

    pkgs.openssl
    pkgs.fontconfig

    pkgs.libpulseaudio
    pkgs.wayland
    pkgs.wayland-protocols
    pkgs.xorg.libxcb
    pkgs.libxkbcommon

    pkgs.libGL
    pkgs.mesa

    pkgs.neovim
    pkgs.ripgrep
  ];

shellHook = ''
  export LIB_PATHS=${pkgs.lib.makeLibraryPath [
    pkgs.wayland
    pkgs.libpulseaudio
    pkgs.libxkbcommon
    pkgs.xorg.libxcb
    pkgs.libGL
    pkgs.mesa
  ]}

  export FONTCONFIG_FILE=${pkgs.makeFontsConf {
    fontDirectories = [
      "${pkgs.nerd-fonts.jetbrains-mono}/share/fonts"
      "${pkgs.nerd-fonts.caskaydia-cove}/share/fonts"
      "${pkgs.nerd-fonts.fira-code}/share/fonts"
    ];
  }}


  export LD_LIBRARY_PATH=$LIB_PATHS:$LD_LIBRARY_PATH
  export LIBRARY_PATH=$LIB_PATHS:$LIBRARY_PATH
'';
}
