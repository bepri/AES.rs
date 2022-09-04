use std::fmt;



pub struct State(pub [u8; 16]);

impl From<u128> for State {
    fn from(mut input: u128) -> Self {
        let mut state: [u8; 16] = [0; 16];
        for i in (0..=15).rev() {
            state[i] = (input & 0xff) as u8;
            input >>= 2;
        }

        Self(state)
    }
}

impl Default for State {
    fn default() -> Self {
        Self::from(0)
    }
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for i in 0..4 {
            for j in 0..4 {
                write!(f, "{:02X} ", self.get(i, j));
            }
            write!(f, "\n");
        }

        Ok(())
    }
}

impl State {
    pub fn get(&self, i: usize, j: usize) -> u8 {
        self.0[i + (j * 4)]
    }
}