{
  inputs = {
    # Use the github URL for real packages
    cargo2nix.url = "github:cargo2nix/cargo2nix/master";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
    rust-overlay.inputs.nixpkgs.follows = "nixpkgs";
    rust-overlay.inputs.flake-utils.follows = "flake-utils";
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
  };

  outputs = { self, nixpkgs, cargo2nix, flake-utils, rust-overlay, ... }:
    # Build the output set for each default system and map system sets into
    # attributes, resulting in paths such as:
    # nix build .#packages.x86_64-linux.<name>
    flake-utils.lib.eachDefaultSystem (system:

      # let-in expressions, very similar to Rust's let bindings.  These names
      # are used to express the output but not themselves paths in the output.
      let

        # create nixpkgs that contains rustBuilder from cargo2nix overlay
        pkgs = import nixpkgs {
          inherit system;
          overlays = [(import "${cargo2nix}/overlay")
                      rust-overlay.overlay];
        };

        # create the workspace & dependencies package set
        rustPkgs = pkgs.rustBuilder.makePackageSet' {
          rustChannel = "1.56.1";
          packageFun = import ./Cargo.nix;

          # Use the existing all list of overrides and append your override
          packageOverrides = pkgs: pkgs.rustBuilder.overrides.all ++ [
            # parentheses disambiguate each makeOverride call as a single list element
            (pkgs.rustBuilder.rustLib.makeOverride {
              name = "glib-sys";
              overrideAttrs = drv: {
                propagatedNativeBuildInputs = drv.propagatedNativeBuildInputs or [ ] ++ [
                  pkgs.glib.dev
                ];
              };
            })

            (pkgs.rustBuilder.rustLib.makeOverride {
              name = "cairo-sys-rs";
              overrideAttrs = drv: {
                propagatedNativeBuildInputs = drv.propagatedNativeBuildInputs or [ ] ++ [
                  pkgs.cairo.dev
                ];
              };
            })

            (pkgs.rustBuilder.rustLib.makeOverride {
              name = "graphene-sys";
              overrideAttrs = drv: {
                propagatedNativeBuildInputs = drv.propagatedNativeBuildInputs or [ ] ++ [
                  pkgs.graphene
                ];
              };
            })

            (pkgs.rustBuilder.rustLib.makeOverride {
              name = "pango-sys";
              overrideAttrs = drv: {
                propagatedNativeBuildInputs = drv.propagatedNativeBuildInputs or [ ] ++ [
                  pkgs.pango.dev
                ];
              };
            })

            (pkgs.rustBuilder.rustLib.makeOverride {
              name = "gdk-pixbuf-sys";
              overrideAttrs = drv: {
                propagatedNativeBuildInputs = drv.propagatedNativeBuildInputs or [ ] ++ [
                  pkgs.gdk_pixbuf.dev
                ];
              };
            })

            (pkgs.rustBuilder.rustLib.makeOverride {
              name = "gdk4-sys";
              overrideAttrs = drv: {
                propagatedNativeBuildInputs = drv.propagatedNativeBuildInputs or [ ] ++ [
                  pkgs.gtk4.dev
                ];
              };
            })
            
          ];
        };

      in rec {
        # this is the output (recursive) set (expressed for each system)

        devShell = import ./shell.nix { inherit pkgs; };
        # the packages in `nix build .#packages.<system>.<name>`
        packages = {
          # nix build .#hello-world
          # nix build .#packages.x86_64-linux.hello-world
          rew-shutdown = (rustPkgs.workspace.rew-down {}).bin;
        };

        # nix build
        defaultPackage = packages.rew-shutdown;
      }
    );
}
