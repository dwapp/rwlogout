rec {
  description = "Yet another logout menu";

  inputs = {
    flake-utils.url = "github:numtide/flake-utils";
    nixpkgs.url = "github:nixos/nixpkgs/nixpkgs-unstable";
  };

  outputs =
    {
      self,
      flake-utils,
      nixpkgs,
    }:
    let
      inherit (builtins) substring;
      inherit (nixpkgs) lib;

      mtime = self.lastModifiedDate;
      date = "${substring 0 4 mtime}-${substring 4 2 mtime}-${substring 6 2 mtime}";
      rev = self.rev or (lib.warn "Git changes are not committed" (self.dirtyRev or "dirty"));

      mkPackage =
        { pkgs }:
        pkgs.rustPlatform.buildRustPackage {
          pname = "wlogout2";
          version = "0-unstable-${date}";
          src = self;

          cargoLock = {
            lockFile = ./Cargo.lock;
            allowBuiltinFetchGit = false;
          };

          nativeBuildInputs = with pkgs; [
            pkg-config
          ];

          buildInputs = with pkgs; [
            glib
            zlib
            pango
            gdk-pixbuf
            gtk4
            gtk4-layer-shell
          ];

          CFG_RELEASE = "git-${rev}";

          meta = {
            homepage = "https://github.com/dwapp/wlogout2";
            license = with lib.licenses; [
              mit
            ];
            mainProgram = "wlogout2";
          };
        };
    in
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
      in
      rec {
        packages = rec {
          default = wlogout2;
          wlogout2 = pkgs.callPackage mkPackage { };
        };

        devShells.default = pkgs.mkShell {
          inputsFrom = [
            self.packages.${system}.default
          ];
          RUST_BACKTRACE = "short";
          RUST_SRC_PATH = "${pkgs.rustPlatform.rustLibSrc}";
          NIXPKGS = nixpkgs;
        };
      }
    )
    // {
      overlays = {
        default = final: prev: {
          wlogout2 = final.callPackage mkPackage { };
        };
      };
    };
}
