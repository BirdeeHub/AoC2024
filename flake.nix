{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixpkgs-unstable";
  };
  outputs = { nixpkgs, ... }@inputs: let
    forEachSystem = with builtins; f: let # flake-utils.lib.eachSystem
      op = attrs: system: let
        ret = f system;
        op = attrs: key: attrs // {
          ${key} = (attrs.${key} or { })
          // { ${system} = ret.${key}; };
        };
      in foldl' op attrs (attrNames ret);
    in foldl' op { } nixpkgs.lib.platforms.all;

    days = nixpkgs.lib.filterAttrs (n: v: v == "directory") (builtins.readDir ./.);

    appOverlay = final: prev: builtins.mapAttrs (n: v: {
      ${n} = final.callPackage ./. ({ APPNAME = n; src = ./${n}; } // inputs);
    }) days;
  in {
    overlays.default = appOverlay;
  } // (
    forEachSystem (system: let
      pkgs = import nixpkgs { inherit system; overlays = [ appOverlay ]; };
      allset = builtins.mapAttrs (n: _: pkgs.${n}) days;
      paths = with builtins; concatLists (map attrValues (map (n: pkgs.${n}) (attrNames days)));
      binpaths = builtins.map (v: "${v}/bin/${pkgs.lib.getName v}") paths;
      all = pkgs.symlinkJoin {
        name = "all-solutions";
        inherit paths;
        postBuild = ''
          mkdir -p $out/bin;
          cat > $out/bin/run_all <<EOFTAG
          for script in ${pkgs.lib.escapeShellArgs binpaths}; do
            echo;
            echo "\$script";
            bash -c "\$script";
          done
          EOFTAG
          chmod +x $out/bin/run_all
        '';
      };
    in{
      packages = { default = all; } // allset;
    })
  );
}
