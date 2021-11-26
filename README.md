# Game of Life

According to Wikipedia, the **Game of Life**, also known simply as *Life*, is a [cellular automaton]
devised by the British mathematician [John Horton Conway] in 1970.
It is a [zero-player game], meaning that its evolution is determined by its initial state,
requiring no further input. One interacts with the Game of Life by creating an initial
configuration and observing how it evolves. It is [Turing complete] and can simulate a
[uuniversal constructor] or any other [Turing machine].

[John Horton Conway]: https://en.wikipedia.org/wiki/John_Horton_Conway
[cellular automaton]: https://en.wikipedia.org/wiki/Cellular_automaton
[zero-player game]: https://en.wikipedia.org/wiki/Zero-player_game
[Turing complete]: https://en.wikipedia.org/wiki/Turing_complete
[Turing machine]: https://en.wikipedia.org/wiki/Turing_machine

## 🚴 Usage

### 🛠 Build with `wasm-pack build`

```shell
$ wasm-pack build
```

After running this command, you should have a folder named `pkg/` which contains the wasm generated files.
If `package.json` isn't generated for you, create the file (`pkg/package.json`) with the following contents:

```json
{
  "name": "game-of-life",
  "collaborators": [
    "Victor I. Afolabi <javafolabi@gmail.com>"
  ],
  "description": "Conway's Game of Life",
  "version": "1.0.0",
  "license": "MIT",
  "repository": {
    "type": "git",
    "url": "git@github.com:victor-afolabi/game-of-life.git"
  },
  "files": [
    "game_of_life_bg.wasm",
    "game_of_life.d.ts"
  ],
  "main": "game_of_life.js",
  "types": "game_of_life.d.ts"
}
```

### 🌎 Start Web Server with `npm`

```shell
$ cd www/
$ npm install
$ npm run start
```

### 🔬 Test in Headless Browsers with `wasm-pack test`

```shell
$ wasm-pack test --headless --firefox
```

### 🎁 Publish to NPM with `wasm-pack publish`

```shell
$ wasm-pack publish
```

## 🔋 Batteries Included

- [`wasm-bindgen`] for communicating between WebAssembly and JavaScript.
- [`console_error_panic_hook`] for logging panic messages to the developer console.
- [`wee_alloc`], an allocator optimized for small code size.

[`wasm-bindgen`]: https://github.com/rustwasm/wasm-bindgen
[`console_error_panic_hook`]: https://github.com/rustwasm/console_error_panic_hook
[`wee_alloc`]: https://github.com/rustwasm/wee_alloc
