import * as wasm from "wasm_chip8";
import {memory} from "wasm_chip8/wasm_chip8_bg";


const SCREEN_WIDTH = 64; 
const SCREEN_HEIGHT = 32; 

const canvas = document.getElementById('chip8-canvas');
const ctx = canvas.getContext('2d')

let isRunning = false; 
let debug  = false; 

const CANVAS_COLOR = [255, 0, 0, 255];

const ROMS = ["snake" , "octojam2title", "PONG", "danm8ku", "TETRIS", "TANK"]

const main = () =>  { 
    let chip8 = wasm.WasmChip8.new(); 
    let default_rom = "snake.ch8";
    
    const forward_btn = document.getElementById('forward-chip8');
    const start_btn = document.getElementById('start-chip8');
    const stop_btn = document.getElementById('stop-chip8');
    const debug_btn = document.getElementById("debug")

    const keypad_rows = document.getElementsByClassName('keypad_row');
    setup_keypad(keypad_rows, chip8)

    const sel_element = create_rom_selection(); 

    sel_element.addEventListener('change', (event) => { 
        let rom_name = event.target.value; 
        let file_extension = `${rom_name}.ch8`;

        get_rom_file(file_extension).then(rom => {
            chip8.reset();
            chip8.load_rom_js(rom);
            update_memory(chip8);
            // update_state_ui(chip8);
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

    debug_btn.addEventListener('click', function() { 
        debug = !debug; 
        let ul = document.getElementById("memory_list");
        ul.innerHTML = ''
        let register_list = document.getElementById("register_list");
        // clear the element of old list elements by removing the inner html
        register_list.innerHTML = ''

           // Update the other state list <ul> element
        let state_list = document.getElementById("other_state_list");
        // clear the element of old list elements by removing the inner html
        state_list.innerHTML = ''
 
    
        update_memory(chip8);

    })

    document.addEventListener('keydown', event => { 
        if (CHIP8_KEYBOARD.hasOwnProperty(event.key)) {
            let key_code = CHIP8_KEYBOARD[event.key];
            chip8.key_down(key_code);           
        }
        else { 
            console.log(`wrong key!, used ${event.key}`)
        }
    });

    document.addEventListener('keyup', event => { 
        if (CHIP8_KEYBOARD.hasOwnProperty(event.key)) {
            let key_code = CHIP8_KEYBOARD[event.key];
            chip8.key_up(key_code);
        }
        else { 
            console.log(`wrong key!, used ${event.key}`)
        }
    });


    get_rom_file(default_rom).then((rom) => { 
        chip8.reset();
        chip8.load_rom_js(rom); 
        update_memory(chip8);
        // update_state_ui(chip8);

        update_canvas(chip8);

        emulation_loop(chip8);

    }); 
}


const setup_keypad = (keypad_rows, chip8) => { 
    for ( let i = 0; i< keypad_rows.length; i++) { 
        let row = keypad_rows[i];

        let buttons = row.children; 

        for(let j =0; j <buttons.length; j++) { 
            let button = buttons[j];

            KEYPAD_MAP[button.innerText.toLowerCase()] = button;

            // MOUSE EVENTS
            button.addEventListener('mouseup', function() { 
                if (CHIP8_KEYBOARD.hasOwnProperty(button.innerText.toLowerCase())) {
                    let key_code = CHIP8_KEYBOARD[button.innerText.toLowerCase()];
                    chip8.key_up(key_code);
                }
            })

            button.addEventListener('mousedown', function() { 
                if (CHIP8_KEYBOARD.hasOwnProperty(button.innerText.toLowerCase())) {
                    let key_code = CHIP8_KEYBOARD[button.innerText.toLowerCase()];
                    chip8.key_down(key_code);
                }
            })
            // TOUCHSCREEN EVENTS
            button.addEventListener('touchend', function() { 
                if (CHIP8_KEYBOARD.hasOwnProperty(button.innerText.toLowerCase())) {
                    let key_code = CHIP8_KEYBOARD[button.innerText.toLowerCase()];
                    chip8.key_up(key_code);
                }
            })

            button.addEventListener('touchstart', function() { 
                if (CHIP8_KEYBOARD.hasOwnProperty(button.innerText.toLowerCase())) {
                    let key_code = CHIP8_KEYBOARD[button.innerText.toLowerCase()];
                    chip8.key_down(key_code);
                }
            })

            
        }

    }

}

const emulate_cycle = (chip8) => { 
    chip8.cycle(); 
    update_memory(chip8);
    // update_state_ui(chip8);
    update_canvas(chip8); 
}


const emulation_loop = (chip8) => { 
    if(isRunning) { 
        // run 9 cycles of the chip8 CPU before rendering the screen
        for(var i =0; i < 9; i++) {
            let no_err = chip8.cycle(); 
            update_memory(chip8);
            // update_state_ui(chip8);
            if(!no_err) { 
                console.log(chip8.disassemble_memory())
            }
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
            data[i]     = CANVAS_COLOR[0]    // red
            data[i + 1] = CANVAS_COLOR[1]; // green
            data[i + 2] = CANVAS_COLOR[2]; // blue
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

const update_memory = (chip8) => { 

    if (!debug) { 
        return
    }

    // get an <ul> element
    let ul = document.getElementById("memory_list");
    ul.innerHTML = ''

    let memory = chip8.disassemble_memory();

    memory.forEach(instruction => { 
        let li = document.createElement('li'); 
        li.innerHTML = instruction; 
        ul.appendChild(li); 
    })

    update_state_ui(chip8)
}

const update_state_ui = (chip8) => { 

    // Update the register list <ul> element
    let register_list = document.getElementById("register_list");
    // clear the element of old list elements by removing the inner html
    register_list.innerHTML = ''

    let registers = chip8.get_registers();
    // Apply each string to
    registers.forEach(register => { 
        let li = document.createElement('li'); 
        li.innerHTML = register; 
        register_list.appendChild(li); 
    })

     // Update the other state list <ul> element
     let state_list = document.getElementById("other_state_list");
     // clear the element of old list elements by removing the inner html
     state_list.innerHTML = ''
 
     let other_state = chip8.get_other_state();
     // Apply each string to
     other_state.forEach(register => { 
         let li = document.createElement('li'); 
         li.innerHTML = register; 
         state_list.appendChild(li); 
     })

}



const get_rom_file = async(name) => { 
    let file = await fetch(`roms/${name}`); 
    let buff = await file.arrayBuffer(); 
    return new DataView(buff, 0, buff.byteLength); 
}

const create_rom_selection = () => { 
    const sel_element = document.getElementById('rom-select');
   

    const rom_select = ROMS.map(rom => {
        return `<option value="${rom}">${rom}</option>`;
    });
    sel_element.innerHTML = rom_select;
    return sel_element; 
}

// CHIP-8 Keypad    User Keyboard
// +-+-+-+-+        +-+-+-+-+
// |1|2|3|C|        |1|2|3|4|
// +-+-+-+-+        +-+-+-+-+
// |4|5|6|D|        |Q|W|E|R|
// +-+-+-+-+   <=   +-+-+-+-+
// |7|8|9|E|        |A|S|D|F|
// +-+-+-+-+        +-+-+-+-+
// |A|0|B|F|        |Z|X|C|V|
// +-+-+-+-+        +-+-+-+-+

const CHIP8_KEYBOARD = {

    // KEYBOARD
    '1': 0x1,
    '2': 0x2,
    '3': 0x3,
    '4': 0xc,
  
    'q': 0x4,
    'w': 0x5,
    'e': 0x6,
    'r': 0xd,
  
    'a': 0x7,
    's': 0x8,
    'd': 0x9,
    'f': 0xe,
  
    'z': 0xa,
    'x': 0x0,
    'c': 0xb,
    'v': 0xf,

    
};

// A map of a chip8 keyboard to the textual button on the screen
const KEYPAD_MAP = { 

}

main()