/*
 * Minimal wrapper header for NetHack FFI
 * Exposes essential initialization and game loop functions
 */

/* Basic types from NetHack */
typedef struct coord {
    int x, y;
} coord;

/* Player info structure for safe FFI */
typedef struct {
    int x, y;           /* ux, uy */
    int level;          /* ulevel */
    int hp, max_hp;     /* mh, mhmax */
    int dungeon_level;  /* dlevel */
} player_state_t;

/* Game initialization functions (from unixmain.c flow) */
extern void early_init(void);
extern void choose_windows(int);
extern void initoptions(void);
extern void init_nhwindows(int *, char **);
extern void dlb_init(void);
extern void vision_init(void);
extern void init_sound_disp_gamewindows(void);

/* Game functions */
extern void newgame(void);
extern void moveloop(int);
extern int docommand(void);

/* Save/restore */
extern void getlock(void);
extern void player_selection(void);

/* Global variables */
extern int dlevel;
extern int dunlevs;

/* Opaque types (not defining structure details) */
struct you;
struct monst;
struct obj;
struct dungeon_topology;
struct window_procs;

extern struct you u;
extern struct dungeon_topology dungeon;
extern struct obj *fobj;
extern struct monst *fmon;
extern struct window_procs *windowprocs;

/* Safe accessor functions for player state */
/* These are implemented in a companion C file to safely read struct you */
int get_player_x(void);
int get_player_y(void);
int get_player_level(void);
int get_player_hp(void);
int get_player_maxhp(void);

/* Bulk player state accessor */
void get_player_state(player_state_t *state);

