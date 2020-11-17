# Superpoll

superpoll - A rust async runtime extends futures strictly, rather than rewriting many things.

## Aim

superpoll aims not to do much original works, but integrate and import the best components 
there are, and expose unified interface.

The most important focus: we follow `futures` strictly, and will do nothing conflicted with `futures`,
if there are some function in `futures`, we will use it/them directly, do not rewrite, or overwrite it.

We have done:

- ported stjepang's `async-io` as `superpoll::io`
- ported stjepang's `blocking` as `superpoll::blocking`
- ported stjepang's `async-net` as `superpoll::net`
- ported stjepang's `async-fs` as `superpoll::fs`
- ported stjepang's `async-process` as `superpoll::process`

## Components

- [superpoll::io](https://github.com/daogangtang/superpoll-io): Timer, Async
- [superpoll::blocking](https://github.com/daogangtang/superpoll-blocking): Unblock, unblock
- [superpoll::net](https://github.com/daogangtang/superpoll-net): TcpListner, TcpStream, UdpSocket, UnixSocket and so on...
- [superpoll::fs](https://github.com/daogangtang/superpoll-fs): standard fs async api
- [superpoll::process](https://github.com/daogangtang/superpoll-process): POSIX process async api

## Reorgnization Details

- `superpoll::io`
	- changed dependency `futures-lite` to `futures`
- `superpoll::blocking`
	- changed dependency `futures-lite` to `futures`
	- changed dependency `async-channel::bounded` to `futures::channel::mpsc::channel`
	- changed dependency `atomic-waker::AtomicWaker` to `futures::task::AtomicWaker`
- `superpoll::net`
	- changed dependency `futures-lite` to `futures`
- `superpoll::fs`
	- changed dependency `futures-lite` to `futures`
	- changed dependency `async-lock::Mutex` to `futures::lock::Mutex`
- `superpoll::process`
	- changed dependency `futures-lite` to `futures`

## Not Finished

- `superpoll::blocking` depends on `async-task`'s `Runnable` and `Task`, which doesn't exist in `futures` crate, 
  so we need to count how to rewrite it? Or not necessary to do so.
- Reexport more symbols to superpoll root space for convienence.

