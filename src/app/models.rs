#[derive(Clone, Debug)]
pub struct AlbumDescription {
    pub title: String,
    pub artist: String,
    pub uri: String,
    pub art: String,
    pub songs: Vec<SongDescription>,
    pub id: String
}

impl PartialEq for AlbumDescription {
    fn eq(&self, other: &AlbumDescription) -> bool {
        self.id == other.id
    }
}

impl Eq for AlbumDescription {}

#[derive(Clone, Debug)]
pub struct SongDescription {
    pub title: String,
    pub artist: String,
    pub uri: String,
    pub duration: u32
}

impl SongDescription {
    pub fn new(title: &str, artist: &str, uri: &str, duration: u32) -> Self {
        Self { title: title.to_string(), artist: artist.to_string(), uri: uri.to_string(), duration }
    }
}