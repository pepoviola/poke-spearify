# poke-spearify

What if the description of each Pokémon were to be writting using Shakespeare's style? This small project response to the question providing a REST api that allow users to get the description of each Pokémon in Shakespeare's style.

It's use the [poke api](https://pokeapi.co/) to get the description of the given pokémon and the [fun translation api](https://funtranslations.com/api/shakespeare) to translate to Shakespeare's style.

## Requeriments

- [Rust](https://www.rust-lang.org/), this project is written in `rust` and is required to build it. You can follow the [get started guide](https://www.rust-lang.org/learn/get-started) to install the toolchain.

- [Docker](https://www.docker.com) (*Optional*), you can install docker from the [get started page](https://www.docker.com/get-started) if you want to build this project in a container.

- [Git](https://git-scm.com) (*Optional*), you can install from the [inatallation page](https://git-scm.com/book/en/v2/Getting-Started-Installing-Git).


## How to get the code
You can clone the code using `git` or download the source as `zip`.

## How to build and run (*locally*)

- With `dev` profile.

  You can build this project with the default `dev` profile running

  ```bash
  $ cargo build
  ```

  And the run with

  ```bash
  $ ./target/debug/poke-spearify
  ```

  Or just run this command to build & run

  ```bash
  $ cargo run
  ```


- With `Release` profile.

  You can build this project for release with this command

  ```bash
  $ cargo build --release
  ```

  And the run with

  ```bash
  $ ./target/debug/poke-spearify
  ```

  Or just run this command to build & run

  ```bash
  $ cargo run --release
  ```

  ## Settings

You can set following environment variables

- RUST_LOG, set the log level.
- PORT, the port number to listen. Default to `5000`.
- TRANSLATION_API_KEY, [funtranslations](https://funtranslations.com/api/shakespeare) has a rate limit of 60 API calls a day with distribution of 5 calls an hour. **If** you have a subscription you can set your `API SECRET` here.