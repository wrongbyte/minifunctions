{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
    rust-overlay.inputs.nixpkgs.follows = "nixpkgs";
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay }:
    flake-utils.lib.eachDefaultSystem (system:
    let
      overlays = {
        rust-bin = import rust-overlay;
      };
      pkgs = import nixpkgs {
        inherit system;
        overlays = builtins.attrValues overlays; 
      };
      rustToolchain = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
      postgresql = pkgs.postgresql_15;
      jq = pkgs.jq;
    in 
    with pkgs; {
      devShells.default = pkgs.mkShell {
        buildInputs = [
          rustToolchain
          postgresql
          jq
        ];
      # TODO: run `initdb` if no PGDATA (or not pg_isready)
        shellHook = ''
          export PGHOST="$PWD/.postgresql"
          export PGDATA="$PGHOST/data"
          export PGDATABASE=local-postgres
          export PGUSER=$(whoami)
          export PGPASSWORD=postgres
          export LOG_PATH="$PGHOST/log"
          export DATABASE_URL="postgres://$PGUSER:$PGPASSWORD@$(echo $PGHOST | jq -Rr '@uri')/$PGDATABASE"
          
          if [ -f ./database/init.sh ]; then
            chmod +x ./database/init.sh
            ./database/init.sh
          else
            echo "Warning: init.sh not found!"
          fi

          printf "\nEnvironment is set up! ٩(◕‿◕｡)۶\n\n"
        '';
      };
    }
  );
}