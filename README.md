# toml2json

Convert TOML to JSON.

Example:

```
toml2json -f Cargo.lock | jq -r '.package | .[] | "\(.name): \(.version)"'
```

```
cat Cargo.lock | toml2json | jq -r '.package | .[] | "\(.name): \(.version)"'
```

## License ##

This code is open source software licensed under the [Apache 2.0 License](http://www.apache.org/licenses/LICENSE-2.0.html).
