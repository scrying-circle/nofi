pub use clap::Parser;
pub use flutter_rust_bridge::frb;
pub use image::codecs::png::PngEncoder;
pub use image::DynamicImage;
pub use indicium::simple::{SearchIndex, SearchIndexBuilder};
pub use std::collections::HashMap;
use std::collections::HashSet;
use std::env;
use std::path::PathBuf;
pub use std::process;
pub use std::{
    cmp,
    fs::File,
    io::{BufRead, BufReader, Write},
};

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
pub enum SpellType {
    Projectile,
    Modifier,
    StaticProjectile,
    Material,
    Utility,
    Other,
    Passive,
    Multicast,
}
impl SpellType {
    pub fn from_int(i: usize) -> Self {
        match i {
            1 => SpellType::Projectile,
            0 => SpellType::Modifier,
            2 => SpellType::StaticProjectile,
            3 => SpellType::Material,
            4 => SpellType::Utility,
            7 => SpellType::Other,
            5 => SpellType::Passive,
            6 => SpellType::Multicast,
            _ => SpellType::Other,
        }
    }
    pub fn bg_path(&self) -> &str {
        match self {
            SpellType::Projectile => "data/ui_gfx/inventory/item_bg_projectile.png",
            SpellType::Modifier => "data/ui_gfx/inventory/item_bg_modifier.png",
            SpellType::StaticProjectile => "data/ui_gfx/inventory/item_bg_static_projectile.png",
            SpellType::Material => "data/ui_gfx/inventory/item_bg_material.png",
            SpellType::Utility => "data/ui_gfx/inventory/item_bg_utility.png",
            SpellType::Other => "data/ui_gfx/inventory/item_bg_other.png",
            SpellType::Passive => "data/ui_gfx/inventory/item_bg_passive.png",
            SpellType::Multicast => "data/ui_gfx/inventory/item_bg_draw_many.png",
        }
    }
}
#[frb(opaque)]
pub struct RustApplication {
    graphics_index: HashMap<String, (SpellType, String)>,
    spell_index: HashSet<String>,
    spell_dictionary: HashMap<String, String>,
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

        let mut script_path = env::current_exe().unwrap();
        script_path.pop();
        script_path.push(format!("{ASSET_PATH}/get_spells.lua"));
        let gun_actions_output = process::Command::new("luajit")
            .current_dir(&args.data_path)
            .arg(script_path)
            .output()
            .expect("Failed to execute process");
        let gun_actions = String::from_utf8(gun_actions_output.stdout)
            .expect("Failed to convert output to string");
        let mut graphics_index = HashMap::new();
        for line in gun_actions.split("\n") {
            if line.is_empty() {
                continue;
            }
            let mut parts = line.split(",");
            let spell_id = parts.next().expect("Did not find first element");
            let spell_type = parts.next().expect("Did not find second element");
            let spell_path = parts.next().expect("Did not find third element");
            graphics_index.insert(
                spell_id.to_string(),
                (
                    SpellType::from_int(spell_type.parse().unwrap()),
                    spell_path.to_string(),
                ),
            );
        }

        let mut index = HashSet::new();
        for spell in graphics_index.keys() {
            index.insert(spell.clone());
        }

        let mut spell_dictionary = HashMap::new();

        let path = get_absolute_path_from_relative(&args.dictionary_path);
        let data = std::fs::read_to_string(path).unwrap();

        for lines in data.split("\n") {
            let mut pair = lines.split(",");
            let key = pair.next().unwrap().to_owned();
            let value = pair.next().unwrap().to_owned();
            spell_dictionary.insert(key, value);
        }
        let mut autocomplete = SearchIndexBuilder::default()
            .fuzzy_length(1)
            .fuzzy_minimum_score(0.4)
            .dump_keyword(None)
            .build();
        let index_keys = spell_dictionary
            .keys()
            .map(|x| x.clone())
            .collect::<Vec<String>>();
        index_keys
            .iter()
            .enumerate()
            .for_each(|(num, element)| autocomplete.insert(&num, element));

        Self {
            graphics_index,
            spell_index: index,
            spell_dictionary,
            args,
            image: DynamicImage::new_rgba8(1, 1),
            expression: String::new(),
            image_result_path: get_absolute_path_from_relative(&format!("{ASSET_PATH}/output.png")),
            autocompleter: autocomplete,
        }
    }
    fn get_default(&self) -> DynamicImage {
        match image::ImageReader::open(&format!(
            "{}/data/ui_gfx/inventory/full_inventory_box.png",
            &self.args.data_path
        )) {
            Ok(reader) => match reader.decode() {
                Ok(image) => image,
                Err(e) => {
                    println!("Failed to decode inventory background: {:?}", e);
                    return DynamicImage::new_rgba8(1, 1);
                }
            },
            Err(e) => {
                println!("Failed to read inventory background: {:?}", e);
                return DynamicImage::new_rgba8(1, 1);
            }
        }
    }
    fn encode_png(image: &DynamicImage) -> Vec<u8> {
        let mut output = Vec::new();
        let encoder = image::codecs::png::PngEncoder::new(&mut output);
        match image.write_with_encoder(encoder) {
            Ok(_) => (),
            Err(e) => {
                println!("Failed to encode image: {:?}", e);
            }
        };
        output
    }
    pub fn copy_image(&self) {
        self.image.save(&self.image_result_path).unwrap();

        let path = get_absolute_path_from_relative(&format!("{ASSET_PATH}"));

        match process::Command::new("./clip")
            .current_dir(path)
            .arg("-i")
            .arg("output.png")
            .stdin(process::Stdio::null())
            .spawn()
        {
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
            let mut image = self.get_default();
            if scale != 1 {
                image = image::DynamicImage::resize(
                    &image,
                    image.width() * resize_scale,
                    image.height() * resize_scale,
                    image::imageops::FilterType::Nearest,
                );
            }
            return (RustApplication::encode_png(&image), image.height());
        }

        let spells_per_row: u32 = self.args.spells_per_row;

        let width = cmp::min(tokens.len() as u32, spells_per_row) * scale;
        let height = ((tokens.len() as u32 - 1) / spells_per_row + 1) * scale;

        let mut x: u32 = 0;
        let mut y: u32 = 0;

        let mut image = image::DynamicImage::new_rgba8(width, height);
        for token in tokens {
            let word = token.to_uppercase();
            let token = self.spell_dictionary.get(&word).unwrap_or(&word);
            let (spell_bg, spell_sprite) = match self.graphics_index.get(token) {
                Some((spell_type, sprite_path)) => (
                    image::ImageReader::open(&format!(
                        "{}/{}",
                        &self.args.data_path,
                        spell_type.bg_path()
                    ))
                    .unwrap()
                    .decode()
                    .unwrap(),
                    image::ImageReader::open(&format!("{}/{}", &self.args.data_path, sprite_path))
                        .unwrap()
                        .decode()
                        .unwrap(),
                ),
                None => (
                    image::DynamicImage::new_rgba8(20, 20),
                    image::DynamicImage::new_rgba8(20, 20),
                ),
            };
            image::imageops::overlay(&mut image, &self.get_default(), x as i64, y as i64);
            image::imageops::overlay(&mut image, &spell_bg, x as i64, y as i64);
            image::imageops::overlay(&mut image, &spell_sprite, x as i64 + 2, y as i64 + 2);
            x += scale;
            if x >= image.width() {
                x = 0;
                y += scale;
            }
        }
        if scale != 1 {
            image = image::DynamicImage::resize(
                &image,
                image.width() * resize_scale,
                image.height() * resize_scale,
                image::imageops::FilterType::Nearest,
            );
        }
        self.image = image;

        (
            RustApplication::encode_png(&self.image),
            self.image.height(),
        )
    }
    pub fn autocomplete_expression(&self) -> (String, Vec<String>) {
        let mut tokens = self.expression.split(' ').collect::<Vec<&str>>();

        let query = tokens.pop().unwrap_or("");

        let result: Vec<String> = self.autocompleter.autocomplete_with(
            &indicium::simple::AutocompleteType::Keyword,
            &(self.args.num_of_autocomplete_options as usize),
            &query,
        );
        (
            tokens.join(" "),
            result.iter().map(|x| x.to_uppercase()).collect(),
        )
    }
    pub fn get_number_of_suggestions(&self) -> u32 {
        self.args.num_of_autocomplete_options
    }
    pub fn fetch_eval_tree(&self, ansi: &str) -> String {
        if self.args.mod_path.is_empty() || self.args.data_path.is_empty() {
            return String::from("Error: Mod path or data path not set");
        }

        let empty_string = String::new();
        let spell_list = self
            .expression
            .split(' ')
            .map(|x| match self.spell_dictionary.get(&x.to_uppercase()) {
                Some(y) => y,
                None => self.spell_index.get(x).unwrap_or(&empty_string),
            })
            .filter(|x| !x.is_empty())
            .collect::<Vec<&String>>();

        if spell_list.len() == 0 {
            return String::from("No spells set.");
        }

        let output = process::Command::new("luajit")
            .current_dir(get_absolute_path_from_relative(&format!(
                "{ASSET_PATH}/wand_eval_tree"
            )))
            .arg("main.lua")
            .arg("-a")
            .arg(ansi)
            .arg("-d")
            .arg(&self.args.drained)
            .arg("-e")
            .arg(&self.args.every_other)
            .arg("-u")
            .arg(&self.args.unlimited_spells)
            .arg("-sc")
            .arg(&format!("{}", self.args.spells_per_cast))
            .arg("-ma")
            .arg(&format!("{}", self.args.max_mana))
            .arg("-mc")
            .arg(&format!("{}", self.args.mana_charge))
            .arg("-rt")
            .arg(&format!("{}", self.args.reload_time))
            .arg("-cd")
            .arg(&format!("{}", self.args.cast_delay))
            .arg("-nc")
            .arg(&format!("{}", self.args.number_of_casts))
            .arg("-mp")
            .arg(&self.args.mod_path)
            .arg("-dp")
            .arg(&self.args.data_path)
            .arg("-sp")
            .args(spell_list.as_slice())
            .output()
            .unwrap();
        let string = String::from_utf8(output.stdout)
            .unwrap_or("Something went wrong, probably mod/data filepaths".to_string());
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
            .arg("-t")
            .arg("output.txt")
            .stdin(process::Stdio::null())
            .spawn()
        {
            Ok(_) => (),
            Err(e) => println!("Failed to daemonize: {:?}", e),
        }
    }
}
