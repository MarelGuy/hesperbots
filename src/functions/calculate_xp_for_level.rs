pub const fn calculate_xp_for_level(level: i32) -> i32 {
    const BASE_XP: i32 = 10;
    const XP_MUL: i32 = 25;
    const LVL_INTERVAL: i32 = 10;

    let level_bonus = level / LVL_INTERVAL;
    let scaled_xp_mul = XP_MUL + level_bonus * 5;

    BASE_XP + (level - 1) * scaled_xp_mul
}
