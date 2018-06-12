# Gutsy

An actix based network log writer.

it accepts json log lines via websocket and writes them to stdout.

## Example log line:

```json
{
    "time":"2018-06-12T23:33:41.193Z",
    "category": "cat images",
    "message": [
        "OMG OMG, this is a gutsy log line",
        "you can put anything in here",
        [1,2,3]
    ],
    "sequence": 9001,
    "clientId": "it's me, Mario",
    "tag": "heuer"
}
```

## TODO

* [ ] non-blocking file writing
    currently we only write to stdout `println!();`.
    Eventually something somewhere has to block. Let's use [tokio-fs](https://github.com/tokio-rs/tokio/tree/master/tokio-fs) or something similar.
* [ ] env variable configuration
* [ ] TLS
