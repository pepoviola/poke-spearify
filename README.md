# poke-spearify

What if the description of each Pokémon were to be writting using Shakespeare's style? This small project response to the question providing a REST api that allow users to get the description of each Pokémon in Shakespeare's style.

It's use the [poke api](https://pokeapi.co/) to get the description of the given pokémon and the [fun translation api](https://funtranslations.com/api/shakespeare) to translate to Shakespeare's style.

You can try the api live in [poke-spearify](https://poke-spearify.labs.javierviola.com/).

```bash
curl  https://poke-spearify.labs.javierviola.com/pokemon/charizard
{"name":"charizard","description":"Spits fire yond is hot enow to melt boulders. Known to cause forest fires unintentionally."}
```

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


## How to build and run  with Docker

To run this project as a container you need first to build the `image` with the following command:

```bash
$ docker build -t poke-spearify:latest .
```

We can check the `image`
```bash
$ docker image ls
REPOSITORY                    TAG           IMAGE ID       CREATED          SIZE
poke-spearify                 latest        849123371151   10 minutes ago   82.4MB
```

Run the server (*detached and binding port 5000 to localhost*)

```bash
$ docker run -d -p 5000:5000 --rm poke-spearify
```

Now we can follow the logs

```bash
$ docker logs $(docker ps | grep poke-spearify | awk '{print $1}') --follow
{"level":30,"time":1613931181710,"msg":"Logger started","level":Info}
Server listening on http://0.0.0.0:5000
```

And *in another* terminal make a test request

```bash
$ curl localhost:5000/pokemon/charizard
{"name":"charizard","description":"Spits fire yond is hot enow to melt boulders. Known to cause forest fires unintentionally."}
```


## Possible improvements

- [ ] Improve error handling.
- [ ] Add cache, to memoize 3rd party api responses.
- [ ] Add observability provider (e.g honeycomb).
- [ ] Add UI.
