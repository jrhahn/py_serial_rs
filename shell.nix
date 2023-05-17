with import <nixpkgs> {};
stdenv.mkDerivation rec {
      LIBCLANG_PATH = "${llvmPackages.libclang}/lib"; 
      name = "OpenEthereum";
      src = null;
      buildInputs = [ rustup rust-analyzer openssl pkgconfig udev clang maturin];
  }

