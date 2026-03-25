{
      inputs = {
            nixpkgs.url = "github:NixOs/nixpkgs/nixos-unstable";
            utils.url   = "github:numtide/flake-utils";
      };

      outputs = {self, nixpkgs, utils }:
            utils.lib.eachDefaultSystem (system: 
                  let 
                        pkgs = import nixpkgs { inherit system; }; 
                  in
                  {
                        packages.default = pkgs.rustPlatform.buildRustPackage 
                        {
                              pname       = "what-todo";
                              version     = "0.1.0";
                              src         = ./.;
                              cargoHash   = "sha256-vh6UbgzOi1xeDBeJwcZiY7VyYd5mEDYwHo3E0TBwLho=";
                        };
                  });
  
}
