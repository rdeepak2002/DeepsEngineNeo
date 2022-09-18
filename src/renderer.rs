pub(crate) trait Renderer {
    fn update(&self);
}

pub(crate) struct OpenGLRenderer();

impl Renderer for OpenGLRenderer {
    fn update(&self) {}
}
