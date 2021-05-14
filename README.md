```sh
❯ templatehoshii --help
templatehoshii 0.2.0
moajo <mimirosiasd@gmail.com>

USAGE:
    templatehoshii [FLAGS] [OPTIONS] [template]

FLAGS:
    -h, --help       Prints help information
    -l, --list       list all templates. * means template has only 1 file, and it dumps to stdout as default.
        --rm         template name to remove
    -V, --version    Prints version information

OPTIONS:
    -a, --add <add>    new template name to add

ARGS:
    <template>    target template name
```

# installation

```sh
cargo install --git https://github.com/moajo/templatehoshii
```

# usage

### simple dump

```sh
❯ cat ./Makefile
.PHONY: hello
hello:
        echo "this is Makefile template"

❯ templatehoshii --add make Makefile
👈  Makefile

❯ templatehoshii --list
*make

❯ templatehoshii spec
👉 Makefile

❯ templatehoshii rm make
Template 'make' is removed.
```

### select item interactively

```sh
❯ templatehoshii --add hoge Makefile
👈  Makefile
❯ templatehoshii --add hoge2 Makefile
👈  Makefile
❯ templatehoshii --add hoge3 Makefile
👈  Makefile

❯ templatehoshii
? Select template to dump? ›
❯ *hoge
  *hoge2
  *hoge3
```

### add directory

```sh
❯ ls docker
Dockerfile       requirements.txt

❯ templatehoshii --add docker docker
👈  docker/requirements.txt
👈  docker/Dockerfile

❯ templatehoshii docker
👉 docker/requirements.txt
👉 docker/Dockerfile
```
