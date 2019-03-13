# Octave Pipe

Octave-Pipe is a simple online Octave/Matlab web service written in rust, which can be set up within few steps.

### Brief

With ~~full~~ octave support with highlighted editor, users can do complicated matrix calculations easily. Furthermore, I'm planning to add graph support & custom stdin.

A common instance of this web service was shown below.

![Website Preview](https://github.com/Phosphorus15/octave-pipe/blob/master/_preview.png?raw=true)

### Setup

To compile & setup for yourself, you'll need :

* A server/PC (`Linux` system is recommended)
* An Octave distribution
* Rust & Cargo compiler environment

You can get `GNU Octave` from Offical [Website](https://www.gnu.org/software/octave/), or install it using package manager

```bash
# Ubuntu
sudo apt install octave
# CentOS
sudo yum install octave
# MacOS
brew install octave
```

Then, make sure you have `cargo` installed (using [rustup](https://www.rust-lang.org/tools/install) is recommended)

Clone the repository to your server and compile it using `cargo`

```bash
git clone https://github.com/Phosphorus15/octave-pipe.git # clone from github
cd octave-pipe
cargo build --bin # compile
```

After successfully compiled, you can start the service at any time by simply typing

```bash
cargo run
```

under  the `octave-pipe` folder.

If the service goes up smoothly, you will see

```bash
Octave-pipe started
Bind on localhost:8080
```

on console screen, and the service is now accessible at `http://localhost:8080/`