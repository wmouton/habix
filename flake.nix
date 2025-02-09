{
  description = "Rust project with OpenSSL";
  inputs.nixpkgs.url = "github:NixOS/nixpkgs";

  outputs = { self, nixpkgs }: 
    let
      pkgs = nixpkgs.legacyPackages.x86_64-linux;
    in {
      devShells.default = pkgs.mkShell {
        buildInputs = [ pkgs.openssl pkgs.pkg-config ];
      };
    };
}
