struct BlockModel {
    parent: String,
    textures: BlockTextures,
    elements: Elements
}

struct Item {
    parent: String,
    textures: ItemTextures
}

struct ItemTextures {
    layer0: String,
}

struct BlockTextures {
    texture: String,
    all: String,
    bottom: String,
    top: String,
    front: String,
    back: String,
    side: String,
    end: String,
    edge: String,
    inside: String,
    plant: String,
    fire: String,
    wood: String,
    cross: String,
    wall: String,
    pane: String,
    pattern: String,
    crop: String,
    particle: String
}

struct Elements {
    from: Vec<f32>,
    to: Vec<f32>,
    shade: bool,
}

struct Particles {
    textures: Vec<String>
}