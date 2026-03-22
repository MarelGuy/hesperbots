pub fn calculate_xp_for_level(level: u16) -> u16 {
    const BASE_XP: u16 = 10;
    const XP_MUL: u16 = 25;
    const LVL_INTERVAL: u16 = 10;

    let level_bonus = level / LVL_INTERVAL;
    let scaled_xp_mul = XP_MUL + level_bonus * 5;

    BASE_XP + (level - 1) * scaled_xp_mul
}
