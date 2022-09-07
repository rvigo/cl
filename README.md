# cl

cl (**C**ommand **L**ist) is a way to group all your `aliases`, `once in a while` or `multiple usages with a lot of args` commands in an organized and human readable place

## how to install

### macOs (intel and silicon) and Linux using `homebrew`:

``` bash
$ brew tap rvigo/cl
$ brew install cl 
```

### from source:
You must have `Rust` and `Cargo` installed ([please follow the official Rust site instructions](https://www.rust-lang.org/tools/install))

  ``` bash
  # this will download the lastest tag 
  $ git clone --depth 1 -b $(git describe --tags --abbrev=0) git@github.com:rvigo/cl.git
  # and then install the application
  $ cargo install --path .
  ```

Please do not build from `HEAD`, you may face incomplete work/broken commits

## usage

Using the application interface to add, edit, and run commands:
```bash
$ cl
```

Using the application's CLI to executed stored commands:
```bash
$ cl exec <your command alias>
```
You can pass args and flags to the stored command:
```bash
# the stored command is `docker` and the alias is `d`
$ cl exec d ps # same as `docker ps` 
# flags need to be escaped with \ and surrounded by quotes
$ cl exec d ps '\--help' # same as `docker ps --help` 
```

You can also set `variables` in your command and pass them as `named arguments`:
```bash
# the stored command is `echo "hello #{name}, #{greetings}` and the alias is `echo`
# the arguments names should match the variables names in your command
$ cl exec echo -- --name="John Doe" --grettings "welcome!" 
> "hello John Doe, welcome!"
```

## disclaimer
This project is in early development stage, so if you find some weird behavior, please open an `issue` describing what is happening. Contributions are also welcome
