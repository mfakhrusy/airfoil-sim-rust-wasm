/* tslint:disable */
/* eslint-disable */
export function parse_airfoil_dat(data: string, name: string): Airfoil;
export function greet(): void;
export function greet_console(): void;
export function get_greeting(): string;
export function add(a: number, b: number): number;
export class Airfoil {
  free(): void;
  constructor(name: string);
  get_point(index: number): AirfoilPoint | undefined;
  get_points_as_arrays(): object;
  add_point(x: number, y: number): void;
  get_bounds(): object;
  readonly name: string;
  readonly point_count: number;
}
export class AirfoilPoint {
  free(): void;
  constructor(x: number, y: number);
  readonly x: number;
  readonly y: number;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly __wbg_airfoilpoint_free: (a: number, b: number) => void;
  readonly airfoilpoint_new: (a: number, b: number) => number;
  readonly airfoilpoint_x: (a: number) => number;
  readonly airfoilpoint_y: (a: number) => number;
  readonly __wbg_airfoil_free: (a: number, b: number) => void;
  readonly airfoil_new: (a: number, b: number) => number;
  readonly airfoil_name: (a: number) => [number, number];
  readonly airfoil_point_count: (a: number) => number;
  readonly airfoil_get_point: (a: number, b: number) => number;
  readonly airfoil_get_points_as_arrays: (a: number) => any;
  readonly airfoil_add_point: (a: number, b: number, c: number) => void;
  readonly airfoil_get_bounds: (a: number) => any;
  readonly parse_airfoil_dat: (a: number, b: number, c: number, d: number) => [number, number, number];
  readonly greet: () => void;
  readonly greet_console: () => void;
  readonly get_greeting: () => [number, number];
  readonly add: (a: number, b: number) => number;
  readonly __wbindgen_exn_store: (a: number) => void;
  readonly __externref_table_alloc: () => number;
  readonly __wbindgen_export_2: WebAssembly.Table;
  readonly __wbindgen_malloc: (a: number, b: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
  readonly __wbindgen_free: (a: number, b: number, c: number) => void;
  readonly __externref_table_dealloc: (a: number) => void;
  readonly __wbindgen_start: () => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;
/**
* Instantiates the given `module`, which can either be bytes or
* a precompiled `WebAssembly.Module`.
*
* @param {{ module: SyncInitInput }} module - Passing `SyncInitInput` directly is deprecated.
*
* @returns {InitOutput}
*/
export function initSync(module: { module: SyncInitInput } | SyncInitInput): InitOutput;

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {{ module_or_path: InitInput | Promise<InitInput> }} module_or_path - Passing `InitInput` directly is deprecated.
*
* @returns {Promise<InitOutput>}
*/
export default function __wbg_init (module_or_path?: { module_or_path: InitInput | Promise<InitInput> } | InitInput | Promise<InitInput>): Promise<InitOutput>;
