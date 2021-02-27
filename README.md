# Rust Chip8 and WebAssembly

## [Demo here](https://www.wasmchip8.diegomarcano.dev).

![](public/chip8_screenshot.png)

This project is a [Chip-8](https://en.wikipedia.org/wiki/CHIP-8) Interpreter written in Rust as an excercise to learn how to use WebAssembly. Most of the ROMS are taking from [this archive](https://johnearnest.github.io/chip8Archive/) 

The Chip-8 is considered a rite of passage project for many devs looking into creating emulators. If you're interested I suggest you give it a try! 

Some really useful resources that I found where [this blog](https://austinmorlan.com/posts/chip8_emulator/) which has a C++ tutorial on the Chip8 and this [blog](https://wtfleming.github.io/2020/01/26/chip8-webassembly-rust/) on running their own WebAssembly frontend to their own Chip8. (Much of my frontend is heavily ~~ripped~~ inspired by this post as well)

The specification on which almost all emulators are written from is found [here](http://devernay.free.fr/hacks/chip8/C8TECH10.HTM)

### Building and running

There are two parts to this project. The CPU library located in the `src` folder and the implementations of the frontend in the `sdl_chip8` and `wasm-chip8` directory. 
The only fully working implementation is the WebAssembly one. 

To run the WebAssembly one uses [wasm-pack](https://rustwasm.github.io/wasm-pack/) and node.

```
~ $ cd wasm-chip8

~/wasm-chip8/ $ wasm-pack build 

~/wasm-chip8/ $ cd www

~/wasm-chip8/www/ $ npm run start
```

