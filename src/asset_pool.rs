use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::Read;
use crate::zip_reader::unzip_assets;

use serde_derive::{Deserialize};

pub struct AssetPool {
    textures: HashMap<String, BlockSchematic>,
    shaders: HashMap<String, String>,
}

const ACTIVE_TEXTURE_PACK_DIR: &str = "assets/textures/active/Default_Texture.zip";

impl AssetPool {
    /// Initializes asset pool by loading in texture pack from active directory and deserializing it
    /// to the in memory asset cache.
    pub fn initialize_pool() -> Self {

        // Load Shaders
        let shader_paths = fs::read_dir("assets/shaders/")
            .expect("Error occurred reading in shader data!");

        let mut shaders = HashMap::new();
        for shader_path in shader_paths {
            if shader_path.is_err() {
                continue;
            }

            let entry = shader_path.unwrap();

            let mut file = File::open(entry.path()).expect("Error occurred opening file");
            let mut contents = String::new();
            file.read_to_string(&mut contents).expect("Could not read in shader contents");

            let key = String::from(entry.path().file_stem().unwrap().to_str().unwrap());
            shaders.insert(String::from(key), contents);
        }

        // Call zip reader and begin texture mapping
        unzip_assets(ACTIVE_TEXTURE_PACK_DIR);

        // Load textures
        let texture_paths = fs::read_dir("assets/textures/current/assets/minecraft/models/block/")
            .expect("Error occurred reading in block texture metadata!");


        let mut textures = HashMap::new();
        for texture_path in texture_paths {
            if texture_path.is_err() {
                continue;
            }

            let entry = texture_path.unwrap();

            let mut file = File::open(entry.path()).expect("Error occurred opening file");
            let mut contents = String::new();
            file.read_to_string(&mut contents).expect("Could not read in texture schematic contents!");

            let block_schematic: BlockSchematic = serde_json::from_str(&contents).unwrap();

            let key = String::from(entry.file_name().to_str().unwrap().strip_suffix(".json").unwrap());

            textures.insert(String::from(key), block_schematic);
        }

        return Self {
            textures,
            shaders,
        };
    }

    /// Returns texture object based on supplied id.
    /// TODO: This should return a texture object not a schematic to a texture mapping
    pub fn get_texture(self, id: &str) -> Option<BlockSchematic> {
        return self.textures.get(id).cloned();
    }

    /// Returns wgsl shader instructions based on the shader id.
    pub fn get_shader(self, id: &str) -> Option<String> {
        return self.shaders.get(id).cloned();
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct BlockSchematic {
    parent: Option<String>,
    ambientocclusion: Option<bool>,
    textures: Option<BlockTextures>,
    elements: Option<Vec<Elements>>,
}

#[derive(Clone, Debug, Deserialize)]
struct Item {
    parent: String,
    textures: ItemTextures,
}

#[derive(Clone, Debug, Deserialize)]
struct ItemTextures {
    layer0: String,
}

#[derive(Clone, Debug, Deserialize)]
struct BlockTextures {
    texture: Option<String>,
    all: Option<String>,
    bottom: Option<String>,
    top: Option<String>,
    front: Option<String>,
    back: Option<String>,
    side: Option<String>,
    end: Option<String>,
    edge: Option<String>,
    inside: Option<String>,
    plant: Option<String>,
    fire: Option<String>,
    wood: Option<String>,
    cross: Option<String>,
    wall: Option<String>,
    pane: Option<String>,
    pattern: Option<String>,
    crop: Option<String>,
    particle: Option<String>,
    base: Option<String>,
    level: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
struct Elements {
    from: Option<Vec<f32>>,
    to: Option<Vec<f32>>,
    shade: Option<bool>,
    rotation: Option<Rotation>,
    faces: Option<Faces>,
}

#[derive(Clone, Debug, Deserialize)]
struct Rotation {
    origin: Vec<f32>,
    axis: String,
    angle: f32,
    rescale: Option<bool>,
}

#[derive(Clone, Debug, Deserialize)]
struct Faces {
    down: Option<Face>,
    up: Option<Face>,
    north: Option<Face>,
    south: Option<Face>,
    east: Option<Face>,
    west: Option<Face>,
}

#[derive(Clone, Debug, Deserialize)]
struct Face {
    uv: Option<Vec<f32>>,
    texture: String,
    cullface: Option<String>,
    tintindex: Option<i8>,
}

#[derive(Clone, Debug, Deserialize)]
struct Particles {
    textures: Vec<String>,
}
