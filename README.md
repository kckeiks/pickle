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

```curl
curl 'http://localhost:3031/episode' -H 'Authorization: keyword-randomsequenceofbytes'
```

Proxy runs on localhost.

Endpoints:

```
* /location
* /location/id
* /character
* /character/id
* /episode
* /episode/id
```

This command returns an API key.

```bash
cargo run sign-up foo
```

Keyword will prefix your API key. It simplifies the logic. 
Note that we only append the keyword so it does not lower the 
entropy of the random portion of the key (uuid v4).

Key will allow you to use the cache of the proxy.

## Security 

This crate uses [securestorage](https://docs.rs/securestore/latest/securestore/index.html)
to encrypt and store API keys so we can version them along with the code.
To make experimenting with this project simpler, the secret for the vault of
API keys is versioned and included as well. If you wish, delete the secret and 
use `ssclient` and generate a new secret.
Please see docs in `securestorage` for `ssclient` command.

In addition, for simplicity, the proxy connection is not encrypted so you 
should consider upgrading the `warp` server that this program uses to 
use TLS. You will need to manage your certificates :).

**THUS, THIS IS NOT PRODUCTION SAFE.**

## Components

**[rick-and-morty](https://docs.rs/rick-and-morty/latest/rick_and_morty/index.html)** - 
This is simply a wrapper over `reqwest` so reduces boilerplate code.

**[securestorage](https://docs.rs/securestore/latest/securestore/index.html)** - 
This encrypts and stores API keys so we can version them along with the code.

**[moka](https://docs.rs/moka/latest/moka/)** - Concurrent cache library for Rust.

**[warp](https://docs.rs/warp/latest/warp/index.html)** - warp is a super-easy, composable, web server framework for warp speeds.




