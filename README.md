# Superpoll

superpoll - A rust async runtime extends futures strictly, rather than rewriting many things.

## Plan

superpoll aims not to do much original works, but integrate and import the best components 
there are, and expose unified interface.

The most important focus: we follow `futures` strictly, and will do nothing conflicted with `futures`,
if there are some function in `futures`, we will use it/them directly, do not rewrite, or overwrite it.

So, we think we will:

- port stjepang's `async-io`
- port stjepang's `async-net`
- port stjepang's `blocking`
- port stjepang's `async-fs`
- port stjepang's `async-process`
- integrate alex's `futures-timer`
- and so on..

