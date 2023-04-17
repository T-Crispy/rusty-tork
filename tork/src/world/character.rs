use crate::world::room::Room;
//use crate::world::Item;

pub struct Character {
     pub loc: *const Room, //location of the character
     pub inv: [isize; 7], //ids of items on floor (-1 for empty slot)
     pub inv_lim: usize,
}

impl  Character {
     pub fn get_inv_ind(&self, item_id: usize) -> (usize, bool) {
          for (i, item) in self.inv.iter().enumerate() {
               if item.eq(&(item_id as isize)) {
                    return (i, true);
               }
          }

          (0, false)
     }
}