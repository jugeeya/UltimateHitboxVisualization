use crate::common::consts::*;
use crate::common::*;
use crate::training::mash;
use smash::app::sv_system;
use smash::app::{self, lua_bind::*};
use smash::hash40;
use smash::lib::lua_const::*;
use smash::lib::L2CValue;
use smash::lua2cpp::L2CFighterBase;

#[skyline::hook(replace = smash::lua2cpp::L2CFighterBase_change_status)]
pub unsafe fn handle_change_status(
    fighter: &mut L2CFighterBase,
    status_kind: L2CValue,
    unk: L2CValue,
) -> L2CValue {
    let mut status_kind = status_kind;
    let mut unk = unk;
    mod_handle_change_status(fighter, &mut status_kind, &mut unk);

    original!()(fighter, status_kind, unk)
}

unsafe fn mod_handle_change_status(
    fighter: &mut L2CFighterBase,
    status_kind: &mut L2CValue,
    unk: &mut L2CValue,
) {
    if !is_training_mode() {
        return;
    }

    let module_accessor = sv_system::battle_object_module_accessor(fighter.lua_state_agent);
    if !is_operation_cpu(module_accessor) {
        return;
    }

    if MENU.tech_state == TechOption::None {
        return;
    }

    if MENU.tech_state == TechOption::Miss {
        return;
    }

    let status_kind_int = status_kind
        .try_get_int()
        .unwrap_or(*FIGHTER_STATUS_KIND_WAIT as u64) as i32;

    // Ground Tech
    if status_kind_int == FIGHTER_STATUS_KIND_DOWN
        || status_kind_int == FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_D
    {
        match MENU.tech_state {
            TechOption::Random => {
                let random_statuses = vec![
                    *FIGHTER_STATUS_KIND_DOWN,
                    *FIGHTER_STATUS_KIND_PASSIVE,
                    *FIGHTER_STATUS_KIND_PASSIVE_FB,
                ];

                let random_status_index =
                    app::sv_math::rand(hash40("fighter"), random_statuses.len() as i32) as usize;
                if random_statuses[random_status_index] != FIGHTER_STATUS_KIND_DOWN {
                    *status_kind = L2CValue::new_int(random_statuses[random_status_index] as u64);
                    *unk = LUA_TRUE
                }
            }
            TechOption::InPlace => {
                *status_kind = FIGHTER_STATUS_KIND_PASSIVE.as_lua_int();
                *unk = LUA_TRUE;
            }
            TechOption::Roll => {
                *status_kind = FIGHTER_STATUS_KIND_PASSIVE_FB.as_lua_int();
                *unk = LUA_TRUE;
            }
            _ => (),
        }

        mash::perform_defensive_option();

        return;
    }

    // Wall Tech
    if status_kind_int == FIGHTER_STATUS_KIND_STOP_WALL
        || status_kind_int == FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_LR
    {
        *status_kind = FIGHTER_STATUS_KIND_PASSIVE_WALL.as_lua_int();
        *unk = LUA_TRUE;
        return;
    }

    // Ceiling Tech
    if status_kind_int == FIGHTER_STATUS_KIND_STOP_CEIL
        || status_kind_int == FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_U
    {
        *status_kind = FIGHTER_STATUS_KIND_PASSIVE_CEIL.as_lua_int();
        *unk = LUA_TRUE;
        return;
    }
}

pub unsafe fn get_command_flag_cat(
    module_accessor: &mut app::BattleObjectModuleAccessor,
    _category: i32,
) {
    if !is_training_mode() {
        return;
    }

    if !is_operation_cpu(module_accessor) {
        return;
    }

    if MENU.tech_state == TechOption::None {
        return;
    }

    let status = StatusModule::status_kind(module_accessor) as i32;

    if [
        *FIGHTER_STATUS_KIND_DOWN_WAIT,
        *FIGHTER_STATUS_KIND_DOWN_WAIT_CONTINUE,
    ]
    .contains(&status)
    {
        let random_statuses = vec![
            *FIGHTER_STATUS_KIND_DOWN_STAND,        // Normal Getup
            *FIGHTER_STATUS_KIND_DOWN_STAND_FB,     // Getup Roll
            *FIGHTER_STATUS_KIND_DOWN_STAND_ATTACK, // Getup Attack
        ];

        let random_status_index =
            app::sv_math::rand(hash40("fighter"), random_statuses.len() as i32) as usize;
        StatusModule::change_status_request_from_script(
            module_accessor,
            random_statuses[random_status_index],
            false,
        );
        return;
    }
}

pub unsafe fn change_motion(
    module_accessor: &mut app::BattleObjectModuleAccessor,
    motion_kind: u64,
) -> Option<u64> {
    if !is_training_mode() {
        return None;
    }

    if !is_operation_cpu(module_accessor) {
        return None;
    }

    if MENU.tech_state == TechOption::None {
        return None;
    }

    let random_roll = app::sv_math::rand(hash40("fighter"), 2);

    if [hash40("passive_stand_f"), hash40("passive_stand_b")].contains(&motion_kind) {
        if random_roll != 0 {
            return Some(hash40("passive_stand_f"));
        } else {
            return Some(hash40("passive_stand_b"));
        }
    } else if [hash40("down_forward_u"), hash40("down_back_u")].contains(&motion_kind) {
        if random_roll != 0 {
            return Some(hash40("down_forward_u"));
        } else {
            return Some(hash40("down_back_u"));
        }
    } else if [hash40("down_forward_d"), hash40("down_back_d")].contains(&motion_kind) {
        if random_roll != 0 {
            return Some(hash40("down_forward_d"));
        } else {
            return Some(hash40("down_back_d"));
        }
    }

    None
}
