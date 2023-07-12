## Dev Test
```sh
# Test for model
cargo watch -q -c -w src/ -x 'test model_ -- --test-threads=1 --nocapture'

# Test for web
cargo watch -q -c -w src/ -x 'test web_ -- --test-threads=1 --nocapture'

```

## Dev Web
```sh
cargo watch -q -c -w src/ -x 'run -- web-folder'
```

# Generate entity files of database `bakery` to `entity/src`


