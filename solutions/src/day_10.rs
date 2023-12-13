use crate::Solver;
use anyhow::{Error, Result};
use std::fmt::{Display, Formatter};
use thiserror::Error;

#[derive(Error, Debug)]
enum GridError {
    #[error("no tile at start position")]
    NoStartTile,
    #[error("tile at start position is not of start kind")]
    BadStartTileKind,
    #[error("start tile can not be determined")]
    IndeterminateStart,
    #[error("non grid char encountered")]
    BadCharParse,
    #[error("traversal of leaky pipe system")]
    LeakyPipeSystem,
}

#[derive(PartialEq, Eq, Copy, Clone)]
enum Cardinal {
    North,
    West,
    East,
    South,
}

impl Cardinal {
    fn inverse(self) -> Self {
        match self {
            Cardinal::North => Cardinal::South,
            Cardinal::West => Cardinal::East,
            Cardinal::East => Cardinal::West,
            Cardinal::South => Cardinal::North,
        }
    }
}

#[derive(Clone, PartialEq, Eq)]
enum PipeKind {
    Vertical,
    Horizontal,
    NorthEast,
    NorthWest,
    SouthWest,
    SouthEast,
}

impl PipeKind {
    fn enter(&self, entry: Cardinal) -> Option<Cardinal> {
        match self {
            PipeKind::Vertical => {
                if entry == Cardinal::North {
                    return Some(Cardinal::South);
                }
                if entry == Cardinal::South {
                    return Some(Cardinal::North);
                }
            }
            PipeKind::Horizontal => {
                if entry == Cardinal::West {
                    return Some(Cardinal::East);
                }
                if entry == Cardinal::East {
                    return Some(Cardinal::West);
                }
            }
            PipeKind::NorthEast => {
                if entry == Cardinal::North {
                    return Some(Cardinal::East);
                }
                if entry == Cardinal::East {
                    return Some(Cardinal::North);
                }
            }
            PipeKind::NorthWest => {
                if entry == Cardinal::North {
                    return Some(Cardinal::West);
                }
                if entry == Cardinal::West {
                    return Some(Cardinal::North);
                }
            }
            PipeKind::SouthWest => {
                if entry == Cardinal::South {
                    return Some(Cardinal::West);
                }
                if entry == Cardinal::West {
                    return Some(Cardinal::South);
                }
            }
            PipeKind::SouthEast => {
                if entry == Cardinal::South {
                    return Some(Cardinal::East);
                }
                if entry == Cardinal::East {
                    return Some(Cardinal::South);
                }
            }
        }

        None
    }
}

#[derive(Clone, PartialEq, Eq)]
enum StartKind {
    Unknown,
    Pipe(PipeKind),
}

#[derive(Clone, PartialEq, Eq)]
enum TileKind {
    RegularPipe(PipeKind),
    StartPipe(StartKind),
    Ground,
}

#[derive(Clone, PartialEq, Eq)]
struct Tile {
    position: Point2,
    tile_kind: TileKind,
}

#[derive(Copy, Clone, Default, PartialEq, Eq)]
struct Point2 {
    x: i32,
    y: i32,
}

impl Point2 {
    fn translate(self, cardinal: Cardinal) -> Self {
        match cardinal {
            Cardinal::North => {
                let mut point = self;
                point.y -= 1;

                point
            }
            Cardinal::West => {
                let mut point = self;
                point.x -= 1;

                point
            }
            Cardinal::East => {
                let mut point = self;
                point.x += 1;

                point
            }
            Cardinal::South => {
                let mut point = self;
                point.y += 1;

                point
            }
        }
    }
}

#[derive(Default, Clone)]
struct Grid {
    start: Point2,
    tiles: Vec<Vec<Tile>>,
}

impl Grid {
    fn tile(&self, position: Point2) -> Option<&Tile> {
        if position.x < 0 || position.y < 0 {
            return None;
        }

        let x = position.x as usize;
        let y = position.y as usize;

        self.tiles.get(y).and_then(|row| row.get(x))
    }

    fn tile_mut(&mut self, position: Point2) -> Option<&mut Tile> {
        if position.x < 0 || position.y < 0 {
            return None;
        }

        let x = position.x as usize;
        let y = position.y as usize;

        self.tiles.get_mut(y).and_then(|row| row.get_mut(x))
    }

    fn clean(&mut self) {
        let mut cleaner = false;

        let mut ground_positions = vec![];

        for row in self.tiles.iter() {
            for tile in row.iter() {
                if tile.regular_pipe() && tile.connections(self) < 2 {
                    ground_positions.push(tile.position);
                }
            }
        }

        if !ground_positions.is_empty() {
            cleaner = true;

            for ground_position in ground_positions {
                let ground = self.tile_mut(ground_position).unwrap();

                ground.tile_kind = TileKind::Ground;
            }
        }

        if cleaner {
            self.clean();
        }
    }

    fn determine_start(&mut self) -> Result<()> {
        let start = self
            .tile(self.start)
            .ok_or(Error::from(GridError::NoStartTile))?;

        let TileKind::StartPipe(start_kind) = &start.tile_kind else {
            return Err(Error::from(GridError::BadStartTileKind));
        };

        match start_kind {
            StartKind::Unknown => {}
            StartKind::Pipe(_) => return Ok(()),
        }

        if start.connections(self) != 2 {
            return Err(Error::from(GridError::IndeterminateStart));
        }

        let north = start
            .north(self)
            .map(|north| north.tile_kind.connects_south())
            .unwrap_or(false);
        let west = start
            .west(self)
            .map(|west| west.tile_kind.connects_east())
            .unwrap_or(false);
        let east = start
            .east(self)
            .map(|east| east.tile_kind.connects_west())
            .unwrap_or(false);
        let south = start
            .south(self)
            .map(|south| south.tile_kind.connects_north())
            .unwrap_or(false);

        let start = self.tile_mut(self.start).unwrap();

        start.tile_kind = TileKind::StartPipe(StartKind::Pipe(match (north, west, east, south) {
            (true, true, false, false) => PipeKind::NorthWest,
            (true, false, true, false) => PipeKind::NorthEast,
            (true, false, false, true) => PipeKind::Vertical,
            (false, true, true, false) => PipeKind::Horizontal,
            (false, true, false, true) => PipeKind::SouthWest,
            (false, false, true, true) => PipeKind::SouthEast,
            _ => return Err(Error::from(GridError::IndeterminateStart)),
        }));

        Ok(())
    }
}

impl Tile {
    fn east<'a>(&'a self, grid: &'a Grid) -> Option<&Tile> {
        let mut position = self.position;
        position.x += 1;

        grid.tile(position)
    }

    fn west<'a>(&'a self, grid: &'a Grid) -> Option<&Tile> {
        let mut position = self.position;
        position.x -= 1;

        grid.tile(position)
    }

    fn north<'a>(&'a self, grid: &'a Grid) -> Option<&Tile> {
        let mut position = self.position;
        position.y -= 1;

        grid.tile(position)
    }

    fn south<'a>(&'a self, grid: &'a Grid) -> Option<&Tile> {
        let mut position = self.position;
        position.y += 1;

        grid.tile(position)
    }

    fn connections(&self, grid: &Grid) -> usize {
        let mut connections = 0;

        if self.tile_kind.connects_north() {
            if let Some(north) = self.north(grid) {
                if north.tile_kind.connects_south() {
                    connections += 1;
                }
            }
        }

        if self.tile_kind.connects_west() {
            if let Some(west) = self.west(grid) {
                if west.tile_kind.connects_east() {
                    connections += 1;
                }
            }
        }

        if self.tile_kind.connects_east() {
            if let Some(east) = self.east(grid) {
                if east.tile_kind.connects_west() {
                    connections += 1;
                }
            }
        }

        if self.tile_kind.connects_south() {
            if let Some(south) = self.south(grid) {
                if south.tile_kind.connects_north() {
                    connections += 1;
                }
            }
        }

        connections
    }

    fn regular_pipe(&self) -> bool {
        match self.tile_kind {
            TileKind::RegularPipe(_) => true,
            _ => false,
        }
    }

    fn traverse<'a>(&'a self, grid: &'a Grid, entrypoint: Cardinal) -> Result<(&Tile, Cardinal)> {
        let leaky = Error::from(GridError::LeakyPipeSystem);

        let pipe = match &self.tile_kind {
            TileKind::RegularPipe(kind) | TileKind::StartPipe(StartKind::Pipe(kind)) => kind,
            _ => return Err(leaky),
        };

        let exit = pipe.enter(entrypoint).ok_or(leaky)?;
        let leaky = Error::from(GridError::LeakyPipeSystem);

        let position = self.position.translate(exit);
        let tile = grid.tile(position).ok_or(leaky)?;
        let entrypoint = exit.inverse();

        Ok((tile, entrypoint))
    }
}

impl TileKind {
    fn connects_north(&self) -> bool {
        match self {
            TileKind::StartPipe(StartKind::Pipe(pipe)) | TileKind::RegularPipe(pipe) => {
                pipe.connects_north()
            }
            TileKind::StartPipe(StartKind::Unknown) => true,
            TileKind::Ground => false,
        }
    }

    fn connects_west(&self) -> bool {
        match self {
            TileKind::StartPipe(StartKind::Pipe(pipe)) | TileKind::RegularPipe(pipe) => {
                pipe.connects_west()
            }
            TileKind::StartPipe(StartKind::Unknown) => true,
            TileKind::Ground => false,
        }
    }

    fn connects_east(&self) -> bool {
        match self {
            TileKind::StartPipe(StartKind::Pipe(pipe)) | TileKind::RegularPipe(pipe) => {
                pipe.connects_east()
            }
            TileKind::StartPipe(StartKind::Unknown) => true,
            TileKind::Ground => false,
        }
    }

    fn connects_south(&self) -> bool {
        match self {
            TileKind::StartPipe(StartKind::Pipe(pipe)) | TileKind::RegularPipe(pipe) => {
                pipe.connects_south()
            }
            TileKind::StartPipe(StartKind::Unknown) => true,
            TileKind::Ground => false,
        }
    }
}

impl PipeKind {
    fn connects_north(&self) -> bool {
        match self {
            PipeKind::Vertical | PipeKind::NorthWest | PipeKind::NorthEast => true,
            _ => false,
        }
    }

    fn connects_west(&self) -> bool {
        match self {
            PipeKind::Horizontal | PipeKind::NorthWest | PipeKind::SouthWest => true,
            _ => false,
        }
    }

    fn connects_east(&self) -> bool {
        match self {
            PipeKind::Horizontal | PipeKind::NorthEast | PipeKind::SouthEast => true,
            _ => false,
        }
    }

    fn connects_south(&self) -> bool {
        match self {
            PipeKind::Vertical | PipeKind::SouthEast | PipeKind::SouthWest => true,
            _ => false,
        }
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let last = self.tiles.len() - 1;

        for (index, row) in self.tiles.iter().enumerate() {
            for tile in row {
                write!(
                    f,
                    "{}",
                    match &tile.tile_kind {
                        TileKind::StartPipe(StartKind::Pipe(kind))
                        | TileKind::RegularPipe(kind) => match kind {
                            PipeKind::Vertical => '|',
                            PipeKind::Horizontal => '-',
                            PipeKind::NorthEast => 'L',
                            PipeKind::NorthWest => 'J',
                            PipeKind::SouthWest => '7',
                            PipeKind::SouthEast => 'F',
                        },
                        TileKind::StartPipe(StartKind::Unknown) => 'S',
                        TileKind::Ground => '.',
                    }
                )?;
            }
            if index != last {
                writeln!(f)?;
            }
        }

        Ok(())
    }
}

#[derive(Default)]
pub struct Day {
    input: String,
    grid: Grid,
}

impl Solver for Day {
    fn setup(&mut self, input: &str) {
        self.input = input.to_string();
    }

    fn parse(&mut self) -> Result<()> {
        for (y, line) in self.input.lines().enumerate() {
            let mut row = vec![];
            for (x, char) in line.chars().enumerate() {
                let tile_kind = match char {
                    '|' => TileKind::RegularPipe(PipeKind::Vertical),
                    '-' => TileKind::RegularPipe(PipeKind::Horizontal),
                    'L' => TileKind::RegularPipe(PipeKind::NorthEast),
                    'J' => TileKind::RegularPipe(PipeKind::NorthWest),
                    '7' => TileKind::RegularPipe(PipeKind::SouthWest),
                    'F' => TileKind::RegularPipe(PipeKind::SouthEast),
                    '.' => TileKind::Ground,
                    'S' => TileKind::StartPipe(StartKind::Unknown),
                    _ => return Err(Error::from(GridError::BadCharParse)),
                };

                let position = Point2 {
                    x: x as i32,
                    y: y as i32,
                };

                if tile_kind == TileKind::StartPipe(StartKind::Unknown) {
                    self.grid.start = position;
                }

                row.push(Tile {
                    tile_kind,
                    position,
                })
            }

            self.grid.tiles.push(row);
        }

        self.grid.clean();
        self.grid.determine_start()?;

        Ok(())
    }

    fn part_1(&self) -> Result<String> {
        let start_tile = self.grid.tile(self.grid.start).unwrap();

        let TileKind::StartPipe(StartKind::Pipe(start_pipe)) = &start_tile.tile_kind else {
            return Err(Error::from(GridError::IndeterminateStart));
        };

        let mut entry = None;

        for direction in [
            Cardinal::North,
            Cardinal::West,
            Cardinal::East,
            Cardinal::South,
        ] {
            if let Some(exit) = start_pipe.enter(direction) {
                entry = Some(exit);
                break;
            }
        }

        let mut entry = entry.ok_or(Error::from(GridError::IndeterminateStart))?;
        let mut tile = start_tile;

        let mut steps = 0;

        loop {
            (tile, entry) = tile.traverse(&self.grid, entry)?;
            steps += 1;

            if tile == start_tile {
                break;
            }
        }

        let farthest = steps / 2;

        Ok(farthest.to_string())
    }

    fn part_2(&self) -> Result<String> {
        Ok("Placeholder".into())
    }
}
