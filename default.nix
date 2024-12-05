{ APPNAME
, rustPlatform
, src
, writeShellScriptBin
, ...
# override overrides these args
}: let
APPDRV = rustPlatform.buildRustPackage {
  pname = APPNAME;
  version = "0.0.0";
  src = src;

  cargoLock = {
    lockFileContents = builtins.readFile "${src}/Cargo.lock";
  };

};
in
writeShellScriptBin APPNAME ''
  export AOC_INPUT=./${APPNAME}/input;
  exec ${APPDRV}/bin/${APPNAME}
''
