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

sea-orm-cli generate entity -u postgres://postgres:Rkf7010zaqxsw%21%40%23@localhost/digital_cookbook -o src/entities --with-serde both

sea-orm-cli generate entity --database-url postgres://postgres:Rkf7010zaqxsw%21%40%23@localhost/digital_cookbook --output-dir src/entites

postgres://postgres:Rkf7010zaqxsw%21%40%23@localhost/digital_cookbook
postgres://postgres:Rkf7010zaqxsw%21%40%23@localhost/digital_cookbook
