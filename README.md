# Kvsd

Simple Key Value store .

## Quick Start

terminal1
```
# running server (default port: 7379)
$ kvsd server --disable-tls
```

terminal2
```
# running client
$ kvsd set key1 value1 --disable-tls
OK

$ kvsd get key1 --disable-tls
value1

$ kvsd delete key1 --disable-tls
OK old value: value1
```

## Configurations

The order of configuration priority is as follows.(high to low)

- command line flag
- environment variables
- configuration file
- default value

### kvsd 

### server

| Key | Default | 
| --- | ------- |
| max_tcp_connections | xxx | 
| connection_tcp_buffer_bytes | xxx |

## Logging

To specify logging directive, use `KVSD_LOG` environment variable.

```console
$ KVSD_LOG=info kvs 
```

## TODO

- [ ] Remove `unreachable!() macro`
- [ ] Use derive to reduce message/uow boiler plate code
- [ ] Closing files during graceful shutdown
- [ ] Add Benchmark
