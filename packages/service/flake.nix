{
  description = "rmcs-actions-service dev environment";
  outputs =
    { self, nixpkgs }:
    let
      system = "x86_64-linux";
      pkgs = import nixpkgs { inherit system; };
    in
    {
      devShells.${system}.default = pkgs.mkShell {
        buildInputs = with pkgs; [
          sqlite
        ];

        shellHook = ''
          export DATABASE_URL="sqlite://$PWD/runtime/storage/rmcs-actions.db"
          db_path="$PWD/runtime/storage/rmcs-actions.db"
          mkdir -p "$(dirname "$db_path")"
          sqlite3 "$db_path" < ${./schema.sql}
        '';
      };
    };
}
