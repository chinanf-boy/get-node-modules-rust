## Walk thougn the Given Dir by Rust crate(readir)

### Why

- f.f.f.f..f.f..ff...f.ff.f..fast

### Use

```js
const files = require('get-node-modules-rust').find('./');
```

### Demo

```
npm run start
```

see Result

```bash
[ './node_modules' ]
cost time: 0.043 s
```

- 3.5G ~ 2s

### Build

- Rust, [lib.rs](src/lib.rs) with `libc`

```
cargo build --release
```

cp dylib into the dir named in `package.json` and `index.js`

```
cp target/release/libfind_files.dylib ./rust-dylib
```

- Node, [index.js](index.js) with `node-ffi`

```js
var lib = ffi.Library(path.join(__dirname, './rust-dylib/libget_dir'), {
  get_dir: ['char *', ['string']],
  free_memory: ['void', ['char *']]
});
```

- [more details of `rust-ffi` ](https://github.com/shepmaster/rust-ffi-omnibus)

## Use by

- [node-modules-size ](https://github.com/chinanf-boy/node-modules-size)
