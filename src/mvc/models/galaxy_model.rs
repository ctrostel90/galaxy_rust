#[derive(Clone, Default, Debug, PartialEq)]
pub struct GalaxyModel{
    pub name: String,
    pub position: Coord,
    pub number_planets: i32,
    pub sun_color: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Coord{
    pub x: u8,
    pub y: u8,
}

impl Default for Coord{
    fn default() -> Self {
        Coord{
            x:0,
            y:0,
        }
    }
}