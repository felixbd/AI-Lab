# ai lab nix env - GUI for annotating, training, and evaluating AI models, simplifying workflows
# Copyright (C) 2024 - Felix Drees - GNU General Public License v3.0

{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  buildInputs = with pkgs; [
    # (python310.withPackages(ps: with ps; [
    #    scikit-learn
    # ]))
    cargo
    gtk4
    buildPackages.stdenv
  ];
  shellHook = ''
    echo "Start developing ..."
  '';
}
