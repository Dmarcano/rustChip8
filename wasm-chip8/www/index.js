import * as wasm from "wasm_chip8";
import {memory} from "wasm_chip8/wasm_chip8_bg";


const SCREEN_WIDTH = 64; 
const SCREEN_HEIGHT = 32; 

const canvas = document.getElementById('chip8-canvas');
const ctx = canvas.getContext('2d')

let isRunning = false; 

const CANVAS_COLOR = [255, 0, 0, 255];

const ROMS = ["BC_test", "octojam2title", "danm8ku", "test_opcode" ]

const main = () =>  { 
    let chip8 = wasm.WasmChip8.new(); 
    console.log(chip8)
    let default_rom = "octojam2title.ch8";
    
    const forward_btn = document.getElementById('forward-chip8');
    const start_btn = document.getElementById('start-chip8');
    const stop_btn = document.getElementById('stop-chip8');

    const sel_element = create_rom_selection(); 

    sel_element.addEventListener('change', (event) => { 
        let rom_name = event.target.value; 
        let file_extension = `${rom_name}.ch8`;

        get_rom_file(file_extension).then(rom => {
            chip8.reset();
            chip8.load_rom_js(rom);
            update_canvas(chip8);
        });
    });


    forward_btn.addEventListener("click", function() { 
            emulate_cycle(chip8);
    });

    start_btn.addEventListener("click", function() { 
        isRunning = true; 
        stop_btn.disabled = false; 
        start_btn.disabled = true; 
        forward_btn.disabled = true; 


    });

    stop_btn.addEventListener('click', function() { 
        isRunning = false;
        stop_btn.disabled = true; 
        start_btn.disabled = false; 
        forward_btn.disabled = false;
    })


    get_rom_file(default_rom).then((rom) => { 
        chip8.reset();
        chip8.load_rom_js(rom); 
        update_canvas(chip8);

        emulation_loop(chip8);

    }); 
}

const emulate_cycle = (chip8) => { 
    chip8.cycle(); 
    update_canvas(chip8); 
}


const emulation_loop = (chip8) => { 
    if(isRunning) { 
        for(var i =0; i < 9; i++) {
            chip8.cycle(); 
        }
        update_canvas(chip8); 

        if(!isRunning) { 
            return;
        }
    }
    window.requestAnimationFrame(() => {
        emulation_loop(chip8);
      });
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

const create_rom_selection = () => { 
    const sel_element = document.createElement('select');
    const sel_div = document.getElementById('rom-select');
    sel_element.name = "rom_drop"
    sel_element.id = "rom-select"

    const rom_select = ROMS.map(rom => {
        return `<option value="${rom}">${rom}</option>`;
    });
    sel_element.innerHTML = rom_select;
    window.onload = () => sel_div.appendChild(sel_element);
    return sel_element; 
}



main()