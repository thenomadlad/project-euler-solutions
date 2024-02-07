#[derive(Debug)]
pub struct Triangle {
    pub values: Vec<u32>,
}

#[derive(Debug)]
pub struct TriangleElement<'a> {
    pub value: u32,

    row: usize,
    col: usize,
    triangle: &'a Triangle,
}

impl Triangle {
    pub fn new(triangle_string: &str) -> Triangle {
        let num_rows = triangle_string.trim().lines().count();

        let values = triangle_string
            .trim()
            .lines()
            .flat_map(Triangle::split_line)
            .collect::<Vec<u32>>();

        assert_eq!((1..=num_rows).sum::<usize>(), values.len());

        Triangle { values }
    }

    pub fn get_root(&self) -> TriangleElement<'_> {
        TriangleElement {
            value: *self.values.first().unwrap(),
            row: 1,
            col: 1,
            triangle: self,
        }
    }

    pub fn get_item_at(&self, row: usize, col: usize) -> Option<TriangleElement<'_>> {
        let pos: usize = (1..row).sum::<usize>() + (col - 1);

        self.values.get(pos).copied().map(|value| TriangleElement {
            value,
            row,
            col,
            triangle: self,
        })
    }

    fn split_line(line: &str) -> Vec<u32> {
        line.trim()
            .split(' ')
            .map(|item| item.parse().unwrap())
            .collect()
    }
}

impl<'a> TriangleElement<'a> {
    pub fn get_left_child(&'a self) -> Option<TriangleElement<'a>> {
        self.triangle.get_item_at(self.row + 1, self.col)
    }

    pub fn get_right_child(&'a self) -> Option<TriangleElement<'a>> {
        self.triangle.get_item_at(self.row + 1, self.col + 1)
    }
}

#[cfg(test)]
mod test {
    use super::Triangle;

    const MINI: &str = r#"
    1
    "#;

    const SMALL: &str = r#"
    1
    2 3
    "#;

    const LARGE: &str = r#"
    1
    2 3
    4 5 6
    7 8 9 10
    11 12 13 14 15
    16 17 18 19 20 21
    "#;

    #[test]
    fn creates_mini_triangle() {
        let t = Triangle::new(MINI);
        assert_eq!(t.get_root().value, 1);
    }

    #[test]
    fn small_triangle_second_row() {
        let t = Triangle::new(SMALL);
        assert_eq!(t.get_root().value, 1);
        assert_eq!(t.get_item_at(2, 1).unwrap().value, 2);
        assert_eq!(t.get_item_at(2, 2).unwrap().value, 3);
        assert!(t.get_item_at(3, 1).is_none());
    }

    #[test]
    fn mini_triangle_node_branches() {
        let t = Triangle::new(SMALL);
        assert_eq!(t.get_root().value, 1);
        assert_eq!(t.get_root().get_left_child().unwrap().value, 2);
        assert_eq!(t.get_root().get_right_child().unwrap().value, 3);
    }

    #[test]
    fn large_triangle_node_branches() {
        let t = Triangle::new(LARGE);
        assert_eq!(t.get_root().value, 1);
        assert_eq!(
            t.get_root()
                .get_left_child()
                .unwrap()
                .get_left_child()
                .unwrap()
                .get_right_child()
                .unwrap()
                .value,
            8
        );
        assert_eq!(
            t.get_root()
                .get_right_child()
                .unwrap()
                .get_right_child()
                .unwrap()
                .get_left_child()
                .unwrap()
                .get_right_child()
                .unwrap()
                .value,
            14
        );
    }
}
