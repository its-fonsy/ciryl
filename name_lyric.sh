#!/bin/sh

# Rename and copy lyric using cmus metadata to generate the MD5 digest

# Get song metadata
artist="$(cmus-remote -Q | grep artist | head -n 1 | cut -c 12-)"
title="$(cmus-remote -Q | grep title | head -n 1 | cut -c 11-)"

# Compute digest
digest="$(echo -n "$artist$title" | md5sum -z | cut -d " " -f 1)"

# Select lyric file using fzf
lyric=$(find . -name "*.lrc" -type f | fzf)

# Ask user to confirm renaming
echo " Lyric: $lyric"
echo "Digest: $digest"
echo "  Dest: $LYRICS_DIR/$digest.lrc"
read -p "Confirm? (Y/n) " confirm

if [[ $confirm == [yY] ]] || [[ -z "$confirm" ]]; then
    cp $lyric $LYRICS_DIR/$digest.lrc
    echo "Success"
else
    echo "Aborted"
fi

exit 0
