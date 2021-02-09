```sh
❯ templatehoshii --help
templatehoshii 0.1.0
moajo <mimirosiasd@gmail.com>

USAGE:
    templatehoshii [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    add     add new template
    dump    dump template to stdout or file
    help    Prints this message or the help of the given subcommand(s)
    list    list all templates. * means template has only 1 file, and it dumps to stdout as default.
    rm      remove template
```

# usage

### simple dump

```sh
❯ cat ./Makefile
.PHONY: hello
hello:
        echo "this is Makefile template"

❯ templatehoshii add make Makefile
👈  Makefile

❯ templatehoshii ls
*make

❯ templatehoshii dump make
.PHONY: hello
hello:
        echo "this is Makefile template"

❯ templatehoshii rm make
Template 'make' is removed.
```

### dump as file

```sh
# same as "templatehoshii dump make > Makefile"
❯ templatehoshii dump make -f
```

### select item interactively

```sh
❯ templatehoshii add hoge Makefile
👈  Makefile
❯ templatehoshii add hoge2 Makefile
👈  Makefile
❯ templatehoshii add hoge3 Makefile
👈  Makefile

bash-3.2$ templatehoshii dump
? Select template to dump? ›
❯ *hoge
  *hoge2
  *hoge3
```

### add directory

```sh
❯ ls docker
Dockerfile       requirements.txt

❯ templatehoshii add docker docker
👈  docker/requirements.txt
👈  docker/Dockerfile

❯ templatehoshii dump docker
👉 docker/requirements.txt
👉 docker/Dockerfile
```

> NOTE: directory template will dump to directory has same name. (not stdout)
