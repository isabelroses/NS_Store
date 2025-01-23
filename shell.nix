{
  mkShell,
  callPackage,

  clippy,
  rustfmt,
  rust-analyzer,
  tailwindcss,
}:
let
  mainPkg = callPackage ./default.nix { };
in
mkShell {
  inputsFrom = [ mainPkg ];

  packages = [
    clippy
    rustfmt
    rust-analyzer
    tailwindcss
  ];
}
