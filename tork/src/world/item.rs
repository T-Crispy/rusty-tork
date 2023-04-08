
#[derive(PartialEq)]
pub enum ItemType{
    KEY = 0,
    LIT = 1,
    WPN = 2,
}

pub struct Item {
    pub item_type: ItemType,
    pub name: String,
    pub id: usize,
    pub val1: usize,
    pub val2: usize,
    pub loc: isize,
}

impl Clone for Item{
    fn clone(&self) -> Item {
        let mut item_copy: Item = Item { 
            name: self.name.clone(),
            id: self.id,
            item_type: ItemType::KEY,
            val1: self.val1, 
            val2: self.val2,
            loc: self.loc,
        };

        if self.item_type == ItemType::LIT{
            item_copy.item_type = ItemType::LIT;
        }
        else if self.item_type == ItemType::WPN{
            item_copy.item_type = ItemType::WPN;
        }

        item_copy
    }
}
