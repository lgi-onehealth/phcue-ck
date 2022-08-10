
To install `phcue-ck`, go to the [release page](https://github.com/lgi-onehealth/phcue-ck/releases) and download the appropriate `phcue-ck` binary for your system.

### OSX

Download the `phcue-ck_v<version>_x86_64-apple-darwin.zip` file and unzip it. Move the `phcue-ck` binary to your `PATH` directory.

### Linux

Download the `phcue-ck_v<version>_x86_64-unknown-linux-musl.tar.gz` file and untar it. Move the `phcue-ck` binary to your `PATH` directory.

### Windows

Download the `phcue-ck_v<version>_x86_64-pc-windows-gnu.zip` file and unzip it. Move the `phcue-ck.exe` binary to your `PATH` directory.

### Installation with cargo

If you have `cargo` installed and want to install `phcue-ck` using `cargo`, you can do so by running:

```bash
cargo install phcue-ck
```

This will add the `phcue-ck` binary to `$HOME/.cargo/bin`. Make sure to add that folder to your `PATH` environment variable. For instance, in `bash` shell, you can run the following:

```bash
echo 'export PATH="$HOME/.cargo/bin:$PATH"' >> $HOME/.bashrc
```

## Using with Docker

You can run `phcue-ck` inside a Docker container by running:

```bash
docker run -it --rm -v $PWD:/app lighthousegenomics/phcue-ck:latest phcue-ck --help
```
