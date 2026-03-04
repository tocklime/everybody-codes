{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, fenix }@inputs:
    let
      pkgs = nixpkgs.legacyPackages."x86_64-linux";
      fenixLib = fenix.packages."x86_64-linux";
      rustToolchain = fenixLib.stable.withComponents [ "clippy" "rustc" "cargo" "rust-src" "rustfmt" "rust-analyzer" ];
      neovim = (pkgs.neovim.override {
        configure = {
          packages.myVimPackage = with pkgs.vimPlugins; {
            start = [
              (pkgs.vimPlugins.nvim-treesitter.withPlugins
                (p: [ p.rust p.lua p.toml ]))
              nvim-lspconfig
              nvim-cmp
              cmp-nvim-lsp
              telescope-nvim
              plenary-nvim
            ];
          };
          customRC = "  set runtimepath^=${./nvim}\n  set packpath^=${
                ./nvim
              }\n  lua require(\"dev\").setup()\n";
        };
      });
    in {
      devShells."x86_64-linux".default = pkgs.mkShell {
        buildInputs =
          [ rustToolchain pkgs.bacon neovim pkgs.openssl pkgs.pkg-config pkgs.sqlite pkgs.sqlitebrowser pkgs.google-cloud-sdk pkgs.cargo-udeps ];
        shellHook = ''
          export LD_LIBRARY_PATH=${pkgs.openssl.out}/lib:$LD_LIBRARY_PATH
        '';
      };

    };
}
