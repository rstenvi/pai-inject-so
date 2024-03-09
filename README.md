# pai-inject-so

A tool to inject shared object (SO) files into processes created using [pai](https://github.com/rstenvi/pai)

## Install

~~~{.bash}
cargo install --force pai-inject-so
~~~

## Compile

[cargo-make](https://github.com/sagiegurari/cargo-make) is used to control the
build process. [cross](https://github.com/cross-rs/cross) is used to support
cross-compilation. To simplify the build process, `cross` is used even when
compiling for host target.

The command to build for host target is:

~~~{.bash}
cargo make [build|release] [target]
~~~

The output will be placed in `output/<target>/<debug|release>/pai-inject-so`.

### Cross-compile

Cross-compilation is sometimes as easy as described above, like this example for
Android:

~~~{.bash}
$ cargo make release aarch64-linux-android
$ ls output/aarch64-linux-android/release/pai-inject-so
output/aarch64-linux-android/release/pai-inject-so
~~~

Not all targets are supported in `cross` in those cases, we need to find an
appropriate linker.

## Examples

[testdata/](testdata/) contains some example code to test on. Below is an
example to load a shared object file which overrides the `puts` function call.

### Spawn program

~~~
$ make -C testdata/
$ cargo run -- -i testdata/sofile.so -o puts testdata/demo
constructor was called
prog wrote: Hello World!
~~~

The result is almost the same as using `LD_PRELOAD`. If you try the same using `LD_PRELOAD`, the output is slightly different:

~~~
LD_PRELOAD=testdata/sofile.so testdata/demo
prog wrote: constructor was called
prog wrote: Hello World!
~~~

`LD_PRELOAD`, like the name suggests, load the shared object before other
objects and therefore the hooks take effect immediately. We load the shared
object after the program has started and therefore the hook takes effect later.
The effect of this is minimal, but it means that we can preload on already
running programs.

### Attach program

For this to work, you need to have the appropriate permissions, fix with the
following commands:

~~~
cat /proc/sys/kernel/yama/ptrace_scope
echo 0 | sudo tee /proc/sys/kernel/yama/ptrace_scope
cat /proc/sys/kernel/yama/ptrace_scope
~~~

Then in one terminal start `demo2`, every second it will print:

~~~
$ ./testdata/demo2 
Hello World!
Hello World!
~~~

Then in a second window write:

~~~
cargo run -- -i testdata/sofile.so -o puts --attach demo2
~~~

The first window should now start printing:

~~~
constructor was called
prog wrote: Hello World!
prog wrote: Hello World!
...
~~~
