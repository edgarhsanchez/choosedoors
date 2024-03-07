use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use rand::{self, Rng};
use crossterm::event::{DisableMouseCapture, EnableMouseCapture};

use crossterm::terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};
use std::io::{stdout};
use tui::backend::CrosstermBackend;
use tui::layout::{Constraint, Layout, Direction};
use tui::widgets::{Block, Borders, Row, Table};
use tui::Terminal;
use tui::style::{Style};
use rayon::prelude::*;

#[derive()]
struct Game {
    good_door: i8,
    door_removed: i8,
    first_choice: i8,
    second_choice: i8,
}

pub trait Play {
    fn remove_door(&mut self);
    fn select_door(&mut self);
}

fn pick_one(one: i8, two: i8) -> i8 {
    if rand::thread_rng().gen() { // generates a random boolean
        one
    } else {
        two
    }
}




impl Play for Game {
    fn remove_door(&mut self) {
        match self.good_door {
            1 => match self.first_choice {
                1 => self.door_removed = pick_one(2,3),
                2 => self.door_removed = 3,
                3 => self.door_removed = 2,
                _ => todo!(),
            },
            2 => match self.first_choice {
                1 => self.door_removed = 3,
                2 => self.door_removed = pick_one(1,3),
                3 => self.door_removed = 1,
                _ => todo!(),
            },
            3 => match self.first_choice {
                1 => self.door_removed = 2,
                2 => self.door_removed = 1,
                3 => self.door_removed = pick_one(1,2),
                _ => todo!(),
            },
            _ => todo!(),
        }
    }

    fn select_door(&mut self) {
        match self.first_choice {
            0 => self.first_choice = rand::thread_rng().gen_range(1..=3),
            1 => match self.door_removed {
                2 => self.second_choice = pick_one(1,3),
                3 => self.second_choice = pick_one(1, 2),
                _ => todo!(),
            },
            2 => match self.door_removed {
                1 => self.second_choice = pick_one(2, 3),
                3 => self.second_choice = pick_one(1, 2),
                _ => todo!(),
            },
            3 => match self.door_removed {
                1 => self.second_choice = pick_one(2, 3),
                2 => self.second_choice = pick_one(1, 3),
                _ => todo!(),
            },
            _ => todo!(),
        }
    }
}


struct GameStats {
    total_played: f64,
    total_switched: f64,
    total_stayed: f64,
    total_switched_won: f64,
    total_switched_lost: f64,
    total_stayed_won: f64,
    total_stayed_lost: f64,
}

// fn print_stats(stats: &GameStats) {
//     let switched_won_perc = if stats.total_switched == 0.0 { 0.0 } else { stats.total_switched_won * 100.0 / stats.total_switched };
//     let switched_lost_perc = if stats.total_switched == 0.0 { 0.0 } else { stats.total_switched_lost * 100.0 / stats.total_switched };
//     let stayed_won_perc = if stats.total_stayed == 0.0 { 0.0 } else { stats.total_stayed_won * 100.0 / stats.total_stayed };
//     let stayed_lost_perc = if stats.total_stayed == 0.0 { 0.0 } else { stats.total_stayed_lost * 100.0 / stats.total_stayed };
//     println!("played: {:.1}, switched won: {:.1}, switched lost: {:.1}, stayed won: {:.1}, stayed lost: {:.1}", 
//         stats.total_played, 
//         switched_won_perc,
//         switched_lost_perc,
//         stayed_won_perc,
//         stayed_lost_perc
//     )
// }

fn display_table(game_moves_hash: &HashMap<i8, HashMap<String, i64>>) {
    let stdout = stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend).unwrap();
    terminal.draw(|f| {
        let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints(
            [
                Constraint::Percentage(33),
                Constraint::Percentage(33),
                Constraint::Percentage(34),
            ].as_ref()
        )
        .split(f.size());

    for (i, (key, game_moves)) in game_moves_hash.iter().enumerate() {
        let total_count: i64 = game_moves.values().sum();
        let mut entries: Vec<(&String, &i64)> = game_moves.iter().collect();
        entries.sort_by_key(|entry| entry.0);

        let mut data: Vec<Row> = vec![];
        for (key, value) in &entries {
            let percentage = (**value as f64 * 100.0 / total_count as f64);
            data.push(Row::new(vec![
                key.to_string(), 
                value.to_string(), 
                format!("{:.0}%", percentage)
            ]));
        }

        let table = Table::new(data)
            .block(Block::default().title(format!("Winning Number {}", key)).borders(Borders::ALL))
            .widths(&[Constraint::Percentage(33), Constraint::Percentage(33), Constraint::Percentage(33)])
            .column_spacing(1)
            .style(Style::default().fg(tui::style::Color::White))
            .header(
                Row::new(vec!["Choice", "Count", "Percentage"])
                    .style(Style::default().fg(tui::style::Color::Yellow))
                    .bottom_margin(1),
            );

        f.render_widget(table, chunks[i]);
    }}).unwrap();
}

fn main() {
    
    // let stats = &mut GameStats{
    //     total_played: 0.0,
    //     total_switched: 0.0,
    //     total_stayed: 0.0,
    //     total_switched_won: 0.0,
    //     total_switched_lost:0.0,
    //     total_stayed_won: 0.0,
    //     total_stayed_lost: 0.0,
    // };
    let mut game_moves: Arc<Mutex<HashMap<i8, HashMap<String, i64>>>> = Arc::new(Mutex::new(HashMap::new()));
    
    (1..=10_000_000_000i64).into_par_iter().for_each(|i| {
        
        let game_moves_clone = Arc::clone(&game_moves);
        let mut rng = rand::thread_rng();

        let mut game_moves_local = game_moves_clone.lock().unwrap();
        let mut game = Game{
            good_door: rng.gen_range(1..=3),
            first_choice: 0,
            second_choice: 0,
            door_removed: 0
        };
        game.select_door();
        game.remove_door();
        game.select_door();

        // stats.total_played += 1.0;
        let game_entry_hash = game_moves_local.entry(game.good_door).or_insert(HashMap::new());

        if game.first_choice != game.second_choice {
            // stats.total_switched += 1.0;
            if game.second_choice != game.good_door {
                let mut game_moves_entry_lost = game_entry_hash.entry(format!("lost {}-{}", game.first_choice, game.second_choice)).or_insert(0);
                // stats.total_switched_lost += 1.0;
                *game_moves_entry_lost += 1;

                let mut game_moves_entry_switched_lost = game_entry_hash.entry(format!("switch lost")).or_insert(0);
                *game_moves_entry_switched_lost += 1;
            } else {
                let mut game_moves_entry_won = game_entry_hash.entry(format!("won {}-{}", game.first_choice, game.second_choice)).or_insert(0);
                // stats.total_switched_won += 1.0;
                *game_moves_entry_won += 1;

                let mut game_moves_entry_switched_won = game_entry_hash.entry(format!("switch won")).or_insert(0);
                *game_moves_entry_switched_won += 1;
            }
        }
        else {
            // stats.total_stayed += 1.0;
            if game.second_choice != game.good_door {
                let entry_lost = game_entry_hash.entry(format!("lost {}-{}", game.first_choice, game.second_choice)).or_insert(0);
                *entry_lost += 1;
                let entry_stayed_lost = game_entry_hash.entry(format!("stayed lost")).or_insert(0);
                *entry_stayed_lost += 1;
            } else {
                let entry_won = game_entry_hash.entry(format!("won {}-{}", game.first_choice, game.second_choice)).or_insert(0);
                *entry_won += 1;
                let entry_stayed_won = game_entry_hash.entry(format!("stayed won")).or_insert(0);
                *entry_stayed_won += 1;
            }
        }
        if i % 1000 == 0 {
            print!("\x1B[2J\x1B[1;1H");
            display_table(&game_moves_local);

            // thread::sleep(Duration::from_millis(1));
        }        

    });
 
}


