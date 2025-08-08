use spacetimedb::Table;

use super::*;

#[spacetimedb::table(public, name = global_settings)]
pub struct GlobalSettings {
    #[unique]
    always_zero: u32,
    pub team_slots: u8,
    pub match_g: MatchG,
}

impl Default for GlobalSettings {
    fn default() -> Self {
        Self {
            always_zero: 0,
            team_slots: 5,
            match_g: MatchG {
                unit_buy: 3,
                unit_sell: 1,
                house_buy: 5,
                fusion_slot_mul: 1,
                reroll: 1,
                initial: 10,
            },
        }
    }
}

#[derive(SpacetimeType)]
pub struct MatchG {
    pub unit_buy: i32,
    pub unit_sell: i32,
    pub house_buy: i32,
    pub fusion_slot_mul: i32,
    pub reroll: i32,
    pub initial: i32,
}

impl GlobalSettings {
    pub fn get(ctx: &ReducerContext) -> Self {
        ctx.db.global_settings().always_zero().find(0).unwrap()
    }
    pub fn replace(self, ctx: &ReducerContext) {
        ctx.db.global_settings().always_zero().delete(0);
        ctx.db.global_settings().insert(self);
    }
}
