
// #[derive(Clone,Eq,PartialEq)]
// pub enum LayerUse {
//     Tilemap,
//     Selection,
//     Overlay,
// }

pub struct Layers {
    // layers: vec![
    //     (LayerKind::Dense,  LayerUse::Tilemap),
    //     (LayerKind::Dense,  LayerUse::Tilemap),
    //     (LayerKind::Dense,  LayerUse::Tilemap),
    //     (LayerKind::Dense,  LayerUse::Overlay),
    //     (LayerKind::Sparse, LayerUse::Selection),
    // ],
}

// pub fn get_layer(&self, layer: LayerUse) -> usize {
//     self.layers
//         .iter()
//         .position(|(k,u)| u == &layer)
//         .expect("No layer for type")
// }

// pub fn max_layer(&self) -> usize {
//     self.layers.len()
// }

// pub fn max_tilemap_layer(&self) -> usize {
//     self.layers
//         .iter()
//         .rev()
//         .position(|(k,u)| u == &LayerUse::Tilemap)
//         .expect("No max tilemap layer")
// }