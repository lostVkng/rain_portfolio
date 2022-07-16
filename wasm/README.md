# personal website rust webasm code



### Running code requires unstable API call
```shell
RUSTFLAGS=--cfg=web_sys_unstable_apis cargo build
```
```shell
RUSTFLAGS=--cfg=web_sys_unstable_apis wasm-pack build
```

To compile build web
```shell
RUSTFLAGS=--cfg=web_sys_unstable_apis wasm-pack build --target web --out-name portfolio --out-dir ./dist
```

Using node http-server to test
```shell
http-server -c-1
```

Copy index.html, style.css from web dir, and portfolio_bg.wasm and portfolio.js from dist dir