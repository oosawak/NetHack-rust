/*
 * Minimal wrapper header for NetHack FFI
 * Exposes safe accessor functions for game state
 */

/* Player info structure for safe FFI */
typedef struct {
    int x, y;           /* ux, uy */
    int level;          /* ulevel */
    int hp, max_hp;     /* mh, mhmax */
    int dungeon_level;  /* dlevel */
} player_state_t;

/* Monster data structure for FFI */
typedef struct {
    int x, y;                  /* Position */
    int hp, max_hp;            /* Health */
    int monster_id;            /* Unique ID */
    int symbol;                /* ASCII representation */
    int is_peaceful;           /* 1 if peaceful, 0 if hostile */
} monster_data_t;

/* Object data structure for FFI */
typedef struct {
    int x, y;        /* Position */
    int object_id;   /* Object type ID */
    int symbol;      /* ASCII representation */
} object_data_t;

/* Trap data structure for FFI */
typedef struct {
    int x, y;        /* Position */
    int trap_type;   /* Trap type (1-25) */
    int symbol;      /* ASCII representation */
} trap_data_t;

/* Stairway data structure for FFI */
typedef struct {
    int x, y;        /* Position */
    int is_up;       /* 1 if up, 0 if down */
    int is_ladder;   /* 1 if ladder, 0 if stairs */
    int symbol;      /* ASCII representation */
} stair_data_t;

/* Safe accessor functions for player state */
int get_player_x(void);
int get_player_y(void);
int get_player_level(void);
int get_player_hp(void);
int get_player_maxhp(void);

/* Bulk player state accessor */
void get_player_state(player_state_t *state);

/* Dungeon level accessors */
int get_dlevel(void);
int get_dunlevs(void);

/* Monster accessor functions */
int get_monster_count(void);
int get_monster_by_index(int index, monster_data_t *out);

/* Object accessor functions */
int get_object_count(void);
int get_object_by_index(int index, object_data_t *out);

/* Trap accessor functions */
int get_trap_count(void);
int get_trap_by_index(int index, trap_data_t *out);

/* Stairway accessor functions */
int get_stair_count(void);
int get_stair_by_index(int index, stair_data_t *out);
