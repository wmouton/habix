![Habix Banner](/images/habix-banner.webp)

# Habix

Habix is a command-line tool that fetches pending tasks from Habitica and sends desktop notifications. It helps you stay on top of your daily tasks and to-dos by reminding you of incomplete activities.

## Features

- Fetches pending Habitica tasks
- Sends desktop notifications for unfinished tasks
- Stores API credentials securely in `~/habix-keys/habix.md`
- Simple CLI commands for setup and execution
- Supports installation via Cargo and Nix

## Requirements

- Rust and Cargo installed
- Habitica account
- Linux or macOS (for `notify-send` compatibility)
- OpenSSL and pkg-config (required for building)

## Installing from Source

To install Habix by cloning the repository:

```sh
git clone https://github.com/wmouton/habix.git
cd habix
cargo build --release
```

Once built, the executable will be located at:

```sh
target/release/habix
```

You can move it to a directory in your PATH for easier access:

```sh
sudo mv target/release/habix /usr/local/bin/
```

## Running Habix

After installation, you can set up your Habitica API credentials:

```sh
habix setup
```

The `habix setup` command will prompt you to enter your Habitica User ID and API Key. These credentials will be stored in `~/habix-keys/habix.md` for future use.

To fetch pending tasks and receive notifications, run:

```sh
habix run
```

## Using Nix Flake for Development

If you're using Nix, you can set up the development environment with the provided `flake.nix`.

### Using Flakes

1. Ensure you have [Nix](https://nixos.org) installed and flakes enabled.
2. Clone the repository:

   ```sh
   git clone https://github.com/wmouton/habix.git
   cd habix
   ```

3. Enter the development shell:

   ```sh
   nix develop
   ```

   This will provide a shell with the necessary dependencies (like OpenSSL and pkg-config).

### Using `shell.nix`

If you prefer not to use flakes, you can still enter a development shell using `shell.nix`:

```sh
nix-shell
```

This will set up the required dependencies for building Habix.

---

## Contributing

Feel free to fork the project and submit pull requests! Contributions are welcome to improve Habix and add new features. If you encounter any issues, please open an issue on [GitHub](https://github.com/wmouton/habix/issues).

Now you're ready to contribute or use Habix efficiently!

