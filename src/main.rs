extern crate clap;
extern crate ncurses;

use clap::{App, Arg};
use ncurses::*;
use std::io::stdin;

fn main() {
    let app = App::new("lifegame")
        .arg(
            Arg::with_name("random")
                .long("random")
                .help("Set random map flag"),
        )
        .arg(
            Arg::with_name("height")
                .long("height")
                .help("Set height")
                .takes_value(true)
                .default_value("10"),
        )
        .arg(
            Arg::with_name("width")
                .long("width")
                .help("Set width")
                .takes_value(true)
                .default_value("10"),
        )
        .arg(
            Arg::with_name("auto_time")
                .short("n")
                .help("Set auto display time")
                .takes_value(true),
        );

    let matches = app.get_matches();
    let h = matches.value_of("height").unwrap().parse().unwrap();
    let w = matches.value_of("width").unwrap().parse().unwrap();

    let hw: Vec<usize> = vec![h, w];

    let mut map = Vec::new();
    if matches.is_present("random") {

    } else {
        for _ in 0..hw[0] {
            let mut l = String::new();
            stdin().read_line(&mut l).unwrap();
            let line: Vec<bool> = l.trim()
                .chars()
                .map(|c| match c {
                    '.' => false,
                    '#' => true,
                    _ => panic!("Invalid"),
                })
                .collect();
            assert_eq!(line.len(), hw[1]);
            map.push(line);
        }
    }

    let mut game = LifeGame::new(map);

    initscr();
    keypad(stdscr(), true);
    noecho();

    while !game.is_end() {
        game.next();
        printw(&game.output_new_str());
        refresh();
        getch();
        clear();
        refresh();
    }

    endwin();
}

struct LifeGame {
    now_map: Vec<Vec<bool>>,
    new_map: Vec<Vec<bool>>,
}

impl LifeGame {
    fn new(now: Vec<Vec<bool>>) -> Self {
        Self {
            now_map: Vec::new(),
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

    fn output_new_str(&mut self) -> String {
        let m = self.new_map.clone();
        let mut result = String::new();
        for v in m {
            result += &format!(
                "{}\n",
                v.iter()
                    .map(|t| match t {
                        true => "#",
                        false => ".",
                    })
                    .collect::<Vec<&str>>()
                    .concat()
            );
        }
        result
    }

    fn is_end(&mut self) -> bool {
        self.now_map == self.new_map
    }
}

trait Live {
    fn is_live(&self, h: usize, w: usize) -> bool;
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
                if mm as usize == h && nn as usize == w {
                    continue;
                }
                if let Some(b) = self.get(mm as usize).and_then(|a| a.get(nn as usize)) {
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
}

#[cfg(test)]
mod tests {
    use super::*;

    /**
     * init(vec![
     * ".#.#",
     * "#.#.",
     * ".#.#",
     * "#.#.",
     * ])
     */
    fn init(m: Vec<&str>) -> LifeGame {
        let v = m.iter()
            .map(|b| {
                b.chars()
                    .map(|c| match c {
                        '.' => false,
                        '#' => true,
                        _ => panic!("Invalid"),
                    })
                    .collect()
            })
            .collect();
        LifeGame::new(v)
    }

    #[test]
    fn is_live_test3() {
        let mut game = init(vec![".#.", ".#.", ".#."]);
        assert!(!game.is_end());
        game.next();

        let n = game.output_new_str();
        assert_eq!(n, "...\n###\n...\n");
    }
}
