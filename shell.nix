{pkgs ? import <nixpkgs> {}}:
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
