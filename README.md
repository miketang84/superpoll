# Superpoll

superpoll - A rust async runtime extends futures strictly, rather than rewriting many things.

## Plan

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

## Details

- `superpoll::io`
	- changed dependancy `futures-lite` to `futures`
- `superpoll::blocking`
	- changed dependancy `futures-lite` to `futures`
	- changed dependancy `async-channel::bounded` to `futures::channel::mpsc::channel`
	- changed dependancy `atomic-waker::AtomicWaker` to `futures::task::AtomicWaker`
- `superpoll::net`
	- changed dependancy `futures-lite` to `futures`
- `superpoll::fs`
	- changed dependancy `futures-lite` to `futures`
	- changed dependancy `async-lock::Mutex` to `futures::lock::Mutex`
- `superpoll::process`
	- changed dependancy `futures-lite` to `futures`

## Not Finished

- `superpoll::blocking` depends on `async-task`'s `Runnable` and `Task`, which doesn't exist in `futures` crate, 
  so we need to count how to rewrite it? Or not necessary to do so.
- Reexport more symbols to superpoll root space for convienence.

