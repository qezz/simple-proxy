# Simple stupid proxy

This proxy receives your request to the `in-addr` and retries it to
the specified host in `out-addr`.

The general idea behind it is to have an ability to request websites
with modern TLS while the services in the internal network only use
http. 

Imagine you have a very strange configuration of the host machine, say
with an outdated operating system or maybe Java 6 application. The
solution is to use some kind of proxy your application doesn't know
about. It is very easy to use simple-proxy in the internal network to
avoid these issues:

```
cargo build --release -- -i 0.0.0.0:8899 -o https://rust-lang.org

# in another terminal
curl -vvv 127.0.0.1:8899/tools/install
# see the output of https://www.rust-lang.org/tools/install
```

## Build

### On the same arch

As usual,

```
cargo build --release
```

### Statically against musl

```
docker build -f Dockerfile -t simpleproxy-bilder:latest .
docker run --rm -v $(pwd)/build:/build simpleproxy-builder:latest
```

## License

MIT or Apache 2.0, at your choice.

## Author

Sergey Mishin <sergei.a.mishin@gmail.com>

