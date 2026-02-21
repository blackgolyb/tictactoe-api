#[derive(Debug, Clone)]
pub struct InvalidSegment {
    pub start: Point,
    pub end: Point,
}

#[derive(Debug, Clone)]
struct InvalidCoordinates {
    start: Point,
    end: Point,
}

pub type Point = (u64, u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SegmentDirection {
    Horizontal,
    Vertical,
    DiagonalAscending,
    DiagonalDescending,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Segment {
    start: Point,
    end: Point,
    direction: SegmentDirection,
}

impl Segment {
    pub fn new(start: Point, end: Point) -> Result<Self, InvalidSegment> {
        Ok(Segment {
            start,
            end,
            direction: Segment::calc_diraction(start, end)?,
        })
    }

    fn calc_diraction(start: Point, end: Point) -> Result<SegmentDirection, InvalidSegment> {
        if start.0 == end.0 && start.1 != end.1 {
            Ok(SegmentDirection::Vertical)
        } else if start.1 == end.1 && start.0 != end.0 {
            Ok(SegmentDirection::Horizontal)
        } else if (end.0 as i64 - start.0 as i64) == (end.1 as i64 - start.1 as i64)
            && end.0 > start.0
        {
            Ok(SegmentDirection::DiagonalDescending)
        } else if (end.0 as i64 - start.0 as i64) == -(end.1 as i64 - start.1 as i64)
            && end.0 > start.0
        {
            Ok(SegmentDirection::DiagonalAscending)
        } else {
            Err(InvalidSegment { start, end })
        }
    }

    pub fn start(&self) -> Point {
        self.start
    }

    pub fn end(&self) -> Point {
        self.end
    }

    pub fn direction(&self) -> SegmentDirection {
        self.direction
    }

    pub fn contains(&self, point: Point) -> bool {
        let (x, y) = point;
        let (start_x, start_y) = self.start;
        let (end_x, end_y) = self.end;

        let intersect_segment =
            // #[inline(always)]
        |start, end, target| {
            (start <= target && target <= end) || (end <= target && target <= start)
        };

        match self.direction {
            SegmentDirection::Horizontal => y == start_y && intersect_segment(start_x, end_x, x),
            SegmentDirection::Vertical => x == start_x && intersect_segment(start_y, end_y, y),
            SegmentDirection::DiagonalDescending => {
                // Points must be on the line y = x + c
                ((x as i64 - start_x as i64) == (y as i64 - start_y as i64))
                    && intersect_segment(start_x, end_x, x)
                    && intersect_segment(start_y, end_y, y)
            }
            SegmentDirection::DiagonalAscending => {
                // Points must be on the line y = -x + c
                ((x as i64 - start_x as i64) == -(y as i64 - start_y as i64))
                    && intersect_segment(start_x, end_x, x)
                    && intersect_segment(start_y, end_y, y)
            }
        }
    }
}

pub struct SegmentIter {
    current: Point,
    end: Point,
    direction: SegmentDirection,
    finished: bool,
}

impl Iterator for SegmentIter {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        if self.finished {
            return None;
        }

        let (x, y) = self.current;
        let (end_x, end_y) = self.end;

        let result = self.current;

        // Check if we've reached the end
        if self.current == self.end {
            self.finished = true;
            return Some(result);
        }

        self.current = match self.direction {
            SegmentDirection::Horizontal => (x + 1, y),
            SegmentDirection::Vertical => (x, y + 1),
            SegmentDirection::DiagonalDescending => (x + 1, y + 1),
            SegmentDirection::DiagonalAscending => (x + 1, y - 1),
        };

        Some(result)
    }
}

impl Segment {
    pub fn iter(&self) -> SegmentIter {
        SegmentIter {
            current: self.start,
            end: self.end,
            direction: self.direction,
            finished: false,
        }
    }
}
