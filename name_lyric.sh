#!/bin/sh

# Copy LYRIC_FILE using cmus metadata to generate the MD5 digest as filename.

set -e

# Ensure $LYRICS_DIR is set
if [ -z "$LYRICS_DIR" ]; then
    echo "Error: \$LYRICS_DIR is not set."
    exit 1
fi

# Set variables
artist="$(cmus-remote -Q | grep artist | head -n 1 | cut -c 12-)"
title="$(cmus-remote -Q | grep title | head -n 1 | cut -c 11-)"
digest="$(echo -n "$artist$title" | md5sum -z | cut -d " " -f 1)"
lyric_file=""
out_dir="${LYRICS_DIR%/}"

# CLI default configuration
PRINT_ONLY_INFO=false
NO_CONFIRM=false

usage() {
    echo "Usage: $(basename $0) [OPTIONS] LYRIC_FILE"
    echo ""
    echo "Copy LYRIC_FILE in \$LYRICS_DIR using cmus metadata to generate"
    echo "the MD5 digest as filename."
    echo ""
    echo "OPTIONS"
    echo "  -d, --digest        Print MD5 digest of the current playing song artist-title."
    echo "  -i, --info          Print information about destination, digest and input file."
    echo "  -n, --no-confirm    Copy without asking to confirm."
    echo "  -h, --help          Print this message."
}

info() {
    echo "  Input file: $(basename "$1")"
    echo "      Digest: $digest"
    echo " Destination: $out_dir/$digest.lrc"
}

rename_lyric() {
    if [[ -z "$1" ]]; then
        echo "Error: LYRIC_FILE argument is required."
        exit 1
    fi

    info $1

    if $NO_CONFIRM; then
        cp $1 $out_dir/$digest.lrc
        echo "Copied successfully"
    else
        read -p "Confirm? (Y/n) " confirm
        if [[ $confirm == [yY] ]] || [[ -z "$confirm" ]]; then
            cp $1 $out_dir/$digest.lrc
            echo "Copied successfully"
        else
            echo "Abort"
        fi
    fi
}

# Handle arguments
while [[ "$#" -gt 0 ]]; do
    case "$1" in
        -d|--digest) echo "$digest"; exit 0 ;;
        -i|--info) PRINT_ONLY_INFO=true ;;
        -n|--no-confirm) NO_CONFIRM=true ;;
        -h|--help) usage; exit 0 ;;
        -*)
            echo "Error: Unknown OPTION $1" >&2
            exit 1
            ;;
        *) lyric_file="$1" ;;
    esac
    shift
done

if $PRINT_ONLY_INFO; then
    info $lyric_file
else
    rename_lyric $lyric_file
fi

exit 0
