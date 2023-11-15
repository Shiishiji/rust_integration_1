# Prerequisites

It is highly recommended to use cargo to build and run this project.
Install cargo: `curl https://sh.rustup.rs -sSf | sh`.
It will download a script, and start the installation.

GUI is written with GTK4. To figure out the version installed on your system run
`pkg-config --modversion gtk4`. Mine is `4.6.9`.

For database to work `sqlx-cli` is needed.

Installation of sqlx-cli: `cargo install sqlx-cli`.
It is important to source the `.env` file by executing `source .env`.

# Building project

To compile the binary run `cargo build`.

To run app in dev mode execute `cargo run`.

# Running migrations

