use bevy_tilemap::chunk::LayerKind;

#[derive(Clone,Eq,PartialEq)]
pub enum LayerUse {
    Tilemap,
    Selection,
    Overlay,
    Units,
}

#[derive(Clone)]
pub struct Layers {
    layers: Vec<(LayerKind,LayerUse)>,
}

impl Default for Layers {
    fn default() -> Self {
        Self {
            layers: vec![
                (LayerKind::Dense,  LayerUse::Tilemap),
                (LayerKind::Dense,  LayerUse::Tilemap),
                (LayerKind::Dense,  LayerUse::Tilemap),
                (LayerKind::Dense,  LayerUse::Overlay),
                (LayerKind::Sparse, LayerUse::Units),
                (LayerKind::Sparse, LayerUse::Selection),
            ],
        }
    }
}

impl Layers {
    pub fn get(&self, layer: &LayerUse) -> Option<usize> {
        self.nth(0,layer)
    }

    pub fn nth(&self, n: usize, layer: &LayerUse) -> Option<usize> {
        let mut i = 0;
        self.layers
            .iter()
            .position(|(k,u)| {
                let mut r = false;
                if u == layer {
                    if i == n {
                        r = true;
                    }
                    else {
                        i += 1;
                    }
                }
                r
            })
    }

    pub fn max(&self, layer: &LayerUse) -> Option<usize> {
        self.layers
            .iter()
            .rposition(|(k,u)| u == layer)
    }

    pub fn data(&self) -> Vec<(LayerKind,LayerUse)> {
        self.layers.clone()
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_get_nth() {
        let layers = Layers::default();

        // positive cases
        assert_eq!(layers.get(&LayerUse::Tilemap),Some(0));
        assert_eq!(layers.nth(0,&LayerUse::Tilemap),Some(0));
        assert_eq!(layers.nth(1,&LayerUse::Tilemap),Some(1));
        assert_eq!(layers.nth(2,&LayerUse::Tilemap),Some(2));
        assert_eq!(layers.nth(0,&LayerUse::Overlay),Some(3));
        assert_eq!(layers.nth(0,&LayerUse::Selection),Some(4));

        // negative cases
        assert_eq!(layers.nth(1,&LayerUse::Selection),None);
        assert_eq!(layers.nth(3,&LayerUse::Tilemap),None);
        assert_eq!(layers.nth(3,&LayerUse::Overlay),None);
        assert_eq!(layers.nth(2,&LayerUse::Overlay),None);
    }

    #[test]
    fn test_get_max() {
        let layers = Layers::default();

        assert_eq!(layers.max(&LayerUse::Tilemap),Some(2));
        assert_eq!(layers.max(&LayerUse::Overlay),Some(3));
        assert_eq!(layers.max(&LayerUse::Selection),Some(4));
    }

}