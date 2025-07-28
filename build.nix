{
  lib,
  rustPlatform,
  rust,
  buildInputs,
  nativeBuildInputs,
  package_version,
  ...
}:

let
  pname = "stoicquotes";
  version = package_version;

  buildRustPackage = rustPlatform.buildRustPackage.override {
    rustc = rust;
    cargo = rust;
  };

in
buildRustPackage {
  inherit pname version;

  doCheck = false;

  src = ./.;

  cargoLock = {
    lockFile = ./Cargo.lock;
  };

  inherit buildInputs nativeBuildInputs;

  # Override the Rust compiler used
  rustc = "${rust}/bin/rustc";
  cargo = "${rust}/bin/cargo";

  meta = with lib; {
    description = "Stoic Quotes";
    homepage = "https://github.com/storopoli/stoic-quotes";
    license = licenses.mit;
    maintainers = [ maintainers.storopoli ];
    platforms = platforms.all;
  };
}
