## Cyril

Cyril is a TUI that display the lyric of the song currently playing on cmus.
It will search for lyric files inside `$LYRICS_DIR` environment variable.

#### Build and install

Clone the repo, build and install

```
git clone https://github.com/its-fonsy/ciryl
cd ciryl
make install
```

or build and install manually

```
cargo build -r
install -m 755 target/release/ciryl <install-path>
```

#### Lyric filename

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

The repository also provides the `name_lyric.sh` script that can be used to
speed up the process of naming lyrics.
