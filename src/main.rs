/*
 * bliss-analyze
 *
 * Copyright (c) 2022 Craig Drummond <craig.p.drummond@gmail.com>
 *
 * ----
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 *
 */


use bliss_audio::Song;
use std::env;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    for path in &args {
        match Song::new(&path) {
            Ok(song) => {
                let vals: Vec<f32> = song.analysis.as_vec();
                if args.len()>1 {
                    print!("{}: ", path);
                }
                for v in &vals {
                    print!("{:.18} ", v);
                }
            },
            Err(e) => {
                if args.len()>1 {
                    println!("{}: {}", path, e);
                } else {
                    println!("{}", e);
                }
            }
        }
    }
}

