## Cyril

Cyril is a TUI that display the lyric of the song currently playing on cmus.
It will search for lyric files inside `$LYRICS_DIR` environment variable.

### Build and install

Clone the repo, build and install

```
git clone https://github.com/its-fonsy/ciryl
cd ciryl
make install
```

The default install directory is `$HOME/.local/bin`.
To build and install manually

```
cargo build -r
install -m 755 target/release/ciryl <install-path>
```

### Lyric filename

The lyric filename is the computed MD5 using song artist and title as input.
This is done to avoid strange for the filesystem.

More precisely the input is "\<artist\>\<title\>" and the file must be named
"\<digest\>.lrc".

In case that the application doesn't find the corresponding lyric file, it will
display artist, title and the expected digest.

*Example*

If the song playing on cmus is "One Step Closer" by "Linkin Park" then the
input string is "Linkin ParkOne Step Closer". Using `md5sum`, the digest will be

```
$ echo -n "Linkin ParkOne Step Closer" | md5sum
cc4cfe6eeafe0094b364876717b6f49f  -
```

thus, the application will look for `cc4cfe6eeafe0094b364876717b6f49f.lrc`.

#### Renaming script

The repository also provides `name_lyric.sh` script that can be used to speed
up the process of naming lyrics. The documentation is given via the "-h,
--help" flag

```
$ name_lyric.sh -h
Usage: name_lyric.sh [OPTIONS] LYRIC_FILE

Copy LYRIC_FILE in $LYRICS_DIR using cmus metadata to generate
the MD5 digest as filename.

OPTIONS
  -d, --digest        Print MD5 digest of the current playing song artist-title.
  -i, --info          Print information about destination, digest and input file.
  -n, --no-confirm    Copy without asking to confirm.
  -h, --help          Print this message.
```

To install it in `$HOME/.local/bin` use

```
make install-script
```

*Examples*

```
# Default usage
$ name_lyric.sh lyric.lrc

# Copy without asking to confirm
$ name_lyric.sh -n lyric.lrc

# Use fzf to select the lyric file from all files in current directory
$ name_lyric.sh $(fzf)

# Filter only ".lrc" file to choose with fzf
$ name_lyric.sh $(find . -name "*.lrc" -type f | fzf)

# Display the digest of the current playing song of cmus.
# Useful to double check the value displayed in ciryl.
$ name_lyric.sh -d
```
