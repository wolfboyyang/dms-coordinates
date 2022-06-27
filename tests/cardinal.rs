use dms_coordinates::Cardinal;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn is_northern() {
        assert_eq!(Cardinal::North.is_northern(), true);
        assert_eq!(Cardinal::NorthEast.is_northern(), true);
        assert_eq!(Cardinal::NorthWest.is_northern(), true);
        assert_eq!(Cardinal::South.is_northern(), false);
        assert_eq!(Cardinal::SouthEast.is_northern(), false);
        assert_eq!(Cardinal::SouthWest.is_northern(), false);
        assert_eq!(Cardinal::East.is_northern(), false);
        assert_eq!(Cardinal::West.is_northern(), false);
    }
    #[test]
    fn is_southern() {
        assert_eq!(Cardinal::North.is_southern(), false);
        assert_eq!(Cardinal::NorthEast.is_southern(), false);
        assert_eq!(Cardinal::NorthWest.is_southern(), false);
        assert_eq!(Cardinal::South.is_southern(), true);
        assert_eq!(Cardinal::SouthEast.is_southern(), true);
        assert_eq!(Cardinal::SouthWest.is_southern(), true);
        assert_eq!(Cardinal::East.is_southern(), false);
        assert_eq!(Cardinal::West.is_southern(), false);
    }
    #[test]
    fn is_eastern() {
        assert_eq!(Cardinal::North.is_eastern(), false);
        assert_eq!(Cardinal::NorthEast.is_eastern(), true);
        assert_eq!(Cardinal::NorthWest.is_eastern(), false);
        assert_eq!(Cardinal::South.is_eastern(), false);
        assert_eq!(Cardinal::SouthEast.is_eastern(), true);
        assert_eq!(Cardinal::SouthWest.is_eastern(), false);
        assert_eq!(Cardinal::East.is_eastern(), true);
        assert_eq!(Cardinal::West.is_eastern(), false);
    }
    #[test]
    fn is_western() {
        assert_eq!(Cardinal::North.is_western(), false);
        assert_eq!(Cardinal::NorthEast.is_western(), false);
        assert_eq!(Cardinal::NorthWest.is_western(), true);
        assert_eq!(Cardinal::South.is_western(), false);
        assert_eq!(Cardinal::SouthEast.is_western(), false);
        assert_eq!(Cardinal::SouthWest.is_western(), true);
        assert_eq!(Cardinal::East.is_western(), false);
        assert_eq!(Cardinal::West.is_western(), true);
    }
    #[test]
    fn is_sub_quadrant() {
        assert_eq!(Cardinal::North.is_sub_quadrant(), false);
        assert_eq!(Cardinal::West.is_sub_quadrant(), false);
        assert_eq!(Cardinal::East.is_sub_quadrant(), false);
        assert_eq!(Cardinal::South.is_sub_quadrant(), false);
        assert_eq!(Cardinal::NorthEast.is_sub_quadrant(), true);
        assert_eq!(Cardinal::NorthWest.is_sub_quadrant(), true);
        assert_eq!(Cardinal::SouthEast.is_sub_quadrant(), true);
        assert_eq!(Cardinal::SouthWest.is_sub_quadrant(), true);
    }
}