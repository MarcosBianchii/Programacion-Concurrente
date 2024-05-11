//! Tag struct models the stats of a tag in the data.

use serde::Serialize;

use std::cmp::Ordering;

#[derive(Clone, Copy, Default, Eq, Serialize)]
pub struct Tag {
    pub questions: usize,
    pub words: usize,
}

impl PartialEq for Tag {
    fn eq(&self, other: &Self) -> bool {
        (self.score() - other.score()).abs() < f32::EPSILON
    }
}

impl PartialOrd for Tag {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Tag {
    fn cmp(&self, other: &Self) -> Ordering {
        let score_a = (100.0 * self.score()) as u64;
        let score_b = (100.0 * other.score()) as u64;
        score_a.cmp(&score_b)
    }
}

impl Tag {
    /// Instanciate a new `Tag` with the given values.
    pub fn with(questions: usize, words: usize) -> Self {
        Self { questions, words }
    }

    /// Calculates the scoring given the amount of
    /// words and questions for this tag.
    pub fn score(&self) -> f32 {
        self.words as f32 / self.questions as f32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ordering() {
        let tag1 = Tag::with(20, 2000);
        let tag2 = Tag::with(10, 1000);
        assert_eq!(Ordering::Equal, tag1.cmp(&tag2));

        let tag1 = Tag::with(35, 10000);
        let tag2 = Tag::with(1, 100);
        assert_eq!(Ordering::Greater, tag1.cmp(&tag2));

        let tag1 = Tag::with(1, 10000000000000);
        let tag2 = Tag::with(1, 10000000000001);
        assert_eq!(Ordering::Equal, tag1.cmp(&tag2));

        // 11,884,204.203139277760416460638378
        let tag1 = Tag::with(758391, 9012873509823);
        // 33,513,223,976.470521666571738098435
        let tag2 = Tag::with(17258591, 578391025701298357);
        assert_eq!(Ordering::Less, tag1.cmp(&tag2));

        let tag1 = Tag::with(2, 10000001);
        let tag2 = Tag::with(2, 10000000);
        assert_eq!(Ordering::Greater, tag1.cmp(&tag2));
    }
}
