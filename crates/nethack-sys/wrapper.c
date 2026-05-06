/*
 * wrapper.c - FFI wrapper functions for safe access to NetHack globals
 *
 * These functions provide safe, simple accessors to C structs that would be
 * complex to expose directly to Rust via FFI.
 */

#include "config.h"
#include "hack.h"
#include "monst.h"
#include "obj.h"
#include "rm.h"
#include "decl.h"
#include <unistd.h>
#include <pwd.h>
#include <stdlib.h>
#include <string.h>

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

/* Access player HP */
int get_player_hp(void) {
    return u.mh;
}

/* Access player max HP */
int get_player_maxhp(void) {
    return u.mhmax;
}

/* Bulk player state accessor */
typedef struct {
    int x, y;
    int level;
    int hp, max_hp;
    int dungeon_level;
} player_state_t;

void get_player_state(player_state_t *state) {
    if (!state) return;
    state->x = (int)u.ux;
    state->y = (int)u.uy;
    state->level = u.ulevel;
    state->hp = u.mh;
    state->max_hp = u.mhmax;
    state->dungeon_level = 1;  /* Default to level 1 - actual level tracking TODO */
}

/* Access dungeon level (placeholder) */
int get_dlevel(void) {
    return 1;  /* Placeholder - actual dlevel tracking TODO */
}

/* Access total dungeon levels (placeholder) */
int get_dunlevs(void) {
    return 50;  /* Placeholder - actual dunlevs tracking TODO */
}

/* FFI structure for passing monster data */
typedef struct {
    int x, y;                  /* Position */
    int hp, max_hp;            /* Health */
    int monster_id;            /* Unique ID */
    int symbol;                /* ASCII representation */
    int is_peaceful;           /* 1 if peaceful, 0 if hostile */
} monster_data_t;

/* FFI structure for passing object data */
typedef struct {
    int x, y;        /* Position */
    int object_id;   /* Object type ID */
    int symbol;      /* ASCII representation */
} object_data_t;

/* Get monster count (scan fmon list) */
int get_monster_count(void) {
    struct monst *m = svl.level.monlist;
    int count = 0;
    while (m != NULL) {
        count++;
        m = m->nmon;
    }
    return count;
}

/* Get monster data by iteration index (0 to count-1) */
int get_monster_by_index(int index, monster_data_t *out) {
    if (!out) return -1;
    
    struct monst *m = svl.level.monlist;
    int count = 0;
    
    while (m != NULL && count < index) {
        count++;
        m = m->nmon;
    }
    
    if (m == NULL) return 0;  /* Not found */
    
    out->x = (int)m->mx;
    out->y = (int)m->my;
    out->hp = m->mhp;
    out->max_hp = m->mhpmax;
    out->monster_id = m->mnum;
    out->symbol = (int)'m';  /* Default symbol */
    out->is_peaceful = (m->mpeaceful != 0) ? 1 : 0;
    
    return 1;  /* Success */
}

/* Get object count (scan fobj list for floor objects) */
int get_object_count(void) {
    struct obj *o = fobj;
    int count = 0;
    
    while (o != NULL) {
        /* Count only objects on the floor, not in containers or inventory */
        if (o->where == OBJ_FLOOR) {
            count++;
        }
        o = o->nobj;
    }
    
    return count;
}

/* Get object data by iteration index (returns floor objects only) */
int get_object_by_index(int index, object_data_t *out) {
    if (!out) return -1;
    
    struct obj *o = fobj;
    int count = 0;
    
    while (o != NULL) {
        if (o->where == OBJ_FLOOR) {
            if (count == index) {
                /* Found the object at the requested index */
                out->x = (int)o->ox;
                out->y = (int)o->oy;
                out->object_id = o->otyp;  /* Use object type as ID */
                out->symbol = (int)'*';    /* Generic item symbol */
                
                return 1;  /* Success */
            }
            count++;
        }
        o = o->nobj;
    }
    
    return 0;  /* Not found */
}

/* Wrapper for chdirx - change working directory
 * Called by earlyarg.c when processing options.
 * The static version in libnhmain.c isn't exported, so we provide a wrapper here. */
void chdirx(const char *dir, boolean wr) {
    if (!dir) return;
    (void)wr;  /* unused */
    
    if (chdir(dir) == -1) {
        /* Silently fail - directory might not exist or no permission */
    }
}

/* Wrapper for whoami - get current user name
 * Called by libnhmain.c and other files.
 * The local version in libnhmain.c isn't exported, so we provide a wrapper here. */
char *whoami(void) {
    static char buf[256];
    struct passwd *pw = getpwuid(getuid());
    
    if (pw && pw->pw_name) {
        strncpy(buf, pw->pw_name, sizeof(buf) - 1);
        buf[sizeof(buf) - 1] = '\0';
        return buf;
    }
    
    /* Fallback to environment or default */
    const char *user = getenv("USER");
    if (user) {
        strncpy(buf, user, sizeof(buf) - 1);
        buf[sizeof(buf) - 1] = '\0';
        return buf;
    }
    
    strcpy(buf, "player");
    return buf;
}
