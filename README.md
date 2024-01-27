# kumash 🐻🍄

[![Actions Status](https://github.com/kumavale/kumash/workflows/Rust/badge.svg)](https://github.com/kumavale/kumash/actions)

`kumash` is a toy command line shell for Linux.

## Features

- execute

```
$ ls -a
.  ..  Cargo.lock  Cargo.toml  .git  .github  .gitignore  LICENSE-APACHE  LICENSE-MIT  README.md  src  target
```

- builtin

```
$ exit
(^-^)/~~
```

- pipe

```
$ cat src/main.rs | grep unwrap
        io::stdout().flush().unwrap();
                Ok(output) => io::stdout().write_all(&output.stdout).unwrap(),
                Stdio::from(child.stdout.unwrap())
                .unwrap();
                let last_token = input.split_whitespace().last().unwrap_or("");
                let re = Regex::new(&format!(r"\b{last_token}([\w.]*)\b")).unwrap();
                let output = Command::new("ls").output().unwrap();
        io::stdout().flush().unwrap();
```

- tab-completion

```
$ cat Car<\t>
Cargo.lock Cargo.toml
```

