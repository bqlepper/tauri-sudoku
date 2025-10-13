// A cell holds the potential values for that cell in a 16 bit number.
// Only the lower 9 bits are used since each bit represents a valid value.
pub(super) struct Cell {
    pv: u16,
    set_by_user: bool,
}

impl Cell {
    pub(super) fn new() -> Cell {
        Cell {
            pv : 0x1ff,
            set_by_user : false,
        }
    }

    pub(super) fn get_values(&self) -> u16 { self.pv }

    pub(super) fn is_solved(&self) -> bool { self.pv.count_ones() == 1 }

    pub(super) fn is_set_by_user(&self) -> bool { self.set_by_user }

    pub(super) fn potential_value_count(&self) -> usize { self.pv.count_ones().try_into().unwrap() }

    pub(super) fn lock_set_by_user(&mut self) { self.set_by_user = true; }

    pub(super) fn is_value_valid(&self, value: u8) -> bool {
        let changed_value = self.pv & 1 << (value - 1);
        changed_value != 0
    }

    // returns the remaining potential values in a vector.  If already solved return empty vector.
    pub(super) fn get_value_list(&self) -> Vec<u8> {
        let mut values = Vec::new();
        if !self.is_solved() {
            let mut temp_pv = self.pv;
            let mut value: u8 = 1;
            while temp_pv > 0 {
                if temp_pv & 1 > 0 {
                    values.push(value);
                }
                value += 1;
                temp_pv >>= 1;
            }
        }
        values
    }

    // Return true if the passed in cell has only potential values that are also potential values in this cell
    pub(super) fn is_partner(&self, partner_value: u16) -> bool {
        partner_value | self.pv == partner_value
    }

    pub(super) fn clear(&mut self) {
        self.pv = 0x1ff;
        self.set_by_user = false;
    }

    pub(super) fn set_value(&mut self, value: u8) -> Result<bool, ()> {
        let changed_value = self.pv & 1 << (value - 1);
        if changed_value == 0 { return Err(()); } // Value not valid
        if changed_value == self.pv { return Ok(false); } // Value already set
        self.pv = changed_value;
        Ok(true)
    }

    pub(super) fn remove_value(&mut self, value: u8) -> Result<bool, ()> {
        if value == 0 { return Ok(false); } // Can't remove nothing, should never happen
        let changed_value = self.pv & !(1 << (value - 1));
        if changed_value == 0 { return Err(()); } // Cannot remove all values
        if changed_value == self.pv { return Ok(false); } // Value already removed
        self.pv = changed_value;
        Ok(true)
    }

    pub(super) fn remove_values(&mut self, value: u16) -> Result<bool, ()> {
        let changed_value = self.pv & !(value);
        if changed_value == 0 { return Err(()); } // Cannot remove all values
        if changed_value == self.pv { return Ok(false); } // Values already removed
        self.pv = changed_value;
        Ok(true)
    }

    pub(super) fn get_value(&self) -> Result<u8, ()> {
        if self.pv == 0 { return Err(()); } // Cell has no valid potential values
        if !self.is_solved() { return Ok(0); } // Cell is not yet solved, return 0
        Ok((self.pv.trailing_zeros() + 1).try_into().unwrap())
    }
}
