{
  lib,
  rustPlatform,
  tailwindcss,
}:
let
  toml = (lib.importTOML ./Cargo.toml).package;
in
rustPlatform.buildRustPackage {
  pname = "NS_Store";
  inherit (toml) version;

  src = lib.fileset.toSource {
    root = ./.;
    fileset = lib.fileset.intersection (lib.fileset.fromSource (lib.sources.cleanSource ./.)) (
      lib.fileset.unions [
        ./Cargo.toml
        ./Cargo.lock
        ./src
        ./templates
        ./static
        ./styles
      ]
    );
  };

  nativeBuildInputs = [ tailwindcss ];

  preBuild = ''
    pushd styles
    tailwindcss -i ./base.css -o ../styles.css
    popd
  '';

  cargoLock.lockFile = ./Cargo.lock;

  meta = {
    inherit (toml) homepage description;
    license = lib.licenses.mit;
    maintainers = with lib.maintainers; [ isabelroses ];
    mainPackage = "ns";
  };
}
