use pyxel::{Pyxel, PyxelCallback};
use std::sync::{Arc, Mutex};

const SZ: usize = 31;

// For the wasm version, the App state has to be protected
// against deallocation to avoid memory corruption!
struct App {
    t: Arc<Mutex<[[i16; SZ]; SZ]>>,
    spd: u16,
}

impl App {
    fn init() {
        let mut pyxel = pyxel::init(
            SZ as u32,
            SZ as u32,
            Some("SAND"),
            None,
            None,
            None,
            None,
            None,
        );
        let spd = 1;
        let t = Arc::new(Mutex::new([[0; SZ]; SZ]));
        t.lock().unwrap()[SZ / 2][SZ / 2] = 1024;
        pyxel.run(App { t, spd });
    }

    fn color(v: i16) -> u8 {
        match v {
            0 => 0,
            1 => 7,
            2 => 13,
            3 => 1,
            _ => 8,
        }
    }
}

impl PyxelCallback for App {
    fn update(&mut self, pyxel: &mut Pyxel) {
        let mut t = self.t.lock().unwrap();
        for _ in 0..self.spd {
            for y in 0..SZ {
                for x in 0..SZ {
                    let v = t[y][x];
                    if v >= 4 {
                        t[y][x] = v - 4;
                        if x > 0 {
                            t[y][x - 1] += 1;
                        }
                        if x < SZ - 1 {
                            t[y][x + 1] += 1;
                        }
                        if y > 0 {
                            t[y - 1][x] += 1;
                        }
                        if y < SZ - 1 {
                            t[y + 1][x] += 1;
                        }
                    }
                }
            }
        }

        if pyxel.btnp(pyxel::KEY_Q, None, None) {
            pyxel.quit();
        }
        if pyxel.btnp(pyxel::KEY_SPACE, None, None) {
            t[SZ / 2][SZ / 2] += 128;
        }
        if pyxel.btnp(pyxel::KEY_Z, None, None) {
            for j in 0..SZ {
                for i in 0..SZ {
                    t[i][j] = 0;
                }
            }
        }
        if pyxel.btnp(pyxel::KEY_RIGHT, None, None) {
            self.spd *= 2;
        }
        if pyxel.btnp(pyxel::KEY_LEFT, None, None) && self.spd > 1 {
            self.spd /= 2;
        }
    }

    fn draw(&mut self, pyxel: &mut Pyxel) {
        let t = self.t.lock().unwrap();
        pyxel.cls(3);
        for y in 0..SZ {
            for x in 0..SZ {
                pyxel.pset(x as f64, y as f64, App::color(t[y][x]));
            }
        }
    }
}

fn main() {
    App::init();
}
