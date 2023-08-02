{
  description = "My rust flake";


  outputs = { self, nixpkgs }:
    let pkgs = import nixpkgs{system = "x86_64-linux";};

    in {

    devShell.x86_64-linux = pkgs.mkShell {

      packages = [
        pkgs.cargo
        pkgs.rustc
        pkgs.rust-analyzer
        pkgs.rustfmt
      ];

    };

 };
}