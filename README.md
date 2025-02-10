![Habix Banner](/assets/habix-banner.png)

# Habix

Habix is a command-line tool that fetches **incomplete dailies and todos** from Habitica and sends desktop notifications. It helps you stay on top of your daily tasks and to-dos by reminding you of outstanding activities that need your attention.

---

## Features

- **Fetch Incomplete Dailies and Todos**: Retrieves only incomplete tasks of type `daily` and `todo` from Habitica.
- **Desktop Notifications**: Sends notifications for outstanding tasks using `notify-send` (Linux) or native notifications (macOS).
- **Secure Credential Storage**: Stores your Habitica API credentials securely in a `.env` file in the project directory.
- **Simple CLI Commands**: Easy-to-use commands for setup, execution, and cleanup.
- **Cross-Platform Installation**: Supports installation via Cargo and Nix.
- **Development Environment**: Includes Nix Flake and `shell.nix` for seamless development setup.

---

## Requirements

- **Rust and Cargo**: Installed on your system.
- **Habitica Account**: Required to fetch tasks.
- **Linux or macOS**: For desktop notifications (Linux uses `notify-send`, macOS uses native notifications).
- **OpenSSL and pkg-config**: Required for building the project.

---

## Installation

### Installing from Source

1. Clone the repository:
   ```sh
   git clone https://github.com/wmouton/habix.git
   cd habix
   ```

2. Build the project:
   ```sh
   cargo build --release
   ```

   The compiled binary will be located at:
   ```sh
   target/release/habix
   ```

3. Install the binary to your system:
   ```sh
   cargo install --path .
   ```

   This will install the `habix` binary to `~/.cargo/bin/`, which is typically included in your system's `PATH`.

4. (Optional) Move the binary to a directory in your `PATH` for easier access:
   ```sh
   sudo mv target/release/habix /usr/local/bin/
   ```

---

## Running Habix

### Setting Up Credentials

After installation, set up your Habitica API credentials using the `setup` command:
```sh
habix setup
```

This command will prompt you to enter your **Habitica User ID** and **API Token**. These credentials will be securely stored in a `.env` file in the project directory.

### Fetching Outstanding Tasks

To fetch **incomplete dailies and todos** and receive desktop notifications, run:
```sh
habix run
```

This will:
1. Fetch your incomplete `dailies` and `todos` from Habitica.
2. Send a desktop notification listing your outstanding tasks.
3. Notify you if you're all caught up with no incomplete tasks.

If the `.env` file is missing or doesn’t contain valid credentials, the program will print a helpful message prompting you to run `habix setup`.

### Cleaning Up Credentials

To delete the `.env` file and remove your stored credentials, run:
```sh
habix clean
```

This command will:
1. Delete the `.env` file if it exists.
2. Print a confirmation message.

---

## Using Nix for Development

If you're using Nix, you can set up the development environment with the provided `flake.nix` or `shell.nix`.

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

   This will provide a shell with all the necessary dependencies (e.g., OpenSSL, pkg-config).

### Using `shell.nix`

If you prefer not to use flakes, you can still enter a development shell using `shell.nix`:
```sh
nix-shell
```

---

## Contributing

Contributions are welcome! If you'd like to contribute to Habix, follow these steps:

1. Fork the project on [GitHub](https://github.com/wmouton/habix).
2. Clone your fork and create a new branch:
   ```sh
   git checkout -b my-feature-branch
   ```
3. Make your changes and commit them:
   ```sh
   git commit -m "Add my new feature"
   ```
4. Push your changes to your fork:
   ```sh
   git push origin my-feature-branch
   ```
5. Open a pull request on the main repository.

If you encounter any issues, please open an issue on [GitHub](https://github.com/wmouton/habix/issues).

---

## License

Habix is licensed under the **GPL-3.0 License**. See the [LICENSE](LICENSE) file for details.

---

## Example Workflow

1. **Set Up Credentials**:
   ```sh
   habix setup
   ```
   Enter your Habitica User ID and API Token when prompted.

2. **Fetch Outstanding Tasks**:
   ```sh
   habix run
   ```
   You’ll receive a desktop notification with your incomplete `dailies` and `todos`.

3. **Clean Up**:
   ```sh
   habix clean
   ```
   This will delete the `.env` file and remove your stored credentials.

---

## Troubleshooting

- **Missing `.env` File**: If the `.env` file is missing or invalid, `habix run` will prompt you to run `habix setup`.
- **API Errors**: If you encounter API errors (e.g., `401 Unauthorized`), ensure your credentials are correct and up-to-date.
- **Desktop Notifications**: Ensure `notify-send` is installed on Linux or that your macOS system supports native notifications.

---

Now you're ready to use Habix to stay productive and organized by focusing on your **outstanding incomplete dailies and todos**! 🚀

---

### Example Output

If you have the following tasks:
- **Incomplete Todo**: "Complete the Rust project"
- **Completed Daily**: "Write documentation"
- **Incomplete Daily**: "Read a book"

The notification will show:
```
🔴 You have outstanding tasks 🔴:

Complete the Rust project
Read a book
```

---

Enjoy using Habix to stay on top of your tasks! 🚀
