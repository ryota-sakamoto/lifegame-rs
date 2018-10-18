extern crate ncurses;

use std::io::stdin;
use ncurses::*;

fn main() {
    let mut hw = String::new();
    stdin().read_line(&mut hw).unwrap();
    let hw: Vec<usize> = hw.trim().split(" ").map(|t|t.parse().unwrap()).collect();
    
    let mut map = Vec::new();
    for _ in 0..hw[0] {
        let mut l = String::new();
        stdin().read_line(&mut l).unwrap();
        let line: Vec<bool> = l.trim().chars().map(|c| match c {
            '.' => false,
            '#' => true,
            _ => panic!("Invalid"),
        }).collect();
        assert_eq!(line.len(), hw[1]);
        map.push(line);
    }

    let mut game = LifeGame::new(map);

    initscr();
    keypad(stdscr(), true);
    noecho();

    loop {
        game.next();

        for v in game.output_new() {
            printw(&format!("{}\n", v.iter().map(|t| match t {
                true => "#",
                false => ".",
            }).collect::<Vec<&str>>().concat()));
        }
        refresh();
        getch();
        clear();
        refresh();
    }
}

struct LifeGame {
    now_map: Vec<Vec<bool>>,
    new_map: Vec<Vec<bool>>,
}

impl LifeGame {
    fn new(now: Vec<Vec<bool>>) -> Self {
        Self {
            now_map: now.clone(),
            new_map: now,
        }
    }

    fn next(&mut self) {
        self.now_map = self.new_map.clone();
        for m in 0..self.now_map.len() {
            for n in 0..self.now_map[m].len() {
                self.new_map[m][n] = self.now_map.is_live(m, n);
            }
        }
    }

    fn output_new(&mut self) -> Vec<Vec<bool>> {
        self.new_map.clone()
    }
}

trait Live {
    fn is_live(&self, h: usize, w: usize) -> bool;
    fn is_end(&self) -> bool;
}

impl Live for Vec<Vec<bool>> {
    fn is_live(&self, h: usize, w: usize) -> bool {
        let mut live_count = 0;
        for m in -1..2 {
            let mm = h as i64 + m;
            if mm < 0 {
                continue;
            }
            for n in -1..2 {
                let nn = w as i64 + n;
                if nn < 0 {
                    continue;
                }
                if let Some(b) = self.get(mm as usize).and_then(|a|a.get(nn as usize)) {
                    if *b {
                        live_count += 1;
                    }
                }
            }
        }
        if self[h][w] {
            (live_count == 2 || live_count == 3)
        } else {
            live_count == 3
        }
    }
    
    fn is_end(&self) -> bool {
        false
    }
}