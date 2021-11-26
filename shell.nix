with import <nixpkgs> {};
mkShell {
  buildInputs = [ cargo rustc rust-analyzer rustfmt ];
}
