import * as wasm from "wasm_chip8";
import {memory} from "wasm_chip8/wasm_chip8_bg";


const SCREEN_WIDTH = 64; 
const SCREEN_HEIGHT = 32; 

const canvas = document.getElementById('chip8-canvas');
const ctx = canvas.getContext('2d')

const main = () =>  { 
    let chip8 = wasm.WasmChip8.new(); 
    console.log(chip8)
    let name = "BC_test.ch8";
    
    get_rom_file(name).then((rom) => { 

        chip8.load_rom_js(rom); 
    
       
        let btn = document.getElementById('forward-chip8');
        btn.addEventListener("click", function() { 
           update_emulator(chip8);
        })
    
    }); 
}

const update_emulator = (chip8) => { 
    for(var i =0; i < 9; i++) {
        chip8.cycle();
    }

    update_canvas(chip8); 
}

const update_canvas = (chip8) => {
    
    const image = ctx.createImageData(SCREEN_WIDTH, SCREEN_HEIGHT); 
    const data = image.data; 
    const chip8_display_buf = new Uint8Array( memory.buffer, chip8.get_display(), SCREEN_HEIGHT * SCREEN_WIDTH ); 

    // data is a 4*Width*Height array since each set of 4 indeces corresponds to one pixels RGBA val
    for(var i = 0; i < data.length; i += 4) { 
        if(chip8_display_buf[i/4]) { 
            data[i]     = 255    // red
            data[i + 1] = 0; // green
            data[i + 2] = 0; // blue
            data[i+3] = 255; //alpha
        }
        else { 
            data[i]     = 0    // red
            data[i + 1] = 0; // green
            data[i + 2] = 0; // blue
            data[i+3] = 255; //alpha

        }
    }
    ctx.putImageData(image, 0, 0);
}

const get_rom_file = async(name) => { 
    let file = await fetch(`roms/${name}`); 
    let buff = await file.arrayBuffer(); 
    return new DataView(buff, 0, buff.byteLength); 
}

main()