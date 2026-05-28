// A cell holds the potential values for that cell in a 16 bit number.
// Only the lower 9 bits are used since each bit represents a valid value.
#[derive(Clone, Copy)]
pub(super) struct Cell {
    pv: u16,
    set_by_user: bool,
}

const ALL_VALUES_MASK: u16 = 0x1ff;

impl Cell {
    pub(super) fn new() -> Cell {
        Cell {
            pv : ALL_VALUES_MASK,
            set_by_user : false,
        }
    }

    pub(super) fn is_solved(&self) -> bool { self.pv.count_ones() == 1 }

    pub(super) fn is_valid(&self) -> bool { self.pv.count_ones() >= 1 }

    pub(super) fn is_set_by_user(&self) -> bool { self.set_by_user }

    pub(super) fn lock_set_by_user(&mut self) { self.set_by_user = true; }

    pub(super) fn is_value_valid(&self, value: u8) -> bool {
        let changed_value = self.pv & 1 << (value - 1);
        changed_value != 0
    }

    pub(super) fn candidate_mask(&self) -> u16 { self.pv }

    pub(super) fn clear(&mut self) {
        self.pv = ALL_VALUES_MASK;
        self.set_by_user = false;
    }

    pub(super) fn set_values(&mut self, values: u16) -> Result<bool, ()> {
        if values == 0 || values & !ALL_VALUES_MASK != 0 {
            return Err(());
        }
        if self.pv == values {
            return Ok(false);
        }
        self.pv = values;
        Ok(true)
    }

    pub(super) fn set_value(&mut self, value: u8) -> Result<bool, ()> {
        let changed_value = self.pv & 1 << (value - 1);
        if changed_value == 0 { return Err(()); } // Value not valid
        if changed_value == self.pv { return Ok(false); } // Value already set
        self.pv = changed_value;
        Ok(true)
    }

    pub(super) fn get_value(&self) -> Result<u8, ()> {
        if self.pv == 0 { return Err(()); } // Cell has no valid potential values
        if !self.is_solved() { return Ok(0); } // Cell is not yet solved, return 0
        Ok((self.pv.trailing_zeros() + 1) as u8)
    }
}
