# Superpoll

superpoll - A rust async runtime extends futures strictly, rather than rewriting many things.

## Plan

superpoll aims not to do much original works, but integrates and imports the best components 
there are, and exposes unified interface.

The most important focus: we follow `futures` strictly, will do nothing conflicted with `futures`,
if there are some function in `futures`, we will use it/them directly, do not rewrite, or overwrite it.

So, we think we will do:

- integrate `futures-net`
- porting stjepang's `blocking`
- porting stjepang's `async-fs`
- porting stjepang's `async-process`
- others.


