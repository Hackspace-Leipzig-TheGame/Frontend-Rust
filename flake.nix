{
  inputs = {
    nixpkgs = {};
    fu.url = "github:gytis-ivaskevicius/flake-utils-plus";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  nixConfig = {
    extra-substituters = ["https://nix-community.cachix.org"];
    extra-trusted-public-keys = ["nix-community.cachix.org-1:mB9FSh9qf2dCimDSUo8Zy7bkq5CX+/rkCWyvRCYg3Fs="];
  };

  outputs = {
    self,
    fu,
    fenix,
    ...
  } @ inputs: let
    # === Continuesly build the frontend ===
    watchFrontend = pkgs:
      pkgs.writeShellApplication {
        name = "watch-frontend";
        runtimeInputs = with pkgs; [trunk];
        text = ''
          cd frontend
          npm install
          trunk watch
        '';
      };
    # === Run the backend ===
    runBackend = pkgs:
      pkgs.writeShellApplication {
        name = "run";
        text = ''
          cargo run -p backend
        '';
      };
  in
    fu.lib.mkFlake {
      inherit self inputs;
      supportedSystems = ["x86_64-linux"];

      sharedOverlays = [fenix.overlay];

      outputsBuilder = channels: {
        devShell = channels.nixpkgs.mkShell {
          name = "devShell";
          buildInputs = with channels.nixpkgs; [
            openssl
            pkg-config
            nodejs
            trunk
            (watchFrontend pkgs)
            (runBackend pkgs)
            (with fenix.packages.x86_64-linux;
              combine [
                minimal.rustc
                minimal.cargo
                targets.wasm32-unknown-unknown.latest.rust-std
              ])
          ];
        };
      };
    };
}
