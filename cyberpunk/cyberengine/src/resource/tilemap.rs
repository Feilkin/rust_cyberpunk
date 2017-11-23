//! Complex Tiled Implementation
//!
//! The Tilemap is loaded from Tiled JSON file. (unless someone implements some
//! other loaders Kappa). It should handle loading all the required stuff
//! (Tilesets, Scripts?), and rendering itself.
//!

use std::fs::File;
use std::io::prelude::*;
use std::path::{Path, PathBuf};
use serde_json;
use serde_json::Value;
use serde_json::map::Map;

use genmesh::{Quad, EmitTriangles, MapVertex, Triangulate, MapToVertices, Vertices};
use genmesh::generators::Plane;

use graphics;

gfx_defines! {
    vertex TileVertex {
        pos: [f32; 2] = "a_Pos",
        uv: [f32; 2]  = "a_Uv",
        color: [f32; 3] = "a_Color",
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct MapObject {}

#[derive(Debug, Clone)]
pub struct Terrain {}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum LayerData {
    TileData(Vec<i32>),
    ObjectData(Map<String, Value>),
}

#[derive(Debug, Serialize, Deserialize)]

/*
    width       int     Column count. Same as map width for fixed-size maps.
    height      int     Row count. Same as map height for fixed-size maps.
    name        string  Name assigned to this layer
    type        string  “tilelayer”, “objectgroup”, or “imagelayer”
    visible     bool    Whether layer is shown or hidden in editor
    x           int     Horizontal layer offset in tiles. Always 0.
    y           int     Vertical layer offset in tiles. Always 0.
    data        int     Array of GIDs. tilelayer only.
    objects     object  Array of Objects. objectgroup only.
    properties  object  string key-value pairs.
    opacity     float   Value between 0 and 1
    draworder   string  “topdown” (default) or “index”. objectgroup only.
*/
pub struct Layer {
    pub height: i32,
    pub width: i32,
    pub name: String,
    #[serde(rename = "type")]
    pub layertype: String,
    pub visible: bool,
    pub x: i32,
    pub y: i32,
    pub data: LayerData,
    pub properties: Option<Map<String, Value>>,
    pub opacity: f64,
    #[serde(skip_serializing, skip_deserializing)]
    _plane: Vec<graphics::texture::Vertex>,
}

impl Layer {
    fn setup_plane(&mut self) -> () {
        use graphics::texture::Vertex;
        let vertex_data: Vec<Vertex> = Plane::subdivide(self.width as usize, self.height as usize)
            .vertex(|v| {
                Vertex {
                    pos: [v.pos[0], v.pos[1]],
                    uv: [0., 0.],
                    color: [1., 0., 1.],
                }
            })
            .map(|Quad {
                 x: v0,
                 y: v1,
                 z: v2,
                 w: v3,
             }| {
                Quad::new(
                    Vertex {
                        pos: v0.pos,
                        uv: [0., 64. / 1280.],
                        color: v0.color,
                    },
                    Vertex {
                        pos: v1.pos,
                        uv: [64. / 1728., 64. / 1280.],
                        color: v1.color,
                    },
                    Vertex {
                        pos: v2.pos,
                        uv: [64. / 1728., 0.],
                        color: v2.color,
                    },
                    Vertex {
                        pos: v3.pos,
                        uv: [0., 0.],
                        color: v3.color,
                    },
                )
            })
            .triangulate()
            .vertices()
            .collect();
    }

    #[allow(dead_code)]
    fn get_tile(&self, x: i32, y: i32) -> i32 {
        if self.layertype != "tilelayer" {
            panic!("Attempt to get a tile from non-tile layer {:?}", self);
        }

        if x < 0 || x > self.width {
            panic!("X coordinate out of bounds {:?}", self)
        }


        if y < 0 || y > self.height {
            panic!("Y coordinate out of bounds {:?}", self)
        }

        match self.data {
            LayerData::TileData(ref data) => data[(y * self.width + x) as usize],
            _ => panic!("Not a Tile layer?? {:?}", self),
        }
    }

    fn draw(&self, factory: &mut graphics::Factory) -> () {
        match self.data {
            LayerData::TileData(_) => self.draw_tilelayer(factory),
            LayerData::ObjectData(_) => self.draw_objectlayer(factory),
        }
    }

    fn draw_tilelayer(&self, factory: &mut graphics::Factory) -> () {}

    fn draw_objectlayer(&self, factory: &mut graphics::Factory) -> () {
        unimplemented!();
    }
}

/*
    firstgid        int     GID corresponding to the first tile in the set
    image           string  Image used for tiles in this set
    name            string  Name given to this tileset
    tilewidth       int     Maximum width of tiles in this set
    tileheight      int     Maximum height of tiles in this set
    imagewidth      int     Width of source image in pixels
    imageheight     int     Height of source image in pixels
    properties      object  String key-value pairs
    propertytypes   object  String key-value pairs
    margin          int     Buffer between image edge and first tile (pixels)
    spacing         int     Spacing between adjacent tiles in image (pixels)
    tileproperties  object  Per-tile properties, indexed by gid as string
    terrains        array   Array of Terrains (optional)
    columns         int     The number of tile columns in the tileset
    tilecount       int     The number of tiles in this tileset
    tiles           object  Gid-indexed Tiles (optional)
*/
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Tileset {
    #[serde(skip_serializing, skip_deserializing)]
    pub root: String,
    pub source: Option<String>,
    #[serde(skip_deserializing)]
    pub firstgid: i32,
    #[serde(rename = "firstgid")]
    pub _firstgid: Option<i32>,
    pub image: Option<String>,
    #[serde(skip_deserializing)]
    pub name: String,
    #[serde(rename = "name")]
    pub _name: Option<String>,
    pub tilewidth: Option<i32>,
    pub tileheight: Option<i32>,
    pub imagewidth: Option<i32>,
    pub imageheight: Option<i32>,
    pub properties: Option<Map<String, Value>>,
    pub propertytypes: Option<Map<String, Value>>,
    pub margin: Option<i32>,
    pub spacing: Option<i32>,
    pub tileproperties: Option<Map<String, Value>>,
    #[serde(skip_serializing, skip_deserializing)]
    terrains: Vec<Terrain>,
    pub columns: Option<i32>,
    pub tilecount: Option<i32>,
    //tiles: Map<String, Value>,
    #[serde(skip_serializing, skip_deserializing)]
    _texture: Option<graphics::texture::Texture<graphics::texture::pipe_tex::Meta, TileVertex>>,
}

impl Tileset {
    pub fn load_image(&mut self, factory: &mut graphics::Factory) -> () {
        let path: PathBuf = [
            &self.root,
            match self.image {
                Some(ref img) => img,
                None => panic!("No image specified {:?}", self),
            },
        ].into_iter()
            .collect();
        let texture = graphics::texture::Texture::from_file(factory, path.to_str().unwrap());
        self._texture = Some(texture);
    }

    pub fn get_texture(&self) -> Result<&graphics::texture::Texture<graphics::texture::pipe_tex::Meta, TileVertex>, String> {
        match self._texture {
            Some(ref texture) => Ok(texture),
            None => Err("Texture needs to be loaded first!".to_owned()),
        }
    }
}

/*
    version         number  The JSON format version
    tiledversion    string  The Tiled version used to save the file
    width           int     Number of tile columns
    height          int     Number of tile rows
    tilewidth       int     Map grid width.
    tileheight      int     Map grid height.
    orientation     string  Orthogonal, isometric, or staggered
    layers          array   Array of Layers
    tilesets        array   Array of Tilesets
    backgroundcolor string  Hex-formatted color (#RRGGBB or #AARRGGBB) (optional)
    renderorder     string  Rendering direction (orthogonal maps only)
    properties      object  String key-value pairs
    nextobjectid    int     Auto-increments for each placed object
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct Tilemap {
    #[serde(skip_serializing, skip_deserializing)]
    pub filename: String,
    pub version: f64,
    pub tiledversion: Option<String>,
    pub width: i32,
    pub height: i32,
    pub tilewidth: i32,
    pub tileheight: i32,
    pub orientation: String,
    pub layers: Vec<Layer>,
    pub tilesets: Vec<Tileset>,
    pub properties: Option<Map<String, Value>>,
    nextobjectid: i32,
}

impl Tilemap {
    pub fn new() -> Tilemap {
        Tilemap {
            filename: String::new(),
            version: 0.0,
            tiledversion: None,
            width: 0,
            height: 0,
            tilewidth: 0,
            tileheight: 0,
            orientation: String::new(),
            layers: Vec::new(),
            tilesets: Vec::new(),
            properties: None,
            nextobjectid: 0,
        }
    }


    pub fn from_tiled_json(
        filename: &str,
        factory: &mut graphics::Factory,
    ) -> Result<Tilemap, String> {
        println!("Loading Tilemap from {}", filename);

        let mut f = File::open(filename).expect("file not found");
        let mut contents = String::new();
        f.read_to_string(&mut contents).expect(
            "something went wrong reading the file",
        );

        let mut map: Tilemap = serde_json::from_str(&contents).unwrap();
        map.filename = filename.to_owned();
        map.load_tilesets(factory);
        Ok(map)
    }

    pub fn load_tilesets(&mut self, factory: &mut graphics::Factory) -> () {
        let root = Path::new(&self.filename)
            .parent()
            .unwrap()
            .to_str()
            .unwrap();
        let loaded: Vec<Tileset> = self.tilesets
            .iter()
            .map(|ts| {
                let mut newts = match ts.source {
                    Some(ref source) => {
                        let path: PathBuf = [root, source].into_iter().collect();
                        let mut loaded = load_tileset(path.to_str().unwrap()).unwrap();
                        loaded.firstgid = ts.firstgid;
                        loaded.root = path.parent().unwrap().to_str().unwrap().to_owned();
                        loaded.name = match loaded._name {
                            Some(ref name) => (*name).clone(),
                            None => panic!("Tileset needs a name {:?}", loaded),
                        };
                        loaded
                    }
                    None => {
                        let mut newts = (*ts).clone();
                        newts.firstgid = newts._firstgid.unwrap();
                        newts.root = root.to_owned();
                        newts.name = match newts._name {
                            Some(ref name) => (*name).clone(),
                            None => panic!("Tileset needs a name {:?}", newts),
                        };
                        newts
                    }
                };
                newts.load_image(factory);
                newts
            })
            .collect();
        self.tilesets = loaded;
    }
}

pub fn load_tileset(filename: &str) -> Result<Tileset, String> {
    println!("Loading Tileset from {}", filename);

    let mut f = File::open(filename).expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents).expect(
        "something went wrong reading the file",
    );

    let tileset: Tileset = serde_json::from_str(&contents).unwrap();
    Ok(tileset)
}
