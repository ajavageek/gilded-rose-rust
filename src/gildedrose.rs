use std::fmt::{self, Display};
pub struct Item {
    pub name: String,
    pub sell_in: i32,
    pub quality: i32,
}

impl Item {
    pub fn new(name: impl Into<String>, sell_in: i32, quality: i32) -> Item {
        Item {
            name: name.into(),
            sell_in,
            quality,
        }
    }
    pub fn increase_quality(&mut self) {
        self.quality += 1;
    }
    pub fn decrease_quality(&mut self) {
        self.quality -= 1;
    }
    pub fn reset_quality(&mut self) {
        self.quality = 0;
    }
    pub fn pre_sell_in(&mut self) {
        if self.name == "Aged Brie" {
            if self.quality < 50 {
                self.increase_quality();
            }
        } else if self.name == "Backstage passes to a TAFKAL80ETC concert" {
            if self.quality < 50 {
                self.increase_quality();

                if self.quality < 50 && self.sell_in < 11 {
                    self.increase_quality();
                }
                if self.quality < 50 && self.sell_in < 6 {
                    self.increase_quality();
                }
            }
        } else if self.name == "Sulfuras, Hand of Ragnaros" {
        } else {
            if self.quality > 0 {
                self.decrease_quality();
            }
        }
    }
}

impl Display for Item {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}, {}, {}", self.name, self.sell_in, self.quality)
    }
}

pub struct GildedRose {
    pub items: Vec<Item>,
}

impl GildedRose {
    pub fn new(items: Vec<Item>) -> GildedRose {
        GildedRose { items }
    }

    pub fn update_quality(&mut self) {
        for item in &mut self.items {
            item.pre_sell_in();
            item.sell_in();
            if item.name != "Sulfuras, Hand of Ragnaros" {
                item.sell_in -= 1;
            }

            if item.sell_in < 0 {
                if item.name == "Aged Brie" {
                    if item.quality < 50 {
                        item.increase_quality();
                    }
                } else if item.name == "Backstage passes to a TAFKAL80ETC concert" {
                    item.reset_quality();
                } else if item.name == "Sulfuras, Hand of Ragnaros" {

                } else {
                    if item.quality > 0 {
                        item.decrease_quality();
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{GildedRose, Item};

    #[test]
    pub fn when_updating_regular_item_sell_in_and_quality_should_decrease() {
        let items = vec![Item::new("foo", 10, 10)];
        let mut rose = GildedRose::new(items);
        rose.update_quality();
        assert_eq!(9, rose.items[0].sell_in);
        assert_eq!(9, rose.items[0].quality);
    }

    #[test]
    pub fn when_updating_regular_item_quality_should_stop_decreasing_at_0() {
        let items = vec![Item::new("bar", 10, 10)];
        let mut rose = GildedRose::new(items);
        for _ in 0..20 {
            rose.update_quality();
        }
        assert_eq!(0, rose.items[0].quality);
    }

    #[test]
    pub fn when_updating_hand_of_ragnaros_quality_should_be_constant() {
        let items = vec![Item::new("Sulfuras, Hand of Ragnaros", 10, 10)];
        let mut rose = GildedRose::new(items);
        for _ in 0..20 {
            rose.update_quality();
        }
        assert_eq!(10, rose.items[0].quality);
    }

    #[test]
    pub fn when_updating_aged_brie_quality_should_increase() {
        let items = vec![Item::new("Aged Brie", 10, 10)];
        let mut rose = GildedRose::new(items);
        for _ in 0..20 {
            rose.update_quality();
        }
        assert_eq!(40, rose.items[0].quality);
    }

    #[test]
    pub fn when_updating_backstage_pass_quality_should_increase() {
        let items = vec![Item::new(
            "Backstage passes to a TAFKAL80ETC concert",
            50,
            20,
        )];
        let mut rose = GildedRose::new(items);
        for _ in 0..20 {
            rose.update_quality();
        }
        assert_eq!(40, rose.items[0].quality);
    }

    #[test]
    pub fn when_updating_backstage_pass_quality_should_double_before_sell_in_ten() {
        let items = vec![Item::new(
            "Backstage passes to a TAFKAL80ETC concert",
            20,
            20,
        )];
        let mut rose = GildedRose::new(items);
        for _ in 0..20 {
            rose.update_quality();
        }
        assert_eq!(50, rose.items[0].quality);
    }

    #[test]
    pub fn when_updating_backstage_pass_quality_should_be_zero_when_sell_in_negative() {
        let items = vec![Item::new(
            "Backstage passes to a TAFKAL80ETC concert",
            19,
            20,
        )];
        let mut rose = GildedRose::new(items);
        for _ in 0..20 {
            rose.update_quality();
        }
        assert_eq!(0, rose.items[0].quality);
    }
}
