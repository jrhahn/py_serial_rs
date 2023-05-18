with import <nixpkgs> {};
stdenv.mkDerivation rec {
      LIBCLANG_PATH = "${llvmPackages.libclang}/lib"; 
      name = "OpenEthereum";
      src = null;
      buildInputs = [ 
          clang 
          llvmPackages_13.llvm
          maturin
          openssl 
          pkgconfig 
          rustup 
          rust-analyzer 
          udev 
#          python-language-server  
      ];
  }

