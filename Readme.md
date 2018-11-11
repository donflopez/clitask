# Clitask

Clitask is a **non-official** command line tool for managing webtasks, it comes with a small *beta* store.

This tool just makes it simple to use the available webtasks in [webtask.io](http://webbtask.io)

## How to use
```bash
_$: cargo install clitask

_$: clitask config flopez
> User configured, you now can use this tool to handle webtasks!

_$: clitask list
> flopez/reverse      Reverses the input

_$: clitask add flopez/reverse
> Webtask reverse added to your local repository.

_$: clitask call reverse "reverse this!"
> !siht esreveR

_$: clitask publish to_binary https://webtask.io/webbbtaskurl --description "Convert any input to a binary code"
> Webtask published!
```


