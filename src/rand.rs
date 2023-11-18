use lazy_static::lazy_static;
use spin::Mutex;

static mut SEED: u32 = 3816547290;

pub struct LinearShiftFeedbackRegister {
    buffer: &'static mut u32,
}

lazy_static! {
    pub static ref PRNG: Mutex<LinearShiftFeedbackRegister> = Mutex::new(LinearShiftFeedbackRegister {
        buffer: unsafe { &mut SEED } ,
    });
}

impl LinearShiftFeedbackRegister {
    pub fn gen_range(&mut self, min: u32, max: u32) -> u32 {
        if min >= max {
            panic!("Random range min larger than max")
        }
        self.lsfr();
        return (*self.buffer % (max - min)) + min;
    }

    fn lsfr(&mut self) {
        let bit = ((*self.buffer >> 7)
                    ^ (*self.buffer >> 9)
                    ^ (*self.buffer >> 13)) 
                    & 1;
        *self.buffer = (*self.buffer >> 1) | (bit << 31);
    }
}
