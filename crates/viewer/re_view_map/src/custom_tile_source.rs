use walkers::TileId;
use walkers::sources::{Attribution, TileSource};

pub struct CustomTileSource {
    url_pattern: String,
}

impl CustomTileSource {
    pub fn new(url_pattern: String) -> Self {
        Self { url_pattern }
    }
}

impl TileSource for CustomTileSource {
    fn tile_url(&self, tile_id: TileId) -> String {
        self.url_pattern
            .replace("{z}", &tile_id.zoom.to_string())
            .replace("{x}", &tile_id.x.to_string())
            .replace("{y}", &tile_id.y.to_string())
    }

    fn attribution(&self) -> Attribution {
        Attribution {
            text: "Custom Tiles",
            url: "",
            logo_light: None,
            logo_dark: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tile(zoom: u8, x: u32, y: u32) -> TileId {
        TileId { zoom, x, y }
    }

    #[test]
    fn test_tile_url_substitution() {
        let src = CustomTileSource::new("https://example.com/{z}/{x}/{y}.png".to_owned());
        assert_eq!(
            src.tile_url(tile(5, 12, 8)),
            "https://example.com/5/12/8.png"
        );
    }

    #[test]
    fn test_tile_url_boundary_values() {
        let src = CustomTileSource::new("https://tiles.example.org/tiles/{z}/{x}/{y}".to_owned());
        assert_eq!(
            src.tile_url(tile(0, 0, 0)),
            "https://tiles.example.org/tiles/0/0/0"
        );
        assert_eq!(
            src.tile_url(tile(18, 131072, 131072)),
            "https://tiles.example.org/tiles/18/131072/131072"
        );
    }

    #[test]
    fn test_tile_url_missing_placeholder_unchanged() {
        // A pattern missing {y} should leave that part as-is.
        let src = CustomTileSource::new("https://example.com/{z}/{x}/0.png".to_owned());
        assert_eq!(
            src.tile_url(tile(3, 4, 99)),
            "https://example.com/3/4/0.png"
        );
    }

    #[test]
    fn test_tile_url_placeholder_order_independent() {
        let src = CustomTileSource::new("https://example.com/{y}/{x}/{z}.png".to_owned());
        assert_eq!(
            src.tile_url(tile(7, 10, 20)),
            "https://example.com/20/10/7.png"
        );
    }
}
