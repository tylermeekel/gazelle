use super::Layer;

pub struct LayerStack {
    pub layers: Vec<Box<dyn Layer>>,
}

impl LayerStack {
    pub fn create() -> Self {
        Self { layers: Vec::new() }
    }

    // Pushes a layer to the front of the layer stack
    pub fn push_layer(&mut self, layer: Box<dyn Layer>) {
        self.layers.insert(0, layer)
    }

    // Pushes a layer to the back of the stack, so that it operates overtop other layers
    pub fn push_overlay(&mut self, layer: Box<dyn Layer>) {
        self.layers.push(layer)
    }
}
