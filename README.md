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
sea-orm-cli generate entity -u postgresql://postgres:2UVu6QnFJOjeqb5YZSpb@containers-us-west-82.railway.app:6890/railway -o src/entities --with-serde both

