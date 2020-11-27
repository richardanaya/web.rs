# cargo-webassembly 

This cargo extension is for developing front end applications in WebAssembly.

This project is currently **beta**, but totally functional! Next steps include:

* cleaning up the code to use less `unwrap`
* use a better open url library (the current one has some weird behavior).
* support project file watching and recompiling

<p align="center">
  <img height="300" src="../../images/undraw_website_builder_bxki.png">
</p>

Install with the following command:

```
cargo install cargo-webassembly
```

# Create a new project

```
cargo webassembly new my_project
```

This will initialize a Rust WebAssembly project completely setup to run in the browser.

# Run your project

Go into your projects root directory (e.g. `cd my_project`)

```
cargo webassembly run
```

This will compile and start a server by default on port `8080`, you can change the port with the `-p` option.

This server is setup for single-page apps where all non-static file routes redirect to the root `index.html`.

# Building your project

```
cargo webassembly build
```

This command will just compile your WebAssembly and place everything you need for your web app in the `dist` folder.