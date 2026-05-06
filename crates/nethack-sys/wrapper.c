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
#include "trap.h"
#include "stairs.h"
#include "wrapper.h"
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

/* Get trap count on current level */
int get_trap_count(void) {
    struct trap *t = gf.ftrap;
    int count = 0;
    
    while (t != NULL) {
        count++;
        t = t->ntrap;
    }
    
    return count;
}

/* Get trap data by iteration index */
int get_trap_by_index(int index, trap_data_t *out) {
    if (!out) return -1;
    
    struct trap *t = gf.ftrap;
    int count = 0;
    
    while (t != NULL) {
        if (count == index) {
            /* Found the trap at the requested index */
            out->x = (int)t->tx;
            out->y = (int)t->ty;
            out->trap_type = (int)t->ttyp;
            out->symbol = (int)'^';  /* Generic trap symbol */
            
            return 1;  /* Success */
        }
        count++;
        t = t->ntrap;
    }
    
    return 0;  /* Not found */
}

/* Get stairway count on current level */
int get_stair_count(void) {
    stairway *s = gs.stairs;
    int count = 0;
    
    while (s != NULL) {
        count++;
        s = s->next;
    }
    
    return count;
}

/* Get stairway data by iteration index */
int get_stair_by_index(int index, stair_data_t *out) {
    if (!out) return -1;
    
    stairway *s = gs.stairs;
    int count = 0;
    
    while (s != NULL) {
        if (count == index) {
            /* Found the stairway at the requested index */
            out->x = (int)s->sx;
            out->y = (int)s->sy;
            out->is_up = s->up ? 1 : 0;
            out->is_ladder = s->isladder ? 1 : 0;
            out->symbol = s->up ? (int)'<' : (int)'>';  /* < for up, > for down */
            
            return 1;  /* Success */
        }
        count++;
        s = s->next;
    }
    
    return 0;  /* Not found */
}
