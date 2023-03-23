use std::collections::HashMap;

struct AssetPool {
    textures: HashMap<String, BlockModel>,
    shaders: HashMap<String, String>
}

impl AssetPool {

    pub fn initialize_pool() -> Self {
        // Call zip reader and begin texture mapping

        // Load shaders

        todo!()
    }

    pub fn get_texture(self, id: String) -> BlockModel {
        // Returns texture object based on supplied id.
        todo!()
    }

    pub fn get_shader(self, id: String) -> String {
        // Returns wgsl file contents based on file name.
        todo!()
    }
}
