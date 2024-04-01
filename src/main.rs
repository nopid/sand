use pyxel::{Pyxel, PyxelCallback};

const SZ: usize = 31;

struct App<'a> {
    t: &'a mut [[i16; SZ]; SZ],
    spd: u16,
}

impl App<'_> {
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
        let t = &mut [[0; SZ]; SZ];
        t[SZ / 2][SZ / 2] = 1024;
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

impl PyxelCallback for App<'_> {
    fn update(&mut self, pyxel: &mut Pyxel) {
        for _ in 0..self.spd {
            for y in 0..SZ {
                for x in 0..SZ {
                    let v = self.t[y][x];
                    if v >= 4 {
                        self.t[y][x] = v - 4;
                        if x > 0 {
                            self.t[y][x - 1] += 1;
                        }
                        if x < SZ - 1 {
                            self.t[y][x + 1] += 1;
                        }
                        if y > 0 {
                            self.t[y - 1][x] += 1;
                        }
                        if y < SZ - 1 {
                            self.t[y + 1][x] += 1;
                        }
                    }
                }
            }
        }

        if pyxel.btnp(pyxel::KEY_Q, None, None) {
            pyxel.quit();
        }
        if pyxel.btnp(pyxel::KEY_SPACE, None, None) {
            self.t[SZ / 2][SZ / 2] += 128;
        }
        if pyxel.btnp(pyxel::KEY_Z, None, None) {
            for j in 0..SZ {
                for i in 0..SZ {
                    self.t[i][j] = 0;
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
        pyxel.cls(3);
        for y in 0..SZ {
            for x in 0..SZ {
                pyxel.pset(x as f64, y as f64, App::color(self.t[y][x]));
            }
        }
    }
}

fn main() {
    App::init();
}
