pub use std::collections::HashMap;
use std::env;
use std::path::PathBuf;
pub use clap::Parser;
pub use indicium::simple::{SearchIndex, SearchIndexBuilder};
pub use std::process;
pub use std::{cmp, fs::File, io::{BufReader, Write, BufRead}};
pub use flutter_rust_bridge::frb;
pub use image::codecs::png::PngEncoder;
pub use image::DynamicImage;

static ASSET_PATH: &str = "data/flutter_assets/assets";

#[derive(Parser, Debug, Clone)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(short, long, default_value_t = 3)]
    pub resize_scale: u32,

    #[arg(short, long, default_value_t = 10)]
    pub spells_per_row: u32,

    #[arg(short, long, default_value = format!("{ASSET_PATH}/dictionary.txt"))]
    pub dictionary_path: String,

    #[arg(short, long, default_value_t = 5)]
    pub num_of_autocomplete_options: u32,


    #[arg(long, default_value = format!("false"))]
    pub drained: String,

    #[arg(long, default_value = format!("false"))]
    pub every_other: String,

    #[arg(long, default_value = format!("true"))]
    pub unlimited_spells: String,

    #[arg(long, default_value_t = 26)]
    pub spells_per_cast: u32,

    #[arg(long, default_value_t = 10000)]
    pub max_mana: u32,

    #[arg(long, default_value_t = 0)]
    pub mana_charge: u32,

    #[arg(long, default_value_t = 0)]
    pub reload_time: u32,

    #[arg(long, default_value_t = 0)]
    pub cast_delay: u32,

    #[arg(long, default_value_t = 1)]
    pub number_of_casts: u32,

    #[arg(long, default_value = format!(""))]
    pub mod_path: String,

    #[arg(long, default_value = format!(""))]
    pub data_path: String,
}

#[frb(opaque)]
pub struct RustApplication {
    spell_index: HashMap<String, String>,
    args: Args,
    expression: String,
    image: DynamicImage,
    image_result_path: PathBuf,
    autocompleter: SearchIndex<usize>,
}
fn get_absolute_path_from_relative(path: &str) -> PathBuf {
        let mut exe_path = PathBuf::from(env::current_exe().unwrap());
        exe_path.pop();
        exe_path.join(PathBuf::from(path))
    }
impl RustApplication {
    #[frb(sync)]
    pub fn new() -> Self {
        let args = Args::parse();
        let mut index = HashMap::from([
            ("BOMB".to_string(), "BOMB".to_string()),
            ("LIGHT_BULLET".to_string(), "LIGHT_BULLET".to_string()),
            ("LIGHT_BULLET_TRIGGER".to_string(), "LIGHT_BULLET_TRIGGER".to_string()),
            ("LIGHT_BULLET_TRIGGER_2".to_string(), "LIGHT_BULLET_TRIGGER_2".to_string()),
            ("LIGHT_BULLET_TIMER".to_string(), "LIGHT_BULLET_TIMER".to_string()),
            ("BULLET".to_string(), "BULLET".to_string()),
            ("BULLET_TRIGGER".to_string(), "BULLET_TRIGGER".to_string()),
            ("BULLET_TIMER".to_string(), "BULLET_TIMER".to_string()),
            ("HEAVY_BULLET".to_string(), "HEAVY_BULLET".to_string()),
            ("HEAVY_BULLET_TRIGGER".to_string(), "HEAVY_BULLET_TRIGGER".to_string()),
            ("HEAVY_BULLET_TIMER".to_string(), "HEAVY_BULLET_TIMER".to_string()),
            ("AIR_BULLET".to_string(), "AIR_BULLET".to_string()),
            ("SLOW_BULLET".to_string(), "SLOW_BULLET".to_string()),
            ("SLOW_BULLET_TRIGGER".to_string(), "SLOW_BULLET_TRIGGER".to_string()),
            ("SLOW_BULLET_TIMER".to_string(), "SLOW_BULLET_TIMER".to_string()),
            ("HOOK".to_string(), "HOOK".to_string()),
            ("BLACK_HOLE".to_string(), "BLACK_HOLE".to_string()),
            ("BLACK_HOLE_DEATH_TRIGGER".to_string(), "BLACK_HOLE_DEATH_TRIGGER".to_string()),
            ("WHITE_HOLE".to_string(), "WHITE_HOLE".to_string()),
            ("BLACK_HOLE_BIG".to_string(), "BLACK_HOLE_BIG".to_string()),
            ("WHITE_HOLE_BIG".to_string(), "WHITE_HOLE_BIG".to_string()),
            ("BLACK_HOLE_GIGA".to_string(), "BLACK_HOLE_GIGA".to_string()),
            ("WHITE_HOLE_GIGA".to_string(), "WHITE_HOLE_GIGA".to_string()),
            ("TENTACLE_PORTAL".to_string(), "TENTACLE_PORTAL".to_string()),
            ("SPITTER".to_string(), "SPITTER".to_string()),
            ("SPITTER_TIMER".to_string(), "SPITTER_TIMER".to_string()),
            ("SPITTER_TIER_2".to_string(), "SPITTER_TIER_2".to_string()),
            ("SPITTER_TIER_2_TIMER".to_string(), "SPITTER_TIER_2_TIMER".to_string()),
            ("SPITTER_TIER_3".to_string(), "SPITTER_TIER_3".to_string()),
            ("SPITTER_TIER_3_TIMER".to_string(), "SPITTER_TIER_3_TIMER".to_string()),
            ("BUBBLESHOT".to_string(), "BUBBLESHOT".to_string()),
            ("BUBBLESHOT_TRIGGER".to_string(), "BUBBLESHOT_TRIGGER".to_string()),
            ("DISC_BULLET".to_string(), "DISC_BULLET".to_string()),
            ("DISC_BULLET_BIG".to_string(), "DISC_BULLET_BIG".to_string()),
            ("DISC_BULLET_BIGGER".to_string(), "DISC_BULLET_BIGGER".to_string()),
            ("BOUNCY_ORB".to_string(), "BOUNCY_ORB".to_string()),
            ("BOUNCY_ORB_TIMER".to_string(), "BOUNCY_ORB_TIMER".to_string()),
            ("RUBBER_BALL".to_string(), "RUBBER_BALL".to_string()),
            ("ARROW".to_string(), "ARROW".to_string()),
            ("POLLEN".to_string(), "POLLEN".to_string()),
            ("LANCE".to_string(), "LANCE".to_string()),
            ("LANCE_HOLY".to_string(), "LANCE_HOLY".to_string()),
            ("ROCKET".to_string(), "ROCKET".to_string()),
            ("ROCKET_TIER_2".to_string(), "ROCKET_TIER_2".to_string()),
            ("ROCKET_TIER_3".to_string(), "ROCKET_TIER_3".to_string()),
            ("GRENADE".to_string(), "GRENADE".to_string()),
            ("GRENADE_TRIGGER".to_string(), "GRENADE_TRIGGER".to_string()),
            ("GRENADE_TIER_2".to_string(), "GRENADE_TIER_2".to_string()),
            ("GRENADE_TIER_3".to_string(), "GRENADE_TIER_3".to_string()),
            ("GRENADE_ANTI".to_string(), "GRENADE_ANTI".to_string()),
            ("GRENADE_LARGE".to_string(), "GRENADE_LARGE".to_string()),
            ("MINE".to_string(), "MINE".to_string()),
            ("MINE_DEATH_TRIGGER".to_string(), "MINE_DEATH_TRIGGER".to_string()),
            ("PIPE_BOMB".to_string(), "PIPE_BOMB".to_string()),
            ("PIPE_BOMB_DEATH_TRIGGER".to_string(), "PIPE_BOMB_DEATH_TRIGGER".to_string()),
            ("FISH".to_string(), "FISH".to_string()),
            ("EXPLODING_DEER".to_string(), "EXPLODING_DEER".to_string()),
            ("EXPLODING_DUCKS".to_string(), "EXPLODING_DUCKS".to_string()),
            ("WORM_SHOT".to_string(), "WORM_SHOT".to_string()),
            ("BOMB_DETONATOR".to_string(), "BOMB_DETONATOR".to_string()),
            ("LASER".to_string(), "LASER".to_string()),
            ("MEGALASER".to_string(), "MEGALASER".to_string()),
            ("LIGHTNING".to_string(), "LIGHTNING".to_string()),
            ("BALL_LIGHTNING".to_string(), "BALL_LIGHTNING".to_string()),
            ("LASER_EMITTER".to_string(), "LASER_EMITTER".to_string()),
            ("LASER_EMITTER_FOUR".to_string(), "LASER_EMITTER_FOUR".to_string()),
            ("LASER_EMITTER_CUTTER".to_string(), "LASER_EMITTER_CUTTER".to_string()),
            ("DIGGER".to_string(), "DIGGER".to_string()),
            ("POWERDIGGER".to_string(), "POWERDIGGER".to_string()),
            ("CHAINSAW".to_string(), "CHAINSAW".to_string()),
            ("LUMINOUS_DRILL".to_string(), "LUMINOUS_DRILL".to_string()),
            ("LASER_LUMINOUS_DRILL".to_string(), "LASER_LUMINOUS_DRILL".to_string()),
            ("TENTACLE".to_string(), "TENTACLE".to_string()),
            ("TENTACLE_TIMER".to_string(), "TENTACLE_TIMER".to_string()),
            ("HEAL_BULLET".to_string(), "HEAL_BULLET".to_string()),
            ("ANTIHEAL".to_string(), "ANTIHEAL".to_string()),
            ("SPIRAL_SHOT".to_string(), "SPIRAL_SHOT".to_string()),
            ("MAGIC_SHIELD".to_string(), "MAGIC_SHIELD".to_string()),
            ("BIG_MAGIC_SHIELD".to_string(), "BIG_MAGIC_SHIELD".to_string()),
            ("CHAIN_BOLT".to_string(), "CHAIN_BOLT".to_string()),
            ("FIREBALL".to_string(), "FIREBALL".to_string()),
            ("METEOR".to_string(), "METEOR".to_string()),
            ("FLAMETHROWER".to_string(), "FLAMETHROWER".to_string()),
            ("ICEBALL".to_string(), "ICEBALL".to_string()),
            ("SLIMEBALL".to_string(), "SLIMEBALL".to_string()),
            ("DARKFLAME".to_string(), "DARKFLAME".to_string()),
            ("MISSILE".to_string(), "MISSILE".to_string()),
            ("FUNKY_SPELL".to_string(), "FUNKY_SPELL".to_string()),
            ("PEBBLE".to_string(), "PEBBLE".to_string()),
            ("DYNAMITE".to_string(), "DYNAMITE".to_string()),
            ("GLITTER_BOMB".to_string(), "GLITTER_BOMB".to_string()),
            ("BUCKSHOT".to_string(), "BUCKSHOT".to_string()),
            ("FREEZING_GAZE".to_string(), "FREEZING_GAZE".to_string()),
            ("GLOWING_BOLT".to_string(), "GLOWING_BOLT".to_string()),
            ("SPORE_POD".to_string(), "SPORE_POD".to_string()),
            ("GLUE_SHOT".to_string(), "GLUE_SHOT".to_string()),
            ("BOMB_HOLY".to_string(), "BOMB_HOLY".to_string()),
            ("BOMB_HOLY_GIGA".to_string(), "BOMB_HOLY_GIGA".to_string()),
            ("PROPANE_TANK".to_string(), "PROPANE_TANK".to_string()),
            ("BOMB_CART".to_string(), "BOMB_CART".to_string()),
            ("CURSED_ORB".to_string(), "CURSED_ORB".to_string()),
            ("EXPANDING_ORB".to_string(), "EXPANDING_ORB".to_string()),
            ("CRUMBLING_EARTH".to_string(), "CRUMBLING_EARTH".to_string()),
            ("SUMMON_ROCK".to_string(), "SUMMON_ROCK".to_string()),
            ("SUMMON_EGG".to_string(), "SUMMON_EGG".to_string()),
            ("SUMMON_HOLLOW_EGG".to_string(), "SUMMON_HOLLOW_EGG".to_string()),
            ("TNTBOX".to_string(), "TNTBOX".to_string()),
            ("TNTBOX_BIG".to_string(), "TNTBOX_BIG".to_string()),
            ("SWARM_FLY".to_string(), "SWARM_FLY".to_string()),
            ("SWARM_FIREBUG".to_string(), "SWARM_FIREBUG".to_string()),
            ("SWARM_WASP".to_string(), "SWARM_WASP".to_string()),
            ("FRIEND_FLY".to_string(), "FRIEND_FLY".to_string()),
            ("ACIDSHOT".to_string(), "ACIDSHOT".to_string()),
            ("THUNDERBALL".to_string(), "THUNDERBALL".to_string()),
            ("FIREBOMB".to_string(), "FIREBOMB".to_string()),
            ("SOILBALL".to_string(), "SOILBALL".to_string()),
            ("DEATH_CROSS".to_string(), "DEATH_CROSS".to_string()),
            ("DEATH_CROSS_BIG".to_string(), "DEATH_CROSS_BIG".to_string()),
            ("INFESTATION".to_string(), "INFESTATION".to_string()),
            ("WALL_HORIZONTAL".to_string(), "WALL_HORIZONTAL".to_string()),
            ("WALL_VERTICAL".to_string(), "WALL_VERTICAL".to_string()),
            ("WALL_SQUARE".to_string(), "WALL_SQUARE".to_string()),
            ("TEMPORARY_WALL".to_string(), "TEMPORARY_WALL".to_string()),
            ("TEMPORARY_PLATFORM".to_string(), "TEMPORARY_PLATFORM".to_string()),
            ("PURPLE_EXPLOSION_FIELD".to_string(), "PURPLE_EXPLOSION_FIELD".to_string()),
            ("DELAYED_SPELL".to_string(), "DELAYED_SPELL".to_string()),
            ("LONG_DISTANCE_CAST".to_string(), "LONG_DISTANCE_CAST".to_string()),
            ("TELEPORT_CAST".to_string(), "TELEPORT_CAST".to_string()),
            ("SUPER_TELEPORT_CAST".to_string(), "SUPER_TELEPORT_CAST".to_string()),
            ("CASTER_CAST".to_string(), "CASTER_CAST".to_string()),
            ("MIST_RADIOACTIVE".to_string(), "MIST_RADIOACTIVE".to_string()),
            ("MIST_ALCOHOL".to_string(), "MIST_ALCOHOL".to_string()),
            ("MIST_SLIME".to_string(), "MIST_SLIME".to_string()),
            ("MIST_BLOOD".to_string(), "MIST_BLOOD".to_string()),
            ("CIRCLE_FIRE".to_string(), "CIRCLE_FIRE".to_string()),
            ("CIRCLE_ACID".to_string(), "CIRCLE_ACID".to_string()),
            ("CIRCLE_OIL".to_string(), "CIRCLE_OIL".to_string()),
            ("CIRCLE_WATER".to_string(), "CIRCLE_WATER".to_string()),
            ("MATERIAL_WATER".to_string(), "MATERIAL_WATER".to_string()),
            ("MATERIAL_OIL".to_string(), "MATERIAL_OIL".to_string()),
            ("MATERIAL_BLOOD".to_string(), "MATERIAL_BLOOD".to_string()),
            ("MATERIAL_ACID".to_string(), "MATERIAL_ACID".to_string()),
            ("MATERIAL_CEMENT".to_string(), "MATERIAL_CEMENT".to_string()),
            ("TELEPORT_PROJECTILE".to_string(), "TELEPORT_PROJECTILE".to_string()),
            ("TELEPORT_PROJECTILE_SHORT".to_string(), "TELEPORT_PROJECTILE_SHORT".to_string()),
            ("TELEPORT_PROJECTILE_STATIC".to_string(), "TELEPORT_PROJECTILE_STATIC".to_string()),
            ("SWAPPER_PROJECTILE".to_string(), "SWAPPER_PROJECTILE".to_string()),
            ("TELEPORT_PROJECTILE_CLOSER".to_string(), "TELEPORT_PROJECTILE_CLOSER".to_string()),
            ("NUKE".to_string(), "NUKE".to_string()),
            ("NUKE_GIGA".to_string(), "NUKE_GIGA".to_string()),
            ("FIREWORK".to_string(), "FIREWORK".to_string()),
            ("SUMMON_WANDGHOST".to_string(), "SUMMON_WANDGHOST".to_string()),
            ("TOUCH_GOLD".to_string(), "TOUCH_GOLD".to_string()),
            ("TOUCH_WATER".to_string(), "TOUCH_WATER".to_string()),
            ("TOUCH_OIL".to_string(), "TOUCH_OIL".to_string()),
            ("TOUCH_ALCOHOL".to_string(), "TOUCH_ALCOHOL".to_string()),
            ("TOUCH_PISS".to_string(), "TOUCH_PISS".to_string()),
            ("TOUCH_GRASS".to_string(), "TOUCH_GRASS".to_string()),
            ("TOUCH_BLOOD".to_string(), "TOUCH_BLOOD".to_string()),
            ("TOUCH_SMOKE".to_string(), "TOUCH_SMOKE".to_string()),
            ("DESTRUCTION".to_string(), "DESTRUCTION".to_string()),
            ("MASS_POLYMORPH".to_string(), "MASS_POLYMORPH".to_string()),
            ("BURST_2".to_string(), "BURST_2".to_string()),
            ("BURST_3".to_string(), "BURST_3".to_string()),
            ("BURST_4".to_string(), "BURST_4".to_string()),
            ("BURST_8".to_string(), "BURST_8".to_string()),
            ("BURST_X".to_string(), "BURST_X".to_string()),
            ("SCATTER_2".to_string(), "SCATTER_2".to_string()),
            ("SCATTER_3".to_string(), "SCATTER_3".to_string()),
            ("SCATTER_4".to_string(), "SCATTER_4".to_string()),
            ("I_SHAPE".to_string(), "I_SHAPE".to_string()),
            ("Y_SHAPE".to_string(), "Y_SHAPE".to_string()),
            ("T_SHAPE".to_string(), "T_SHAPE".to_string()),
            ("W_SHAPE".to_string(), "W_SHAPE".to_string()),
            ("CIRCLE_SHAPE".to_string(), "CIRCLE_SHAPE".to_string()),
            ("PENTAGRAM_SHAPE".to_string(), "PENTAGRAM_SHAPE".to_string()),
            ("I_SHOT".to_string(), "I_SHOT".to_string()),
            ("Y_SHOT".to_string(), "Y_SHOT".to_string()),
            ("T_SHOT".to_string(), "T_SHOT".to_string()),
            ("W_SHOT".to_string(), "W_SHOT".to_string()),
            ("QUAD_SHOT".to_string(), "QUAD_SHOT".to_string()),
            ("PENTA_SHOT".to_string(), "PENTA_SHOT".to_string()),
            ("HEXA_SHOT".to_string(), "HEXA_SHOT".to_string()),
            ("SPREAD_REDUCE".to_string(), "SPREAD_REDUCE".to_string()),
            ("HEAVY_SPREAD".to_string(), "HEAVY_SPREAD".to_string()),
            ("RECHARGE".to_string(), "RECHARGE".to_string()),
            ("LIFETIME".to_string(), "LIFETIME".to_string()),
            ("LIFETIME_DOWN".to_string(), "LIFETIME_DOWN".to_string()),
            ("NOLLA".to_string(), "NOLLA".to_string()),
            ("SLOW_BUT_STEADY".to_string(), "SLOW_BUT_STEADY".to_string()),
            ("EXPLOSION_REMOVE".to_string(), "EXPLOSION_REMOVE".to_string()),
            ("EXPLOSION_TINY".to_string(), "EXPLOSION_TINY".to_string()),
            ("LASER_EMITTER_WIDER".to_string(), "LASER_EMITTER_WIDER".to_string()),
            ("MANA_REDUCE".to_string(), "MANA_REDUCE".to_string()),
            ("BLOOD_MAGIC".to_string(), "BLOOD_MAGIC".to_string()),
            ("MONEY_MAGIC".to_string(), "MONEY_MAGIC".to_string()),
            ("BLOOD_TO_POWER".to_string(), "BLOOD_TO_POWER".to_string()),
            ("DUPLICATE".to_string(), "DUPLICATE".to_string()),
            ("QUANTUM_SPLIT".to_string(), "QUANTUM_SPLIT".to_string()),
            ("GRAVITY".to_string(), "GRAVITY".to_string()),
            ("GRAVITY_ANTI".to_string(), "GRAVITY_ANTI".to_string()),
            ("SINEWAVE".to_string(), "SINEWAVE".to_string()),
            ("CHAOTIC_ARC".to_string(), "CHAOTIC_ARC".to_string()),
            ("PINGPONG_PATH".to_string(), "PINGPONG_PATH".to_string()),
            ("AVOIDING_ARC".to_string(), "AVOIDING_ARC".to_string()),
            ("FLOATING_ARC".to_string(), "FLOATING_ARC".to_string()),
            ("FLY_DOWNWARDS".to_string(), "FLY_DOWNWARDS".to_string()),
            ("FLY_UPWARDS".to_string(), "FLY_UPWARDS".to_string()),
            ("HORIZONTAL_ARC".to_string(), "HORIZONTAL_ARC".to_string()),
            ("LINE_ARC".to_string(), "LINE_ARC".to_string()),
            ("ORBIT_SHOT".to_string(), "ORBIT_SHOT".to_string()),
            ("SPIRALING_SHOT".to_string(), "SPIRALING_SHOT".to_string()),
            ("PHASING_ARC".to_string(), "PHASING_ARC".to_string()),
            ("TRUE_ORBIT".to_string(), "TRUE_ORBIT".to_string()),
            ("BOUNCE".to_string(), "BOUNCE".to_string()),
            ("REMOVE_BOUNCE".to_string(), "REMOVE_BOUNCE".to_string()),
            ("HOMING".to_string(), "HOMING".to_string()),
            ("ANTI_HOMING".to_string(), "ANTI_HOMING".to_string()),
            ("HOMING_WAND".to_string(), "HOMING_WAND".to_string()),
            ("HOMING_SHORT".to_string(), "HOMING_SHORT".to_string()),
            ("HOMING_ROTATE".to_string(), "HOMING_ROTATE".to_string()),
            ("HOMING_SHOOTER".to_string(), "HOMING_SHOOTER".to_string()),
            ("AUTOAIM".to_string(), "AUTOAIM".to_string()),
            ("HOMING_ACCELERATING".to_string(), "HOMING_ACCELERATING".to_string()),
            ("HOMING_CURSOR".to_string(), "HOMING_CURSOR".to_string()),
            ("HOMING_AREA".to_string(), "HOMING_AREA".to_string()),
            ("PIERCING_SHOT".to_string(), "PIERCING_SHOT".to_string()),
            ("CLIPPING_SHOT".to_string(), "CLIPPING_SHOT".to_string()),
            ("DAMAGE".to_string(), "DAMAGE".to_string()),
            ("DAMAGE_RANDOM".to_string(), "DAMAGE_RANDOM".to_string()),
            ("BLOODLUST".to_string(), "BLOODLUST".to_string()),
            ("DAMAGE_FOREVER".to_string(), "DAMAGE_FOREVER".to_string()),
            ("CRITICAL_HIT".to_string(), "CRITICAL_HIT".to_string()),
            ("AREA_DAMAGE".to_string(), "AREA_DAMAGE".to_string()),
            ("SPELLS_TO_POWER".to_string(), "SPELLS_TO_POWER".to_string()),
            ("ESSENCE_TO_POWER".to_string(), "ESSENCE_TO_POWER".to_string()),
            ("ZERO_DAMAGE".to_string(), "ZERO_DAMAGE".to_string()),
            ("HEAVY_SHOT".to_string(), "HEAVY_SHOT".to_string()),
            ("LIGHT_SHOT".to_string(), "LIGHT_SHOT".to_string()),
            ("KNOCKBACK".to_string(), "KNOCKBACK".to_string()),
            ("RECOIL".to_string(), "RECOIL".to_string()),
            ("RECOIL_DAMPER".to_string(), "RECOIL_DAMPER".to_string()),
            ("SPEED".to_string(), "SPEED".to_string()),
            ("ACCELERATING_SHOT".to_string(), "ACCELERATING_SHOT".to_string()),
            ("DECELERATING_SHOT".to_string(), "DECELERATING_SHOT".to_string()),
            ("EXPLOSIVE_PROJECTILE".to_string(), "EXPLOSIVE_PROJECTILE".to_string()),
            ("CLUSTERMOD".to_string(), "CLUSTERMOD".to_string()),
            ("WATER_TO_POISON".to_string(), "WATER_TO_POISON".to_string()),
            ("BLOOD_TO_ACID".to_string(), "BLOOD_TO_ACID".to_string()),
            ("LAVA_TO_BLOOD".to_string(), "LAVA_TO_BLOOD".to_string()),
            ("LIQUID_TO_EXPLOSION".to_string(), "LIQUID_TO_EXPLOSION".to_string()),
            ("TOXIC_TO_ACID".to_string(), "TOXIC_TO_ACID".to_string()),
            ("STATIC_TO_SAND".to_string(), "STATIC_TO_SAND".to_string()),
            ("TRANSMUTATION".to_string(), "TRANSMUTATION".to_string()),
            ("RANDOM_EXPLOSION".to_string(), "RANDOM_EXPLOSION".to_string()),
            ("NECROMANCY".to_string(), "NECROMANCY".to_string()),
            ("LIGHT".to_string(), "LIGHT".to_string()),
            ("EXPLOSION".to_string(), "EXPLOSION".to_string()),
            ("EXPLOSION_LIGHT".to_string(), "EXPLOSION_LIGHT".to_string()),
            ("FIRE_BLAST".to_string(), "FIRE_BLAST".to_string()),
            ("POISON_BLAST".to_string(), "POISON_BLAST".to_string()),
            ("ALCOHOL_BLAST".to_string(), "ALCOHOL_BLAST".to_string()),
            ("THUNDER_BLAST".to_string(), "THUNDER_BLAST".to_string()),
            ("BERSERK_FIELD".to_string(), "BERSERK_FIELD".to_string()),
            ("POLYMORPH_FIELD".to_string(), "POLYMORPH_FIELD".to_string()),
            ("CHAOS_POLYMORPH_FIELD".to_string(), "CHAOS_POLYMORPH_FIELD".to_string()),
            ("ELECTROCUTION_FIELD".to_string(), "ELECTROCUTION_FIELD".to_string()),
            ("FREEZE_FIELD".to_string(), "FREEZE_FIELD".to_string()),
            ("REGENERATION_FIELD".to_string(), "REGENERATION_FIELD".to_string()),
            ("TELEPORTATION_FIELD".to_string(), "TELEPORTATION_FIELD".to_string()),
            ("LEVITATION_FIELD".to_string(), "LEVITATION_FIELD".to_string()),
            ("SHIELD_FIELD".to_string(), "SHIELD_FIELD".to_string()),
            ("PROJECTILE_TRANSMUTATION_FIELD".to_string(), "PROJECTILE_TRANSMUTATION_FIELD".to_string()),
            ("PROJECTILE_THUNDER_FIELD".to_string(), "PROJECTILE_THUNDER_FIELD".to_string()),
            ("PROJECTILE_GRAVITY_FIELD".to_string(), "PROJECTILE_GRAVITY_FIELD".to_string()),
            ("VACUUM_POWDER".to_string(), "VACUUM_POWDER".to_string()),
            ("VACUUM_LIQUID".to_string(), "VACUUM_LIQUID".to_string()),
            ("VACUUM_ENTITIES".to_string(), "VACUUM_ENTITIES".to_string()),
            ("SEA_LAVA".to_string(), "SEA_LAVA".to_string()),
            ("SEA_ALCOHOL".to_string(), "SEA_ALCOHOL".to_string()),
            ("SEA_OIL".to_string(), "SEA_OIL".to_string()),
            ("SEA_WATER".to_string(), "SEA_WATER".to_string()),
            ("SEA_SWAMP".to_string(), "SEA_SWAMP".to_string()),
            ("SEA_ACID".to_string(), "SEA_ACID".to_string()),
            ("SEA_ACID_GAS".to_string(), "SEA_ACID_GAS".to_string()),
            ("SEA_MIMIC".to_string(), "SEA_MIMIC".to_string()),
            ("CLOUD_WATER".to_string(), "CLOUD_WATER".to_string()),
            ("CLOUD_OIL".to_string(), "CLOUD_OIL".to_string()),
            ("CLOUD_BLOOD".to_string(), "CLOUD_BLOOD".to_string()),
            ("CLOUD_ACID".to_string(), "CLOUD_ACID".to_string()),
            ("CLOUD_THUNDER".to_string(), "CLOUD_THUNDER".to_string()),
            ("ELECTRIC_CHARGE".to_string(), "ELECTRIC_CHARGE".to_string()),
            ("MATTER_EATER".to_string(), "MATTER_EATER".to_string()),
            ("FREEZE".to_string(), "FREEZE".to_string()),
            ("HITFX_BURNING_CRITICAL_HIT".to_string(), "HITFX_BURNING_CRITICAL_HIT".to_string()),
            ("HITFX_CRITICAL_WATER".to_string(), "HITFX_CRITICAL_WATER".to_string()),
            ("HITFX_CRITICAL_OIL".to_string(), "HITFX_CRITICAL_OIL".to_string()),
            ("HITFX_CRITICAL_BLOOD".to_string(), "HITFX_CRITICAL_BLOOD".to_string()),
            ("HITFX_TOXIC_CHARM".to_string(), "HITFX_TOXIC_CHARM".to_string()),
            ("HITFX_EXPLOSION_SLIME".to_string(), "HITFX_EXPLOSION_SLIME".to_string()),
            ("HITFX_EXPLOSION_SLIME_GIGA".to_string(), "HITFX_EXPLOSION_SLIME_GIGA".to_string()),
            ("HITFX_EXPLOSION_ALCOHOL".to_string(), "HITFX_EXPLOSION_ALCOHOL".to_string()),
            ("HITFX_EXPLOSION_ALCOHOL_GIGA".to_string(), "HITFX_EXPLOSION_ALCOHOL_GIGA".to_string()),
            ("HITFX_PETRIFY".to_string(), "HITFX_PETRIFY".to_string()),
            ("ROCKET_DOWNWARDS".to_string(), "ROCKET_DOWNWARDS".to_string()),
            ("ROCKET_OCTAGON".to_string(), "ROCKET_OCTAGON".to_string()),
            ("FIZZLE".to_string(), "FIZZLE".to_string()),
            ("BOUNCE_EXPLOSION".to_string(), "BOUNCE_EXPLOSION".to_string()),
            ("BOUNCE_SPARK".to_string(), "BOUNCE_SPARK".to_string()),
            ("BOUNCE_LASER".to_string(), "BOUNCE_LASER".to_string()),
            ("BOUNCE_LASER_EMITTER".to_string(), "BOUNCE_LASER_EMITTER".to_string()),
            ("BOUNCE_LARPA".to_string(), "BOUNCE_LARPA".to_string()),
            ("BOUNCE_SMALL_EXPLOSION".to_string(), "BOUNCE_SMALL_EXPLOSION".to_string()),
            ("BOUNCE_LIGHTNING".to_string(), "BOUNCE_LIGHTNING".to_string()),
            ("BOUNCE_HOLE".to_string(), "BOUNCE_HOLE".to_string()),
            ("FIREBALL_RAY".to_string(), "FIREBALL_RAY".to_string()),
            ("LIGHTNING_RAY".to_string(), "LIGHTNING_RAY".to_string()),
            ("TENTACLE_RAY".to_string(), "TENTACLE_RAY".to_string()),
            ("LASER_EMITTER_RAY".to_string(), "LASER_EMITTER_RAY".to_string()),
            ("FIREBALL_RAY_LINE".to_string(), "FIREBALL_RAY_LINE".to_string()),
            ("FIREBALL_RAY_ENEMY".to_string(), "FIREBALL_RAY_ENEMY".to_string()),
            ("LIGHTNING_RAY_ENEMY".to_string(), "LIGHTNING_RAY_ENEMY".to_string()),
            ("TENTACLE_RAY_ENEMY".to_string(), "TENTACLE_RAY_ENEMY".to_string()),
            ("GRAVITY_FIELD_ENEMY".to_string(), "GRAVITY_FIELD_ENEMY".to_string()),
            ("CURSE".to_string(), "CURSE".to_string()),
            ("CURSE_WITHER_PROJECTILE".to_string(), "CURSE_WITHER_PROJECTILE".to_string()),
            ("CURSE_WITHER_EXPLOSION".to_string(), "CURSE_WITHER_EXPLOSION".to_string()),
            ("CURSE_WITHER_MELEE".to_string(), "CURSE_WITHER_MELEE".to_string()),
            ("CURSE_WITHER_ELECTRICITY".to_string(), "CURSE_WITHER_ELECTRICITY".to_string()),
            ("ORBIT_DISCS".to_string(), "ORBIT_DISCS".to_string()),
            ("ORBIT_FIREBALLS".to_string(), "ORBIT_FIREBALLS".to_string()),
            ("ORBIT_NUKES".to_string(), "ORBIT_NUKES".to_string()),
            ("ORBIT_LASERS".to_string(), "ORBIT_LASERS".to_string()),
            ("ORBIT_LARPA".to_string(), "ORBIT_LARPA".to_string()),
            ("CHAIN_SHOT".to_string(), "CHAIN_SHOT".to_string()),
            ("ARC_ELECTRIC".to_string(), "ARC_ELECTRIC".to_string()),
            ("ARC_FIRE".to_string(), "ARC_FIRE".to_string()),
            ("ARC_GUNPOWDER".to_string(), "ARC_GUNPOWDER".to_string()),
            ("ARC_POISON".to_string(), "ARC_POISON".to_string()),
            ("CRUMBLING_EARTH_PROJECTILE".to_string(), "CRUMBLING_EARTH_PROJECTILE".to_string()),
            ("X_RAY".to_string(), "X_RAY".to_string()),
            ("UNSTABLE_GUNPOWDER".to_string(), "UNSTABLE_GUNPOWDER".to_string()),
            ("ACID_TRAIL".to_string(), "ACID_TRAIL".to_string()),
            ("POISON_TRAIL".to_string(), "POISON_TRAIL".to_string()),
            ("OIL_TRAIL".to_string(), "OIL_TRAIL".to_string()),
            ("WATER_TRAIL".to_string(), "WATER_TRAIL".to_string()),
            ("GUNPOWDER_TRAIL".to_string(), "GUNPOWDER_TRAIL".to_string()),
            ("FIRE_TRAIL".to_string(), "FIRE_TRAIL".to_string()),
            ("BURN_TRAIL".to_string(), "BURN_TRAIL".to_string()),
            ("TORCH".to_string(), "TORCH".to_string()),
            ("TORCH_ELECTRIC".to_string(), "TORCH_ELECTRIC".to_string()),
            ("ENERGY_SHIELD".to_string(), "ENERGY_SHIELD".to_string()),
            ("ENERGY_SHIELD_SECTOR".to_string(), "ENERGY_SHIELD_SECTOR".to_string()),
            ("ENERGY_SHIELD_SHOT".to_string(), "ENERGY_SHIELD_SHOT".to_string()),
            ("TINY_GHOST".to_string(), "TINY_GHOST".to_string()),
            ("OCARINA_A".to_string(), "OCARINA_A".to_string()),
            ("OCARINA_B".to_string(), "OCARINA_B".to_string()),
            ("OCARINA_C".to_string(), "OCARINA_C".to_string()),
            ("OCARINA_D".to_string(), "OCARINA_D".to_string()),
            ("OCARINA_E".to_string(), "OCARINA_E".to_string()),
            ("OCARINA_F".to_string(), "OCARINA_F".to_string()),
            ("OCARINA_GSHARP".to_string(), "OCARINA_GSHARP".to_string()),
            ("OCARINA_A2".to_string(), "OCARINA_A2".to_string()),
            ("KANTELE_A".to_string(), "KANTELE_A".to_string()),
            ("KANTELE_D".to_string(), "KANTELE_D".to_string()),
            ("KANTELE_DIS".to_string(), "KANTELE_DIS".to_string()),
            ("KANTELE_E".to_string(), "KANTELE_E".to_string()),
            ("KANTELE_G".to_string(), "KANTELE_G".to_string()),
            ("RANDOM_SPELL".to_string(), "RANDOM_SPELL".to_string()),
            ("RANDOM_PROJECTILE".to_string(), "RANDOM_PROJECTILE".to_string()),
            ("RANDOM_MODIFIER".to_string(), "RANDOM_MODIFIER".to_string()),
            ("RANDOM_STATIC_PROJECTILE".to_string(), "RANDOM_STATIC_PROJECTILE".to_string()),
            ("DRAW_RANDOM".to_string(), "DRAW_RANDOM".to_string()),
            ("DRAW_RANDOM_X3".to_string(), "DRAW_RANDOM_X3".to_string()),
            ("DRAW_3_RANDOM".to_string(), "DRAW_3_RANDOM".to_string()),
            ("ALL_NUKES".to_string(), "ALL_NUKES".to_string()),
            ("ALL_DISCS".to_string(), "ALL_DISCS".to_string()),
            ("ALL_ROCKETS".to_string(), "ALL_ROCKETS".to_string()),
            ("ALL_DEATHCROSSES".to_string(), "ALL_DEATHCROSSES".to_string()),
            ("ALL_BLACKHOLES".to_string(), "ALL_BLACKHOLES".to_string()),
            ("ALL_ACID".to_string(), "ALL_ACID".to_string()),
            ("ALL_SPELLS".to_string(), "ALL_SPELLS".to_string()),
            ("SUMMON_PORTAL".to_string(), "SUMMON_PORTAL".to_string()),
            ("ADD_TRIGGER".to_string(), "ADD_TRIGGER".to_string()),
            ("ADD_TIMER".to_string(), "ADD_TIMER".to_string()),
            ("ADD_DEATH_TRIGGER".to_string(), "ADD_DEATH_TRIGGER".to_string()),
            ("LARPA_CHAOS".to_string(), "LARPA_CHAOS".to_string()),
            ("LARPA_DOWNWARDS".to_string(), "LARPA_DOWNWARDS".to_string()),
            ("LARPA_UPWARDS".to_string(), "LARPA_UPWARDS".to_string()),
            ("LARPA_CHAOS_2".to_string(), "LARPA_CHAOS_2".to_string()),
            ("LARPA_DEATH".to_string(), "LARPA_DEATH".to_string()),
            ("ALPHA".to_string(), "ALPHA".to_string()),
            ("GAMMA".to_string(), "GAMMA".to_string()),
            ("TAU".to_string(), "TAU".to_string()),
            ("OMEGA".to_string(), "OMEGA".to_string()),
            ("MU".to_string(), "MU".to_string()),
            ("PHI".to_string(), "PHI".to_string()),
            ("SIGMA".to_string(), "SIGMA".to_string()),
            ("ZETA".to_string(), "ZETA".to_string()),
            ("DIVIDE_2".to_string(), "DIVIDE_2".to_string()),
            ("DIVIDE_3".to_string(), "DIVIDE_3".to_string()),
            ("DIVIDE_4".to_string(), "DIVIDE_4".to_string()),
            ("DIVIDE_10".to_string(), "DIVIDE_10".to_string()),
            ("METEOR_RAIN".to_string(), "METEOR_RAIN".to_string()),
            ("WORM_RAIN".to_string(), "WORM_RAIN".to_string()),
            ("RESET".to_string(), "RESET".to_string()),
            ("IF_ENEMY".to_string(), "IF_ENEMY".to_string()),
            ("IF_PROJECTILE".to_string(), "IF_PROJECTILE".to_string()),
            ("IF_HP".to_string(), "IF_HP".to_string()),
            ("IF_HALF".to_string(), "IF_HALF".to_string()),
            ("IF_END".to_string(), "IF_END".to_string()),
            ("IF_ELSE".to_string(), "IF_ELSE".to_string()),
            ("COLOUR_RED".to_string(), "COLOUR_RED".to_string()),
            ("COLOUR_ORANGE".to_string(), "COLOUR_ORANGE".to_string()),
            ("COLOUR_GREEN".to_string(), "COLOUR_GREEN".to_string()),
            ("COLOUR_YELLOW".to_string(), "COLOUR_YELLOW".to_string()),
            ("COLOUR_PURPLE".to_string(), "COLOUR_PURPLE".to_string()),
            ("COLOUR_BLUE".to_string(), "COLOUR_BLUE".to_string()),
            ("COLOUR_RAINBOW".to_string(), "COLOUR_RAINBOW".to_string()),
            ("COLOUR_INVIS".to_string(), "COLOUR_INVIS".to_string()),
            ("RAINBOW_TRAIL".to_string(), "RAINBOW_TRAIL".to_string()),
            ("CESSATION".to_string(), "CESSATION".to_string()),
        ]);
        
        let path = get_absolute_path_from_relative(&args.dictionary_path);
        let data = std::fs::read_to_string(path).unwrap();

        for lines in data.split("\n") {
            let mut pair = lines.split(",");
            let key = pair.next().unwrap().to_owned();
            let value = pair.next().unwrap().to_owned();
            index.insert(key, value);
        }
        let mut autocomplete = SearchIndexBuilder::default()
            .fuzzy_length(1)
            .fuzzy_minimum_score(0.4)
            .dump_keyword(None)
            .build();
        let index_keys = index.keys().map(|x| x.clone()).collect::<Vec<String>>();
        index_keys.iter().enumerate().for_each(|(num, element)| autocomplete.insert(&num, element));

        Self { 
            spell_index: index, 
            args, 
            image: DynamicImage::new_rgba8(1, 1), 
            expression: String::new(), 
            image_result_path: get_absolute_path_from_relative(&format!("{ASSET_PATH}/output.png")),
            autocompleter: autocomplete,
        }
    }
    fn get_default() -> DynamicImage {
        match image::ImageReader::open(get_absolute_path_from_relative(&format!("{ASSET_PATH}/sprites/EMPTY.png"))) {
        Ok(reader) => match reader.decode() {
            Ok(image) => image,
            Err(e) => {
                println!("Failed to decode inventory background: {:?}", e);
                return DynamicImage::new_rgba8(1, 1);
            },
        },
        Err(e) => {
                println!("Failed to read inventory background: {:?}", e);
                return DynamicImage::new_rgba8(1, 1);
            },
    }
    }
    fn encode_png(image: &DynamicImage) -> Vec<u8> {
        let mut output = Vec::new();
        let encoder = image::codecs::png::PngEncoder::new(&mut output);
        match image.write_with_encoder(encoder) {
            Ok(_) => (),
            Err(e) => {
                println!("Failed to encode image: {:?}", e);
            },
        };
        output
    }
    pub fn copy_image(&self) {

        self.image.save(&self.image_result_path).unwrap();

        let path = get_absolute_path_from_relative(&format!("{ASSET_PATH}"));

        match process::Command::new("./clip")
                .current_dir(path)
                .arg("-i").arg("output.png")
                .stdin(process::Stdio::null())
                .spawn() {
            Ok(_) => (),
            Err(e) => println!("Failed to daemonize: {:?}", e),
        }
    }
    pub fn evaluate_expression(&mut self, expression: &str) -> (Vec<u8>, u32) {
        self.expression = expression.to_string();

    let tokens = expression.split(' ').into_iter().collect::<Vec<&str>>();

    let resize_scale = self.args.resize_scale;


    let scale: u32 = 20;

    if tokens.len() == 0 {
        let mut image = RustApplication::get_default();
        if scale != 1 {
            image = image::DynamicImage::resize(&image, image.width()*resize_scale, image.height()*resize_scale, image::imageops::FilterType::Nearest);
        }
        return (RustApplication::encode_png(&image), image.height());
    }

    let spells_per_row: u32 = self.args.spells_per_row;

    let width = cmp::min(tokens.len() as u32, spells_per_row) * scale;
    let height = ((tokens.len() as u32 - 1)/spells_per_row +1) * scale;

    let mut x: u32 = 0;
    let mut y: u32 = 0;

    let mut image = image::DynamicImage::new_rgba8(width, height);
    for token in tokens {
        let word = token.to_uppercase();
        let token = &self.spell_index.get(&word).unwrap_or(&word);
        let spell_sprite = match image::ImageReader::open(get_absolute_path_from_relative(&format!("{ASSET_PATH}/sprites/{token}.png"))) {
            Ok(reader) => match reader.decode() {
                Ok(image) => image,
                Err(e) => {
                    println!("Failed to decode spell image: {:?}", e);
                    RustApplication::get_default()
                },
            },
            Err(_) => {
                RustApplication::get_default()
            },
        };
        image::imageops::overlay(&mut image, &spell_sprite, x as i64, y as i64);
        x += scale;
        if x >= image.width() {
            x = 0;
            y += scale;
        }
    }
    if scale != 1 {
        image = image::DynamicImage::resize(&image, image.width()*resize_scale, image.height()*resize_scale, image::imageops::FilterType::Nearest);
    }
    self.image = image;

    (RustApplication::encode_png(&self.image), self.image.height())
    
    }
    pub fn autocomplete_expression(&self) -> (String, Vec<String>) {
        
        let mut tokens = self.expression.split(' ').collect::<Vec<&str>>();

        let query = tokens.pop().unwrap_or("");

        let result: Vec<String> = self.autocompleter.autocomplete_with(
            &indicium::simple::AutocompleteType::Keyword, 
            &(self.args.num_of_autocomplete_options as usize), 
            &query
        );
        (tokens.join(" "), result.iter().map(|x| x.to_uppercase()).collect())
    }
    pub fn get_number_of_suggestions(&self) -> u32 {
        self.args.num_of_autocomplete_options
    }
    pub fn fetch_eval_tree(&self, ansi: &str) -> String {
        if self.args.mod_path.is_empty() || self.args.data_path.is_empty() {
            return String::from("Error: Mod path or data path not set");
        }

        let empty_string = String::new();
        let spell_list = self.expression.split(' ')
            .map(|x| self.spell_index.get(&x.to_uppercase())
            .unwrap_or(&empty_string).as_str())
            .filter(|x| !x.is_empty())
            .collect::<Vec<&str>>();

        if spell_list.len() == 0 {
            return String::from("No spells set.");
        }

        let output = process::Command::new("luajit")
            .current_dir(get_absolute_path_from_relative(&format!("{ASSET_PATH}/wand_eval_tree")))
            .arg("main.lua")
            .arg("-a").arg(ansi)
            .arg("-d").arg(&self.args.drained)
            .arg("-e").arg(&self.args.every_other)
            .arg("-u").arg(&self.args.unlimited_spells)
            .arg("-sc").arg(&format!("{}",self.args.spells_per_cast))
            .arg("-ma").arg(&format!("{}",self.args.max_mana))
            .arg("-mc").arg(&format!("{}",self.args.mana_charge))
            .arg("-rt").arg(&format!("{}",self.args.reload_time))
            .arg("-cd").arg(&format!("{}",self.args.cast_delay))
            .arg("-nc").arg(&format!("{}",self.args.number_of_casts))
            .arg("-mp").arg(&self.args.mod_path)
            .arg("-dp").arg(&self.args.data_path)
            .arg("-sp").args(spell_list.as_slice())
            .output().unwrap();
        let string = String::from_utf8(output.stdout).unwrap_or("Something went wrong, probably mod/data filepaths".to_string());
        string
    }
    pub fn copy_eval_tree(&self) {
        let tree = self.fetch_eval_tree("true");

        let path = get_absolute_path_from_relative(&format!("{ASSET_PATH}/output.txt"));

        match std::fs::write(path, tree) {
            Ok(_) => (),
            Err(e) => println!("Failed to write to file: {:?}", e),
        };

        match process::Command::new("./clip")
                .current_dir(get_absolute_path_from_relative(&format!("{ASSET_PATH}")))
                .arg("-t").arg("output.txt")
                .stdin(process::Stdio::null())
                .spawn() {
            Ok(_) => (),
            Err(e) => println!("Failed to daemonize: {:?}", e),
        }
    }
}
