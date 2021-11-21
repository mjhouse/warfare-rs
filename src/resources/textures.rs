use std::collections::HashMap;
use bevy::sprite::TextureAtlas;
use bevy::asset::AssetServer;

pub struct TextureResource {
    textures: HashMap<&'static str,usize>,
}

fn index(server: &AssetServer, atlas: &TextureAtlas, name: &str) -> usize {
    atlas.get_texture_index(
        &server.get_handle(
            format!("textures/{}.png",name).as_str()))
                .expect(format!("Texture doesn't exist: {}",name).as_str())
}

impl TextureResource {

    pub fn new(server: &AssetServer, atlas: &TextureAtlas) -> Self {
        let labels = vec![
            "water",
            "grass_1",
            "grass_2",
            "grass_3",
            "grass_4",
            "clay",
            "sand",
            "silt",
            "peat",
            "chalk",
            "loam",
            "blank",
            "mark",
        ];

        let textures = labels
            .iter()
            .map(|&l| ( l, index(server,atlas,l) ))
            .collect();

        Self { textures }
    }

    pub fn get(&self, label: &str) -> usize {
        self.textures[label]
    }

}