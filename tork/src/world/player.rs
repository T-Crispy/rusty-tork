use crate::world::room::Room;
//use crate::world::Item;

pub struct Player {
     pub loc: *const Room, //location of the character
     pub inv: [isize; 7], //ids of items on floor (-1 for empty slot)
     pub inv_lim: usize,
     pub hit_points: u16,
}

impl  Player {
     pub fn get_inv_ind(&self, item_id: usize) -> (usize, bool) {
          for (i, item) in self.inv.iter().enumerate() {
               if item.eq(&(item_id as isize)) {
                    return (i, true);
               }
          }
          (0, false)
     }

     pub fn count_inv(&self) -> usize {
          let mut count = 0;

          for item in self.inv.iter() {
               if !item.eq(&-1) {
                    count += 1;
               }
          }
          return count;
     }

     pub fn pocket_item(&mut self, item_id: usize) -> bool {
          for(i, item) in self.inv.iter().enumerate() {
               //check for empty slot
               if item.eq(&-1) {
                    self.inv[i] = item_id as isize;
                    return true;
               }
          }
          return false;
     }

     pub fn drop_item(&mut self, item_id: usize) -> bool {
          for (i, item) in self.inv.iter().enumerate() {
               //compare item IDs
               if item.eq(&(item_id as isize)) {
                    self.inv[i] = -1;
                    break;
               }
          }
          false
     }
}