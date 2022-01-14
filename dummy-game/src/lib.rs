pub mod dummy {
    #[derive(Clone)]
    pub struct Coord {
        pub x: usize,
        pub y: usize,
    }
    pub enum Direction {
        Up,
        Down,
        Left,
        Right,
        Stop,
    }
    pub struct DummyGame {
        map: Vec<Vec<char>>,
        size: Coord,
        current: Coord,
    }

    impl DummyGame {
        fn fit(&self, coord: Coord) -> Coord {
            let x = if coord.x > self.size.x - 1 {
                self.size.x - 1
            } else {
                coord.x
            };

            let y = if coord.y > self.size.y - 1 {
                self.size.y - 1
            } else {
                coord.y
            };

            Coord { x, y }
        }

        pub fn init(width: usize, height: usize, start: Coord) -> Self {
            let mut map = Vec::new();
            for _ in 0..height {
                let mut v = Vec::new();
                for _ in 0..width {
                    v.push('O');
                }
                map.push(v);
            }

            map[start.x][start.y] = 'X';

            Self {
                map,
                size: Coord {
                    x: width,
                    y: height,
                },
                current: start,
            }
        }

        pub fn move_to(&mut self, dir: Direction) {
            let next = match dir {
                Direction::Up => {
                    if self.current.y != 0 {
                        Coord {
                            x: self.current.x,
                            y: self.current.y - 1,
                        }
                    } else {
                        self.current.clone()
                    }
                }
                Direction::Down => self.fit(Coord {
                    x: self.current.x,
                    y: self.current.y + 1,
                }),
                Direction::Left => {
                    if self.current.x != 0 {
                        Coord {
                            x: self.current.x - 1,
                            y: self.current.y,
                        }
                    } else {
                        self.current.clone()
                    }
                }
                Direction::Right => self.fit(Coord {
                    x: self.current.x + 1,
                    y: self.current.y,
                }),
                _=> self.current.clone()
            };

            self.map[self.current.y][self.current.x] = 'O';
            self.current = next;
            self.map[self.current.y][self.current.x] = 'X';
        }

        pub fn as_string(&self) -> String {
            self.map
                .iter()
                .map(|v| {
                    let string: String = v.iter().collect();
                    format!("{}\n", string)
                })
                .collect()
        }
    }
}
