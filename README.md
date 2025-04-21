<p align="center">
    <img src=".github/assets/header.png" alt="Rust Uzbekistan's {Telegram}">
</p>

<p align="center">
    <h3 align="center">Rustacean Telegram Assistant.</h3>
</p>

<p align="center">
    <a href="https://t.me/rustlanguz"><img align="center" src="https://img.shields.io/badge/Chat-grey?style=flat&logo=telegram&logoColor=383636&labelColor=dea584&color=dea584" alt="Telegram Chat"></a>
    <a href="https://github.com/rust-lang-uz/telegram/actions/workflows/test.yml"><img align="center" src="https://img.shields.io/github/actions/workflow/status/rust-lang-uz/telegram/test.yml?style=flat&logo=github&logoColor=383636&labelColor=dea584&color=dea584" alt="Test CI"></a>
</p>

## About

This is an assistant telegram bot for Rust Uzbekistan communtiy. It helps community members with various kind of tasks which you can check out in [features](#features) section!

> Also, this telegram bot is a complience for [Floss Uzbekistan](https://std.floss.uz)'s standartization.

## Features

These are features that was implemented within telegram bot. Features that are underdevelopment are marked `WIP`.

- Greet newcomers of the community (it's a trigger)
- Delete messages from Channel users (it's a trigger)
- Serve rules list for members (via `/rules`)
- Show other rust communities (via `/group`)
- Roadmap for newbies (via `/roadmap`)
- Share with more Rusty useful resources (via `/useful`)
- Get latest version of Rust toolchain (via `/latest`)
- List all published releases of Rust toolchain (via `/version`)
- Warn user about off-topic message and redirect (via `/warn`)

## Development

The project has `shell.nix` which has development environment preconfigured already for you. Just open your
terminal and at the root of this project:

```bash
# Open in bash by default
nix develop

# If you want other shell
nix develop -c $SHELL

# Upon entering development environment for the first
# time, you'll be asked for your development telegram
# bot token, it will be written to .env file for more
# convenient dev env startups. Token is saved at .env
# file at the root of this project. You can change it
# whenever you want!

# After entering development environment, inside the
# env, you can open your editor, so your editor will
# read all $PATH and environmental variables, also
# your terminal inside your editor will adopt all
# variables, so, you can close terminal.

# Neovim
vim .

# VSCode
code .

# Zed Editor
zed .
```

The development environment has whatever you may need already, but feel free to add or remove whatever
inside `shell.nix`.

## Building

Well, there are two ways of building your project. You can either go with classic `cargo build` way, but before that, make sure to enter development environment to have cargo and all rust toolchain available in your PATH, you may do like that:

```bash
# Entering development environment
nix develop -c $SHELL

# Compile the project
cargo build --release
```

Or, you can build your project via nix which will do all the dirty work for you. Just, in your terminal:

```bash
# Build in nix environment
nix build

# Executable binary is available at:
./result/bin/bot
```

## Deploying (works only for flake based NixOS)

Deploying this project, telegram bot requires host machine to have its own flake based configuration.

### Activation

In your configuration, add your project repository to `inputs`.

```nix
{
  inputs = {
    # ...

    rustina.url = "github:rust-lang-uz/rustina";
  };
}
```

Ok, now we have your project in repository list and now, we need to make use of options provided by modules of your project. In order to do that, we need to activate our module by importing our module. In your configuration.nix, find where you imported things and then add your project like that:

```nix
# Most of the time it's at the top part of nix configurations
# and written only once in a nix file.
{ ... }: {
  # ... something

  # And here begins like that
  imports = [
    # Imagine here your existing imports

    # Now import your project module like this
    inputs.rustina.nixosModules.bot
  ];
};
```

Alright! Since we imported the module of our project and options are now available, now head into setting up section!

### Set up

Options are available, modules are activated and everything is ready to deploy, but now, we need to explain NixOS how
to deploy our project by writing some Nix configs. I already wrote some options and configurations which will be available
by default after project bootstrap, you are free to modify, add and remove whatever inside `module.nix` to your
liking. If you need list of available default options or explanations for every option, refer to [available default options] section below. In this guide, I'll
be showing you an example set up you may use to get started very fast, you'll find out the rest option by yourself if you
need something else. In your `configuration.nix` or wherever of your configuration:

```nix
{
  services.rustina = {
    # Enable systemd service
    enable = true;

    # Telegram bot token passed to your bot via arguments
    tokens = {
      telegram = "/srv/bot-token";
      github = "/srv/github-token";
    };

    # Enabling webhook integration which activates
    # caddy or nixos part of nix configuration at
    # `module.nix`
    webhook = {
      # Activate webhook part of nix configuration
      enable = true;

      # From given options (caddy or nginx), choose
      # web server to deploy bot via an http server
      proxy = "caddy";

      # Domain to pass to web server (caddy or nginx)
      domain = "bot.rust-lang.uz";

      # Port to host http server and tell web proxy
      # to were bind that proxy
      port = 8445;
    };
  };
}
```

This is very basic examples, you can tune other things like user who's going to run this systemd service, change group of user and many more. You can add your own modifications and add more options by yourself.

### Available default options

These are options that are available by default, just put services.rustina before the keys:

#### `enable` (required) -> bool

Turn on systemd service of telegram bot project.

#### `tokens.telegram` (required) -> path to file

Telegram bot token to pass to telegram bot, it should be a file that can be placed almost anywhere. Inside the file, there should be only telegram bot token as whole content. Don't type telegram bot token directly as value for this option, it was done like that to don't expose your token openly in your public repository or expose it at /nix/store. Also, you can chain it with secret manager like `sops-nix` like that:

```nix
{
  sops.secrets = {
    "telegram-token" = {
      owner = config.services.rustina.user;
    };
    "github-token" = {
      owner = config.services.rustina.user;
    };
  };

  services.rustina.tokens = {
    telegram = config.sops.secrets."telegram-token".path;
    github = config.sops.secrets."github-token".path;
  };
}
```

#### `tokens.github` (required) -> path to file

The logic is same as `tokens.telegram`, but this one is needed for doing certain GitHub API calls.

#### `webhook.enable` (optional) -> bool

Enable automatic web proxy configuration for either caddy or nginx. If the value is false, telegram bot will be deployed in `polling` mode. This is for people who have or want complex web server configurations.

#### `webhook.proxy` (optional) -> `caddy` or `nginx` as value

Choose which web server software should be integrated with.

#### `webhook.domain` (optional) -> string

It will be passed to web proxy to let it know whether to which domain the configurations should be appointed to.

#### `webhook.port` (optional) -> integer

Which port should be used to host bot and proxy.

#### `user` (optional) -> string

The user that will run the telegram bot. It's defaulted to "{package.name}-bot".

#### `group` (optional) -> string

Name of a group to which the user that's going to run the telegram bot should be added to. It's defaulted to the name of the user.

#### `dataDir` (optional) -> path

A location where working directory should be set to before starting telegram bot. If you have a code to write something in current working directory, the value to this option is where it will be written. It's defaulted to "/var/lib/{package.name}-bot".

#### `package` (optional) -> nix package

The packaged telegram bot with pre-compiled binaries and whatever. Defaulted to current project's build output and highly suggested to not change value of this option unless you know what you're doing.

## Working production

Currently, this telegram bot is deployed at Kolyma's Datacenter Infrastructure which can be observed from here: https://github.com/kolyma-labs/instances/blob/main/nixos/kolyma-2/services/rustina.nix

## Thanks

- [Orzklv](https://github.com/orzklv) - For creating and maintaining this awesome bot (shameless ad).
- [Rust Telegram Bot Template](https://github.com/xinux-org/templates) - For helping to initiate bot faster and proceed with development.

## License

This project is licensed under the MIT or Apache-2.0 license - see the [LICENSE-MIT](LICENSE-MIT) or [LICENSE-APACHE](LICENSE-APACHE) file for details.

<p align="center">
    <img src=".github/assets/footer.png" alt="Rust Uzbekistan's {Telegram}">
</p>
