use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::Read;
use crate::zip_reader::unzip_assets;

pub struct AssetPool {
    textures: HashMap<String, BlockModel>,
    shaders: HashMap<String, String>
}

const ACTIVE_TEXTURE_PACK_DIR: &str = "assets/textures/active/Default_Texture.zip";

impl AssetPool {

    /// Initializes asset pool by loading in texture pack from active directory and deserializing it
    /// to the in memory asset cache.
    pub fn initialize_pool() -> Self {

        // Call zip reader and begin texture mapping
        unzip_assets(ACTIVE_TEXTURE_PACK_DIR);

        // TODO: Load shaders

        // Load textures
        let paths = fs::read_dir("assets/textures/current/assets/minecraft/models/block/")
            .expect("Error occurred reading in block texture metadata!");


        let mut textures = HashMap::new();
        for path in paths {
            if path.is_err() {
                continue;
            }

            let entry = path.unwrap();

            let mut file = File::open(entry.path()).expect("Error occurred opening file");
            let mut contents = String::new();
            file.read_to_string(&mut contents).expect("Could not read in dirt");

            let block: BlockModel = serde_json::from_str(&contents).unwrap();

            let key = String::from(entry.file_name().to_str().unwrap().strip_suffix(".json").unwrap());

            textures.insert(String::from(key), block);
        }
        return Self {
            textures,
            shaders: HashMap::new()
        }
    }

    /// Returns texture object based on supplied id.
    /// TODO: This should return a texture object not a schematic to a texture mapping
    pub fn get_texture(self, id: &str) -> Option<BlockModel> {
        return self.textures.get(id).cloned();
    }

    /// Returns wgsl shader instructions based on the shader id.
    pub fn get_shader(self, id: &str) -> Option<String> {
        return self.shaders.get(id).cloned();
    }
}

use serde_derive::{Deserialize};

#[derive(Clone, Debug, Deserialize)]
pub struct BlockModel {
    parent: Option<String>,
    ambientocclusion: Option<bool>,
    textures: Option<BlockTextures>,
    elements: Option<Vec<Elements>>
}

#[derive(Clone, Debug, Deserialize)]
struct Item {
    parent: String,
    textures: ItemTextures
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
    level: Option<String>
}

#[derive(Clone, Debug, Deserialize)]
struct Elements {
    from: Option<Vec<f32>>,
    to: Option<Vec<f32>>,
    shade: Option<bool>,
    rotation: Option<Rotation>,
    faces: Option<Faces>
}

#[derive(Clone, Debug, Deserialize)]
struct Rotation {
    origin: Vec<f32>,
    axis: String,
    angle: f32,
    rescale: Option<bool>
}

#[derive(Clone, Debug, Deserialize)]
struct Faces {
    down: Option<Face>,
    up: Option<Face>,
    north: Option<Face>,
    south: Option<Face>,
    east: Option<Face>,
    west: Option<Face>
}

#[derive(Clone, Debug, Deserialize)]
struct Face {
    uv: Option<Vec<f32>>,
    texture: String,
    cullface: Option<String>,
    tintindex: Option<i8>
}

#[derive(Clone, Debug, Deserialize)]
struct Particles {
    textures: Vec<String>
}
