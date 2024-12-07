rec {
  description = "rew-down is a software to help wm users shut down";

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
          pname = "rew-down";
          version = "unstable-${date}";
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
          ];

          CFG_RELEASE = "git-${rev}";

          meta = {
            homepage = "https://github.com/wineee/rew-shutdown";
            license = with lib.licenses; [
              mit
            ];
            mainProgram = "rew-shutdown";
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
          default = rew-down;
          rew-down = pkgs.callPackage mkPackage { };
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
        default = self.overlays.rew-down;
        rew-down = final: prev: {
          rew-down = final.callPackage mkPackage { };
        };
      };
    };
}
