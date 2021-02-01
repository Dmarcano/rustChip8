/* tslint:disable */
/* eslint-disable */
/**
*/
export function greet(): void;
/**
*/
export class WasmChip8 {
  free(): void;
/**
* @returns {WasmChip8}
*/
  static new(): WasmChip8;
/**
* @returns {boolean}
*/
  cycle(): boolean;
/**
*/
  reset(): void;
/**
* @param {number} key
*/
  key_down(key: number): void;
/**
* @param {number} key
*/
  key_up(key: number): void;
/**
* @returns {number}
*/
  get_display(): number;
/**
* @returns {number}
*/
  get_memory(): number;
/**
* @param {DataView} data
*/
  load_rom_js(data: DataView): void;
/**
* @returns {number}
*/
  pc(): number;
/**
* @returns {Array<any>}
*/
  get_registers(): Array<any>;
/**
* @returns {Array<any>}
*/
  get_other_state(): Array<any>;
/**
* @returns {Array<any>}
*/
  disassemble_memory(): Array<any>;
}
