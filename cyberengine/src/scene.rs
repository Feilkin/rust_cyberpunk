// This is me trying to make scene graphs with no internet access
use resource;

#[derive(Debug)]
pub enum Component<'a> {
    Group(Vec<Component<'a>>),
    Tilemap(&'a resource::tilemap::Tilemap),
}

#[derive(Debug)]
pub struct Scene<'a> {
    pub root: Vec<Component<'a>>,
}

impl<'a> Scene<'a> {
    pub fn add_component(&mut self, comp: Component<'a>) -> () {
        self.root.push(comp);
    }
}
