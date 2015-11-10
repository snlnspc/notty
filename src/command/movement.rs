use command::prelude::*;
use datatypes::{Direction, Movement};
use datatypes::Direction::*;
use datatypes::Movement::*;

#[derive(Copy, Clone)]
pub struct Move {
    movement: Movement,
}

impl Move {
    pub fn new(movement: Movement) -> Move {
        Move {
            movement: movement,
        }
    }
}

impl Command for Move {
    fn apply(&self, screen: &mut Screen, _: &mut FnMut(InputEvent)) {
        screen.move_cursor(self.movement);
    }
    fn repr(&self) -> String {
        match self.movement {
            To(Up, n, _)        => format!("MOVE UP {}", n),
            To(Down, n, _)      => format!("MOVE DOWN {}", n),
            To(Left, n, _)      => format!("MOVE LEFT {}", n),
            To(Right, n, _)     => format!("MOVE RIGHT {}", n),
            PreviousLine(n)     => format!("MOVE PREV LINE {}", n),
            NextLine(n)         => format!("MOVE NEXT LINE {}", n),
            Tab(Up, n, _)       => format!("MOVE UP TAB {}", n),
            Tab(Down, n, _)     => format!("MOVE DOWN TAB {}", n),
            Tab(Left, n, _)     => format!("MOVE LEFT TAB {}", n),
            Tab(Right, n, _)    => format!("MOVE RIGHT TAB {}", n),
            IndexTo(Up, n)      => format!("MOVE UP INDEX {}", n),
            IndexTo(Down, n)    => format!("MOVE DOWN INDEX {}", n),
            IndexTo(Left, n)    => format!("MOVE LEFT INDEX {}", n),
            IndexTo(Right, n)   => format!("MOVE RIGHT INDEX {}", n),
            Column(n)           => format!("MOVE TO COL {}", n),
            Row(n)              => format!("MOVE TO ROW {}", n),
            Position(coords)    => format!("MOVE TO {},{}", coords.x, coords.y),
            ToEdge(Up)          => String::from("MOVE UP TO EDGE"),
            ToEdge(Down)        => String::from("MOVE DOWN TO EDGE"),
            ToEdge(Left)        => String::from("MOVE LEFT TO EDGE"),
            ToEdge(Right)       => String::from("MOVE RIGHT TO EDGE"),
            ToBeginning         => String::from("MOVE TO BEGINNING"),
            ToEnd               => String::from("MOVE TO END"),
        }
    }
}

pub struct ScrollScreen {
    dir: Direction,
    n: u32,
}

impl ScrollScreen {
    pub fn new(dir: Direction, n: u32) -> ScrollScreen {
        ScrollScreen {
            dir: dir,
            n: n
        }
    }
}

impl Command for ScrollScreen {
    fn apply(&self, screen: &mut Screen, _: &mut FnMut(InputEvent)) {
        screen.scroll(self.dir, self.n)
    }
    fn repr(&self) -> String {
        String::from("SCROLL SCREEN")
    }
}