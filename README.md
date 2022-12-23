# RainCloud 

A simple HTTP file server that serves from an git repo. 
Developed to quckly spin up a cloud-init data source.

Configured using `config.toml` 

# Configuration

Can be configured via flags or a `config.toml` file. 
The config file is expected to be in the same directory as 
that the command is run from, or overriden using the `--config` flag.

All toml options can also be set via flags. I.e. `--foo-bar=baz` 
is the same as: 

```toml
[foo]
bar = baz
```

## `[repo]`

### `remote`
**Required** 
The remote git clone url 
 
### `branch` 
Optional.

### `tag`
Optional.

### `directory`
The directory to serve. 
Defaults to the top level of the git repo.


## `[update]`

### `frequency`
A cron expression, or the literal "Never". 
No value defaults to "Never"

## `[server]`

### `port`
Pretty self explanitory. 
Defaults to `4750`. 

### `bind-address`
Defaults to `0.0.0.0`


## Example Config

```toml
[repo]
remote = "https://github.com/KGB33/raincloud.git"
# branch = "main"
# tag = "v1.2.3"

[update]
frequency = "0 22 * * 1-5" # https://crontab.guru/#0_22_*_*_1-5

[server]
port = 4750
bind_address = "0.0.0.0"
```
