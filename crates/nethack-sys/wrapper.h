/*
 * Minimal wrapper header for NetHack FFI
 * Exposes essential initialization and game loop functions
 */

/* Basic types from NetHack */
typedef struct coord {
    int x, y;
} coord;

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
