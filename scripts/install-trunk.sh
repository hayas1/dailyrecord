#! /bin/sh

if ! (type trunk >/dev/null 2>&1); then
    printf ">>> install trunk? [y/N]"
    read -r ans
    case $ans in
    [Yy]*)
        cargo install trunk
        ;;
    *)
        echo "!!! do not install trunk, finish"
        exit 0
        ;;
    esac
fi
