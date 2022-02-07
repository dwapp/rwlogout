{pkgs ? import <nixpkgs> {}}:
pkgs.mkShell {
  nativeBuildInputs = [ pkgs.cargo ];
  buildInputs = with pkgs; [
    glib
    pkg-config
    zlib
    pango
    gdk-pixbuf
    gtk4

    rustc
    rustup
    rust-analyzer
  ];
}
