# pickle

A CLI tool and proxy to consume [Ricky and Morty API](https://rickandmortyapi.com/).

## Usage
Fetches character with id 1.
```bash
cargo run character 1
```

Fetches all characters.
```bash
cargo run character
```
There are similar commands for episodes and locations. Consult `--help`.

## Proxy
```bash
cargo run proxy -p PORT_NUMBER
```
Proxy runs on localhost. 

```curl
curl 'http://localhost:3031/episode' -H 'Authorization: keyword-randomsequenceofbytes'
```

This command returns an API key.

```bash
cargo run sign-up foo
```

Keyword will prefix your API key. It simplifies the logic. 
Note that we only append the keyword so it does not lower the 
entropy of the random portion of the key (uuid v4).

Key will allow you to use the cache of the proxy.

Please read below for note about secret key.

## Components

**[rick-and-morty](https://docs.rs/rick-and-morty/latest/rick_and_morty/index.html)** - 
This is simply a wrapper over `reqwest` so reduces boilerplate code.

**[securestorage](https://docs.rs/securestore/latest/securestore/index.html)** - 
This encrypts and stores API keys so we can version them. 
To make experimenting with this project simpler, the secret for the vault of 
API keys is versioned and included. **THUS, THIS IS NOT PRODUCTION SAFE.** If you wish,
delete the secret and use `ssclient` with cargo and generate a new secret.
Please see docs in `securestorage`.

**[moka](https://docs.rs/moka/latest/moka/)** - Concurrent cache library for Rust.

**[warp](https://docs.rs/warp/latest/warp/index.html)** - warp is a super-easy, composable, web server framework for warp speeds.




