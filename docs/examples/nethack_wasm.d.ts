/* tslint:disable */
/* eslint-disable */

/**
 * Game state wrapper for JavaScript access
 */
export class Game {
    free(): void;
    [Symbol.dispose](): void;
    /**
     * Execute command character (e.g., 'k' for kick, 's' for search)
     */
    execute_command(command: string): void;
    /**
     * Get dungeon height
     */
    height(): number;
    /**
     * Get index count for rendering
     */
    index_count(): number;
    /**
     * Initialize the game (move player to starting position)
     */
    init(): void;
    /**
     * Check if game is running
     */
    is_running(): boolean;
    /**
     * Move player in a direction (dx, dy)
     */
    move_player(dx: number, dy: number): void;
    /**
     * Create a new game instance
     */
    constructor();
    /**
     * Get player X coordinate
     */
    player_x(): number;
    /**
     * Get player Y coordinate
     */
    player_y(): number;
    /**
     * Quit the game
     */
    quit(): void;
    /**
     * Render game state to vertices
     * Returns flattened vertex buffer as Vec<f32> (x, y, z, r, g, b, a for each vertex)
     */
    render(): Float32Array;
    /**
     * Get render indices (triangle indices)
     */
    render_indices(): Uint16Array;
    /**
     * Update game state each frame
     */
    update(): void;
    /**
     * Get vertex count for rendering
     */
    vertex_count(): number;
    /**
     * Get dungeon width
     */
    width(): number;
}

export function create_game(): Game;

/**
 * Get version info
 */
export function get_version(): string;

export function main(): void;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
    readonly memory: WebAssembly.Memory;
    readonly __wbg_game_free: (a: number, b: number) => void;
    readonly create_game: () => number;
    readonly game_execute_command: (a: number, b: number) => void;
    readonly game_height: (a: number) => number;
    readonly game_index_count: (a: number) => number;
    readonly game_init: (a: number) => void;
    readonly game_is_running: (a: number) => number;
    readonly game_move_player: (a: number, b: number, c: number) => void;
    readonly game_player_x: (a: number) => number;
    readonly game_player_y: (a: number) => number;
    readonly game_quit: (a: number) => void;
    readonly game_render: (a: number) => [number, number];
    readonly game_render_indices: (a: number) => [number, number];
    readonly game_update: (a: number) => void;
    readonly game_vertex_count: (a: number) => number;
    readonly game_width: (a: number) => number;
    readonly get_version: () => [number, number];
    readonly main: () => void;
    readonly game_new: () => number;
    readonly __wbindgen_externrefs: WebAssembly.Table;
    readonly __wbindgen_free: (a: number, b: number, c: number) => void;
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
