#!/bin/bash
set -e

function add_dependency() {
  sed -i '' "/dependencies/a\\
$2
" "$1" 
}

function print_help_and_exit() {
  cat << EOF
usage: $0 [--nom] <day name>
EOF
  exit 1
}

# Simple arg parsing: https://stackoverflow.com/a/14203146
POSITIONAL=()
while [[ $# -gt 0 ]]; do
  key="$1"

  case $key in
    --nom)
      ADD_NOM=true
      shift
      ;;
    -h|--help)
      print_help_and_exit
      ;;
    *)    # unknown option
      POSITIONAL+=("$1") # save it in an array for later
      shift # past argument
      ;;
  esac
done

DAY_NAME="${POSITIONAL[0]}"
if [ -z "$DAY_NAME" ]; then
  print_help_and_exit
fi

cargo new --vcs none $DAY_NAME
cp day_n_template.rs $DAY_NAME/src/main.rs

if [ "$ADD_NOM" = true ]; then
  add_dependency $DAY_NAME/Cargo.toml 'nom = "7.1.0"'
fi

