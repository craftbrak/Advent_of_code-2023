use std::fmt;
use std::fmt::Display;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Range {
    start: i64,
    end: i64,
}

impl Range {
    ///Returns a new range based on the start and and values
    pub fn new(start: i64, end: i64) -> Result<Range, String> {
        if start > end {
            return Err(format!("Start ({}) is less than end ({})", start, end));
        }

        return Ok(Self {
            start,
            end,
        });
    }
    ///Returns a new range based on the start and len values
    pub fn with_length(start: i64, len: i64) -> Result<Self, String> {
        Self::new(start, start + len - 1)
    }

    /// Returns the start of the range
    pub fn start(&self) -> i64 { self.start }

    /// Returns the end of the range
    pub fn end(&self) -> i64 { self.end }

    /// Returns the len of the range
    pub fn len(&self) -> i64 { self.end - self.start + 1 }

    /// Recreates a new range with the new start position
    pub fn move_start_at(&self, start: i64) -> Result<Self, String> {
        Self::with_length(start, self.len())
    }
    /// Returns True if the value is inside the range
    pub fn is_in_range(&self, number: i64) -> bool {
        self.start <= number && self.end >= number
    }
    /// Returns True if the range is coliding with the other (shares at least a value)
    pub fn collide(&self, other: &Self) -> bool {
        self.is_in_range(other.start)
            || self.is_in_range(other.end)
            || other.is_in_range(self.start)
            || other.is_in_range(self.end)
    }
    /// Returns a new range that is the intersection between the 2 ranges
    pub fn intersect(&self, other: &Self) -> Result<Self, String> {
        if !self.collide(other) {
            return Err("Cannot intersect for ranges that doesn't collide".to_string());
        }

        return Ok(Self::new(
            self.start.max(other.start),
            self.end.min(other.end),
        ).unwrap());
    }
    pub fn iter(&self) -> impl Iterator<Item=i64> {
        self.start..=self.end
    }
    /// Returns a vec of ranges
    pub fn diff(&self, other: &Self) -> Vec<Self> {
        // other is outside self
        if other.start <= self.start && other.end >= self.end {
            return vec![];
        }

        // other is inside self
        if other.start > self.start && other.end < self.end {
            return vec![
                Self::new(self.start, other.start - 1).unwrap(),
                Self::new(other.end + 1, self.end).unwrap(),
            ];
        }

        let start: i64;
        let end: i64;

        // collide on side
        if self.start < other.start {
            start = self.start.min(other.start);
            end = self.start.max(other.start) - 1;
        } else {
            start = self.end.min(other.end) + 1;
            end = self.end.max(other.end);
        }

        return vec![Self::new(start, end).unwrap()];
    }

}
impl Display for Range {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.start, self.end)
    }
}
#[cfg(test)]
mod tests {
    use crate::range::Range;

    #[test]
    fn is_in_range() {
        let range = Range::new(5, 7).unwrap();

        assert!(!range.is_in_range(4));
        assert!(range.is_in_range(5));
        assert!(range.is_in_range(6));
        assert!(range.is_in_range(7));
        assert!(!range.is_in_range(8));
    }

    #[test]
    fn collide() {
        let range = Range::new(3, 5).unwrap();

        assert!(!range.collide(&Range::new(1, 2).unwrap()));
        assert!(range.collide(&Range::new(2, 3).unwrap()));
        assert!(range.collide(&Range::new(3, 5).unwrap()));
        assert!(range.collide(&Range::new(4, 6).unwrap()));
        assert!(range.collide(&Range::new(5, 10).unwrap()));
        assert!(!range.collide(&Range::new(6, 12).unwrap()));
        assert!(range.collide(&Range::new(1, 7).unwrap()));
    }

    #[test]
    fn intersect() {
        let range = Range::new(3, 6).unwrap();

        assert_eq!(Range::new(3, 3), range.intersect(&Range::new(2, 3).unwrap()));
        assert_eq!(Range::new(3, 4), range.intersect(&Range::new(2, 4).unwrap()));
        assert_eq!(Range::new(3, 5), range.intersect(&Range::new(1, 5).unwrap()));
        assert_eq!(Range::new(3, 6), range.intersect(&Range::new(0, 6).unwrap()));
        assert_eq!(Range::new(3, 6), range.intersect(&Range::new(0, 7).unwrap()));
        assert_eq!(Range::new(4, 5), range.intersect(&Range::new(4, 5).unwrap()));
        assert_eq!(Range::new(5, 6), range.intersect(&Range::new(5, 7).unwrap()));
    }
}