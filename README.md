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
  $ git clone --depth 1 -b v0.3.0 git@github.com:rvigo/cl.git
  # and then install the application
  $ cargo install --path .
  ```

## usage

Using the application interface to add, edit, and run commands:
```bash
$ cl
```

Using the application's CLI to executed stored commands:
```bash
$ cl X <your command alias>
```

You can also set `variables` in your command and pass them as `named arguments`:
```bash
# the stored command is `echo "hello #{name}, #{greetings}` and the alias is `echo`
# the arguments names should match the variables names in your command
$ cl X echo -- --name="John Doe" --grettings "welcome!" 
> "hello John Doe, welcome!"
```

## disclaimer
This project is in early development stage, so if you find some weird behavior, please open an `issue` describing what is happening. Contributions are also welcome
