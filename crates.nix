{...}: {
  perSystem = {
    pkgs,
    config,
    ...
  }: let
    crateName = "hexcat";
  in {
    # declare projects
    nci.projects.${crateName}.path = ./.;
    # configure crates
    nci.crates.${crateName} = {};
  };
}
