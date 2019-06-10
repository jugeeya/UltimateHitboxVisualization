#ifndef TAUNT_TOGGLES_H
#define TAUNT_TOGGLES_H

#define NONE 0

// Up Taunt
bool HITBOX_VIS = 1;

// Side Taunt
// 0, 0.785398, 1.570796, 2.356194, -3.14159, -2.356194,  -1.570796, -0.785398
// 0, pi/4,     pi/2,     3pi/4,    pi,       5pi/4,      3pi/2,     7pi/4

int DI_STATE = 0;
#define DI_RANDOM_IN_AWAY 9
#define NUM_DI_STATES 10

// Down Taunt
#define MASH_AIRDODGE 1
#define MASH_JUMP 2
#define INFINITE_SHIELD 3
#define HOLD_SHIELD 4
#define LEDGE_OPTION 5

int TOGGLE_STATE = 0;
#define NUM_TOGGLE_STATES 6

#endif // TAUNT_TOGGLES_H
