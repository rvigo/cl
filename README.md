# cl

cl (short for **C**ommand **L**ist) is a way to group all your `aliases`, `once in a while` or `multiple usages with a lot of args` commands in an organized and human readable place

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
If you use ZSH, an autocomplete script is installed with the application.

## usage

Using the application interface to add, edit, and run commands:
```bash
$ cl
```

![Overview Gif](.github/media/cl_overview.gif)

Using the application's CLI to execute stored commands:
```bash
$ cl exec <your command alias>
```

If you are using the ZSH shell and [fzf](https://github.com/junegunn/fzf), a widget can be installed with `cl config zsh-widget --install`. After that, you can call the `exec` function (with some cool autocomplete features) pressing CTRL+O  
  
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

Importing/exporting your aliases is possible using the `share` subcommand:
```bash
# the command bellow will generate the file `shared.toml` in the current working directory by default, with all aliases present in the `git` namespace as provided
$ cl share export --namespace git 
# the command bellow will import all aliases from a file in the specified location. 
# you can also choose which namespace will be imported
$ cl share import -f `path/to/shared.toml`
```

## disclaimer
This project is in early development stage, so if you find some weird behavior, please open an `issue` describing what is happening. Contributions are also welcome
