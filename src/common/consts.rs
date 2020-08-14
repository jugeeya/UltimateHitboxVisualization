use crate::common::get_random_int;
use core::f64::consts::PI;
use smash::lib::lua_const::*;

/// Hitbox Visualization
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum HitboxVisualization {
    Off = 0,
    On = 1,
}

// DI
/*
 0, 0.785398, 1.570796, 2.356194, -3.14159, -2.356194,  -1.570796, -0.785398
 0, pi/4,     pi/2,     3pi/4,    pi,       5pi/4,      3pi/2,     7pi/4
*/

/// DI
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Direction {
    None = 0,
    Right = 1,
    UpRight = 2,
    Up = 3,
    UpLeft = 4,
    Left = 5,
    DownLeft = 6,
    Down = 7,
    DownRight = 8,
    // lol what goes here jug smh my head
    Random = 9,
}

impl From<i32> for Direction {
    fn from(x: i32) -> Self {
        match x {
            0 => Direction::None,
            1 => Direction::Right,
            2 => Direction::UpRight,
            3 => Direction::Up,
            4 => Direction::UpLeft,
            5 => Direction::Left,
            6 => Direction::DownLeft,
            7 => Direction::Down,
            8 => Direction::DownRight,
            9 => Direction::Random,
            _ => panic!("Invalid direction {}", x),
        }
    }
}

pub static ANGLE_NONE: f64 = -69.0;
pub fn direction_to_angle(direction: Direction) -> f64 {
    match direction {
        Direction::None => ANGLE_NONE,
        Direction::Random => ANGLE_NONE, // Random Direction should be handled by the calling context
        _ => (direction as i32 - 1) as f64 * PI / 4.0,
    }
}

/// Mash Attack States
#[repr(i32)]
#[derive(PartialEq, Debug, Copy, Clone)]
pub enum Attack {
    Nair = 0,
    Fair = 1,
    Bair = 2,
    UpAir = 3,
    Dair = 4,
    NeutralB = 5,
    SideB = 6,
    UpB = 7,
    DownB = 8,
    UpSmash = 9,
    FSmash = 10,
    DSmash = 11,
    Grab = 12,
    Jab = 13,
    Ftilt = 14,
    Utilt = 15,
    Dtilt = 16,
    DashAttack = 17,
    Nothing = 9999,
}

impl From<i32> for Attack {
    fn from(x: i32) -> Self {
        use Attack::*;

        match x {
            0 => Nair,
            1 => Fair,
            2 => Bair,
            3 => UpAir,
            4 => Dair,
            5 => NeutralB,
            6 => SideB,
            7 => UpB,
            8 => DownB,
            9 => UpSmash,
            10 => FSmash,
            11 => DSmash,
            12 => Grab,
            13 => Jab,
            14 => Ftilt,
            15 => Utilt,
            16 => Dtilt,
            17 => DashAttack,
            _ => Nothing,
        }
    }
}

// bitflag helper function macro
macro_rules! to_vec_impl {
    ($e:ty) => {
        pub fn to_vec(&self) -> Vec::<$e> {
            let mut vec = Vec::<$e>::new();
            let mut field = <$e>::from_bits_truncate(self.bits);
            while !field.is_empty() {
                let flag = <$e>::from_bits(1u32 << field.bits.trailing_zeros()).unwrap();
                field -= flag;
                vec.push(flag);
            }
            return vec;
        }
    }
}

macro_rules! get_random_impl {
    ($e:ty) => {
        pub fn get_random(&self) -> $e {
            let options = self.to_vec();
            match options.len() {
                0 => {
                    return <$e>::empty();
                }
                1 => {
                    return options[0];
                }
                _ => {
                    return *random_option(&options);
                }
            }
        }
    }
}

// Ledge Option
bitflags! {
    pub struct LedgeOption : u32
    {
        const NEUTRAL = 0x1;
        const ROLL = 0x2;
        const JUMP = 0x4;
        const ATTACK = 0x8;
    }
}

pub fn random_option<T>(arg: &Vec<T>) -> &T {
    return &arg[get_random_int(arg.len() as i32) as usize];
}

impl LedgeOption {
    pub fn into_status(&self) -> Option<i32> {
        Some(match *self {
            LedgeOption::NEUTRAL => *FIGHTER_STATUS_KIND_CLIFF_CLIMB,
            LedgeOption::ROLL => *FIGHTER_STATUS_KIND_CLIFF_ESCAPE,
            LedgeOption::JUMP => *FIGHTER_STATUS_KIND_CLIFF_JUMP1,
            LedgeOption::ATTACK => *FIGHTER_STATUS_KIND_CLIFF_ATTACK,
            _ => return None,
        })
    }
    to_vec_impl! {LedgeOption}
    get_random_impl! {LedgeOption}
}

// Tech options
bitflags! {
    pub struct TechFlags : u32 {
        const NO_TECH = 0x1;
        const ROLL_F = 0x2;
        const ROLL_B = 0x4;
        const IN_PLACE = 0x8;
    }
}

impl TechFlags {
    to_vec_impl! {TechFlags}
    get_random_impl! {TechFlags}
}

/// Mash States
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Mash {
    None = 0,
    Airdodge = 1,
    Jump = 2,
    Attack = 3,
    Spotdodge = 4,
    RollForward = 5,
    RollBack = 6,
    Random = 7,
    Shield = 99,
}

impl From<i32> for Mash {
    fn from(x: i32) -> Self {
        match x {
            0 => Mash::None,
            1 => Mash::Airdodge,
            2 => Mash::Jump,
            3 => Mash::Attack,
            4 => Mash::Spotdodge,
            5 => Mash::RollForward,
            6 => Mash::RollBack,
            7 => Mash::Random,
            _ => panic!("Invalid mash state {}", x),
        }
    }
}

/// Shield States
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Shield {
    None = 0,
    Infinite = 1,
    Hold = 2,
}

// Defensive States
bitflags! {
    pub struct Defensive : u32 {
        const SPOT_DODGE = 0x1;
        const ROLL_F = 0x2;
        const ROLL_B = 0x4;
        const JAB = 0x8;
        const SHIELD = 0x10;
    }
}

impl Defensive {
    to_vec_impl! {Defensive}
    get_random_impl! {Defensive}
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum OnOff {
    Off = 0,
    On = 1,
}

bitflags! {
    pub struct Action : u32 {
        const AIR_DODGE = 0x1;
        const JUMP = 0x2;
        const SHIELD = 0x4;
        const SPOT_DODGE = 0x8;
        const ROLL_F = 0x10;
        const ROLL_B = 0x20;
        const NAIR = 0x40;
        const FAIR = 0x80;
        const BAIR = 0x100;
        const UAIR = 0x200;
        const DAIR = 0x400;
        const NEUTRAL_B = 0x800;
        const SIDE_B = 0x1000;
        const UP_B = 0x2000;
        const DOWN_B = 0x4000;
        const F_SMASH = 0x8000;
        const U_SMASH = 0x10000;
        const D_SMASH = 0x20000;
        const JAB = 0x40000;
        const F_TILT = 0x80000;
        const U_TILT  = 0x100000;
        const D_TILT  = 0x200000;
        const DASH_ATTACK = 0x400000;
        const GRAB = 0x800000;
    }
}

impl Action {
    pub fn into_attack_air_kind(&self) -> Option<i32> {
        Some(match *self {
            Action::NAIR => *FIGHTER_COMMAND_ATTACK_AIR_KIND_N,
            Action::FAIR => *FIGHTER_COMMAND_ATTACK_AIR_KIND_F,
            Action::BAIR => *FIGHTER_COMMAND_ATTACK_AIR_KIND_B,
            Action::DAIR => *FIGHTER_COMMAND_ATTACK_AIR_KIND_LW,
            Action::UAIR => *FIGHTER_COMMAND_ATTACK_AIR_KIND_HI,
            _ => return None,
        })
    }
    to_vec_impl! {Action}
    get_random_impl! {Action}
}

#[repr(C)]
pub struct TrainingModpackMenu {
    pub hitbox_vis: HitboxVisualization,
    pub di_state: Direction,
    pub left_stick: Direction, // Currently only used for air dodge direction
    pub mash_attack_state: Attack,
    pub follow_up: Action,
    pub ledge_state: LedgeOption,
    pub tech_state: TechFlags,
    pub mash_state: Mash,
    pub shield_state: Shield,
    pub defensive_state: Defensive,
    pub oos_offset: u32,
    pub reaction_time: u32,
    pub mash_in_neutral: OnOff,
    pub fast_fall: OnOff,
    pub fast_fall_delay: u32,
    pub falling_aerials: OnOff,
    pub full_hop: OnOff,
}

// Fighter Ids
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FighterId {
    Player = 0,
    CPU = 1,
}
