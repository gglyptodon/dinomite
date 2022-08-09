/* tslint:disable */
/* eslint-disable */
/**
* @returns {string}
*/
export function getState(): string;
/**
* @param {number} width
* @param {number} height
* @param {number} num_dinos
*/
export function newGame(width: number, height: number, num_dinos: number): void;
/**
*/
export function newGameDefault(): void;
/**
* @param {number} x
* @param {number} y
*/
export function toggleFlag(x: number, y: number): void;
/**
* @param {number} x
* @param {number} y
*/
export function checkPosition(x: number, y: number): void;
/**
* @param {number} x
* @param {number} y
*/
export function checkNeighbors(x: number, y: number): void;
/**
* @returns {boolean}
*/
export function isGameOver(): boolean;
/**
* @returns {boolean}
*/
export function isWon(): boolean;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly getState: (a: number) => void;
  readonly newGame: (a: number, b: number, c: number) => void;
  readonly newGameDefault: () => void;
  readonly toggleFlag: (a: number, b: number) => void;
  readonly checkPosition: (a: number, b: number) => void;
  readonly checkNeighbors: (a: number, b: number) => void;
  readonly isGameOver: () => number;
  readonly isWon: () => number;
  readonly __wbindgen_add_to_stack_pointer: (a: number) => number;
  readonly __wbindgen_free: (a: number, b: number) => void;
  readonly __wbindgen_exn_store: (a: number) => void;
}

/**
* Synchronously compiles the given `bytes` and instantiates the WebAssembly module.
*
* @param {BufferSource} bytes
*
* @returns {InitOutput}
*/
export function initSync(bytes: BufferSource): InitOutput;

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {InitInput | Promise<InitInput>} module_or_path
*
* @returns {Promise<InitOutput>}
*/
export default function init (module_or_path?: InitInput | Promise<InitInput>): Promise<InitOutput>;
