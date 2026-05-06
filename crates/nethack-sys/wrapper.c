/*
 * wrapper.c - FFI wrapper functions for safe access to NetHack globals
 *
 * These functions provide safe, simple accessors to C structs that would be
 * complex to expose directly to Rust via FFI.
 *
 * NOTE: We use forward declarations instead of including hack.h
 * to avoid conflicts with the wrapper.h definitions.
 */

#include <stddef.h>

/* Forward declarations of NetHack structures */
typedef short coordxy;

typedef struct {
    int dungeon_num, depth_num;
} d_level;

typedef struct {
    coordxy ux, uy;
    d_level uz, uz0;
    int ulevel;
    int mh, mhmax;
    /* ... other fields not needed ... */
} struct_you;

typedef struct {
    short mnum;
    /* ... */
} struct_role;

typedef struct {
    int x, y;
} coord;

/* FFI types from wrapper.h */
typedef struct {
    int x, y;
    int level;
    int hp, max_hp;
    int dungeon_level;
} player_state_t;

/* External globals from NetHack */
extern struct_you u;
extern int dlevel;

/* Access player X position */
int get_player_x(void) {
    return (int)u.ux;
}

/* Access player Y position */
int get_player_y(void) {
    return (int)u.uy;
}

/* Access player level */
int get_player_level(void) {
    return u.ulevel;
}

/* Access player current HP */
int get_player_hp(void) {
    return u.mh;
}

/* Access player max HP */
int get_player_maxhp(void) {
    return u.mhmax;
}

/* Bulk read of player state */
void get_player_state(player_state_t *state) {
    if (state == NULL) {
        return;
    }
    
    state->x = (int)u.ux;
    state->y = (int)u.uy;
    state->level = u.ulevel;
    state->hp = u.mh;
    state->max_hp = u.mhmax;
    state->dungeon_level = dlevel;
}
