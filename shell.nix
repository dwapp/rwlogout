{pkgs ? import <nixos-unstable-small> {}}:
pkgs.mkShell {
  buildInputs = with pkgs; [
    glib.dev
    pkg-config
    zlib
    cargo
    pango
    gdk-pixbuf
    gtk4 
  ];
}
