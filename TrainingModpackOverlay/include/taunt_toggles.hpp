#pragma once

#define NONE 0

#include <vector>

#include "cpp_utils.hpp"

const std::vector<std::string> on_off{"Off", "On"};
// clang-format off
#define ENUM_CLASS_OnOffFlag(type,x) \
    x(type,On,"On")

// clang-format on
DEFINE_ENUM_CLASS(OnOffFlag);

// Frame Advantage
const std::vector<std::string> frame_advantage_items{""};
const std::string              frame_advantage_help = R""""(
Show frame advantage when
hitting a CPU's shield.

This is comparing true frames
of actionability between the
CPU and player.

Use this to practice optimal
aerial heights and specific
safe hitboxes.)"""";

// Side Taunt

// DI / Left Stick
/*
 0, 0.785398, 1.570796, 2.356194, -3.14159, -2.356194,  -1.570796, -0.785398
 0, pi/4,     pi/2,     3pi/4,    pi,       5pi/4,      3pi/2,     7pi/4
*/

/* DI */
// clang-format off
#define ENUM_CLASS_Direction(type,x) \
    x(type,Out,"Out") \
    x(type,UpOut,"Up Out") \
    x(type,Up,"Up") \
    x(type,UpIn,"Up In") \
    x(type,In,"In") \
    x(type,DownIn,"Down In") \
    x(type,Down,"Down") \
    x(type,DownOut,"Down Out")\
    x(type,Nothing,"Neutral")

DEFINE_ENUM_CLASS(Direction);
const std::string              di_help = R""""(
Specified Direction
CPUs DI in the direction specified
(relative to the player's facing
position).
)"""";

const std::string              sdi_help = R""""(
Specified Direction
CPUs SDI in the direction specified
(relative to the player's facing
position) every four frames
during hitlag.
)"""";

// Left Stick
const std::string left_stick_help = R""""(
Specified Direction
CPUs left stick will be
in the direction specified
(relative to the player's facing
position).

Currently only used for
- Airdodge

)"""";

// Ledge Option
// clang-format off
#define ENUM_CLASS_LedgeFlag(type,x) \
    x(type,Neutral,"Neutral") \
    x(type,Roll,"Roll") \
    x(type,Jump,"Jump") \
    x(type,Attack,"Attack")

// clang-format on

DEFINE_ENUM_CLASS(LedgeFlag);
const std::string ledge_help = R""""(
CPUs will perform a ledge option
among the selected options.

CPUs will also perform a defensive
option after getting up.
)"""";

const std::string ledge_delay_help = R""""(
Frames to delay ledge option.)"""";

// Tech Option

// clang-format off
#define ENUM_CLASS_TechFlag(type,x) \
	x(type,Miss,"Miss Tech") \
	x(type,RollF,"RollF") \
	x(type,RollB,"RollB") \
	x(type,InPlace,"In Place")

// clang-format on
DEFINE_ENUM_CLASS(TechFlag);

constexpr const char* const tech_help = R""""(
CPUs will perform a random
tech option among the selected
options.

CPUs will also perform a defensive
option after getting up.)"""";

// Defensive States

// clang-format off
#define ENUM_CLASS_DefensiveFlag(type,x) \
	x(type,SpotDodge,"Spotdodge") \
	x(type,RollF,"RollF") \
	x(type,RollB,"RollB") \
	x(type,Jab,"Jab")\
	x(type,Shield,"Shield")

// clang-format on
DEFINE_ENUM_CLASS(DefensiveFlag);

const std::string defensive_help = R""""(
Choose the defensive option a CPU
will perform after teching or
getting up from the ledge.

The option will be random
among the selected options.
)"""";

// Mash States
const std::string mash_help = R""""(
Use this toggle along with the Shield
Options toggle to practice moves on
shield.

The option will be random
among the selected options.

CPUs will mash on the first frame out
of hitstun or out of specific states.
)"""";

// Action items (Follow Up only atm)

// clang-format off
#define ENUM_CLASS_ActionFlag(type,x) \
	x(type,Airdodge,"Airdodge") x(type,Jump,"Jump") x(type,Shield,"Shield") x(type,Spotdodge,"Spotdodge") x(type,RollF,"Roll F") x(type,RollB,"Roll B")  \
    x(type,Nair,"Neutral Air") x(type,Fair,"Forward Air") x(type,Bair,"Back Air") x(type,Uair,"Up Air") x(type,Dair,"Down Air")  \
    x(type,NeutralB,"Neutral B") x(type,SideB,"Side B") x(type,UpB,"Up B") x(type,DownB,"Down B") \
    x(type,FSmash,"Forward Smash") x(type,USmash,"Up Smash") x(type,DSmash,"Down Smash") \
    x(type,Jab,"Jab") x(type,FTilt,"Ftilt") x(type,UTilt,"Utilt") x(type,Dtilt,"Dtilt")  \
    x(type,Grab,"Grab") //x(type,DashAttack,"Dash Attack")

// clang-format on
DEFINE_ENUM_CLASS(ActionFlag);

const std::string follow_up_help = R""""(
The selected action will be
buffered after the specified
mash option.

The option will be random
among the selected options.
)"""";

// Shield States
#define SHIELD_INFINITE 1
#define SHIELD_HOLD 2
const std::vector<std::string> shield_items{"None", "Infinite", "Hold"};
const std::string              shield_help = R""""(
Use these toggles in conjunction
with Mash toggles to practice
moves on shield.

Infinite
CPUs will hold a shield that does
not deteriorate over time or
by damage.

Hold
CPUs will hold a normal shield.
This shield will not deteriorate
until hit once.)"""";

// Hitbox visualization
const std::string hitbox_help = R""""(
Currently, hitboxes and
grabboxes are supported.

Original move effects are
paused during normal attacks
and specials when hitbox
visualization is active.)"""";

// Save states
const std::vector<std::string> empty_items{""};
const std::string              save_states_help = R""""(
Press Grab + Down Taunt at any
time to save the state of the
training mode for you and the
CPU.

Press Grab + Up Taunt at any
time to revert to a
previously saved state.

The following attributes
are saved:
- Percent
- Position
- Facing direction)"""";

const std::string              reset_menu_help = R""""(
Reset menu to default
configuration. Please also
use on the first boot after
upgrading Training Modpack
versions.)"""";

// OOS
const std::string oos_help = R""""(
Option to delay oos options
until a certain number of hits
have connected.

Consecutive hits that keep the
CPU locked in shield stun
between hits will count
as a single hit.)"""";

const std::string reaction_time_help = R""""(
Additional reaction time
in frames.

Used to delay OOS Options.)"""";

// Mash in neutral
const std::string mash_neutral_help = R""""(
Force mash options to
always occur, not just
out of specific states.)"""";

// clang-format off
#define ENUM_CLASS_DelayFlag(type,x) \
	x(type,D0,"0") \
	x(type,D1,"1") x(type,D2,"2") x(type,D3,"3") x(type,D4,"4") x(type,D5,"5") \
	x(type,D6,"6") x(type,D7,"7") x(type,D8,"8") x(type,D9,"9") x(type,D10,"10") \
	x(type,D11,"11") x(type,D12,"12") x(type,D13,"13") x(type,D14,"14") x(type,D15,"15") \
	x(type,D16,"16") x(type,D17,"17") x(type,D18,"18") x(type,D19,"19") x(type,D20,"20")
// clang-format on

DEFINE_ENUM_CLASS(DelayFlag);

#define ENUM_CLASS_BoolFlag(type,x) \
	x(type,True,"True") x(type,False,"False")

DEFINE_ENUM_CLASS(BoolFlag);

const std::string fast_fall_help = R""""(
CPUs will fast fall
out of jumps and aerial
attacks as soon as possible.)"""";

const std::string fast_fall_delay_help = R""""(
Frames to delay CPU
fast fall.)"""";

const std::string falling_aerials_help = R""""(
CPUs will only begin
aerials at the apex of
their jump.)"""";

const std::string aerial_delay_help = R""""(
Frames to delay CPU Aerial.)"""";

const std::string full_hop_help = R""""(
CPUs will full hop
rather than short hop
aerials.)"""";