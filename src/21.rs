use priority_queue::PriorityQueue;
use std::cmp;

// notes:
// - Each attack does at least 1 damage.
// - At or below 0 ends the game.
// - Damage is: max(damage - target.armor, 1)
// - Cannot buy duplicate rings

struct Character {
    original_hit_points: i16,
    hit_points: i16,
    damage: i16, 
    armor: i16,
    is_dead: bool
}

impl Character {
    fn receive_attack(&mut self, attacker: &Character) {
        self.hit_points -= cmp::max(1, attacker.damage - self.armor);
        self.is_dead = self.hit_points <= 0;
    }

    fn reset(&mut self) {
        self.is_dead = false;
        self.hit_points = self.original_hit_points;
    }

    fn set_armor(&mut self, armor: i16) {
        self.armor = armor;
    }

    fn set_damage(&mut self, damage: i16) {
        self.damage = damage;
    }
}

#[derive(Hash)]
struct Item {
    cost: i16,
    damage: i16,
    armor: i16
}

impl PartialEq for Item {
    fn eq(&self, other: &Item) -> bool {
        self.cost == other.cost && 
        self.damage == other.damage &&
        self.armor == other.armor
    }
}
impl Eq for Item {}

fn run_battle(player: &mut Character, boss: &mut Character){
    let mut players_turn = true;
    while !player.is_dead && !boss.is_dead {
        if players_turn {
            // print!("Boss Health: ");
            boss.receive_attack(player);
        } else {
            // print!("Player Health: ");
            player.receive_attack(boss);
        }

        players_turn = !players_turn;
    }
}

fn test(ph: i16, pd: i16, pa: i16, bh: i16, bd: i16, ba: i16) {
    let mut player = Character {
        original_hit_points: ph,
        hit_points: ph,
        damage: 0,
        armor: 0,
        is_dead: false
    };

    let mut boss = Character {
        original_hit_points: bh,
        hit_points: bh,
        damage: bd,
        armor: ba,
        is_dead: false
    };

    player.set_damage(pd);
    player.set_armor(pa);
    run_battle(&mut player, &mut boss);

    assert_eq!(player.is_dead, false);
    assert_eq!(boss.is_dead, true);

    player.reset();
    boss.reset();
    run_battle(&mut player, &mut boss);

    assert_eq!(player.is_dead, false);
    assert_eq!(boss.is_dead, true);
}

fn main() {
    test(8, 5, 5, 12, 7, 2);
    test(100, 4, 6, 104, 8, 1);
    test(100, 13, 0, 104, 8, 1);
    test(100, 10, 0, 104, 8, 1);

    let mut player = Character {
        original_hit_points: 100,
        hit_points: 100,
        damage: 0,
        armor: 0,
        is_dead: false
    };

    let mut boss = Character {
        original_hit_points: 104,
        hit_points: 104,
        damage: 8,
        armor: 1,
        is_dead: false
    };

    let weapons = vec![
        Item { cost: 8,  damage: 4, armor: 0 },
        Item { cost: 10, damage: 5, armor: 0 },
        Item { cost: 25, damage: 6, armor: 0 },
        Item { cost: 40, damage: 7, armor: 0 },
        Item { cost: 74, damage: 8, armor: 0 },
    ];

    let armor = vec![
        Item { cost: 0,   damage: 0, armor: 0 },
        Item { cost: 13,  damage: 0, armor: 1 },
        Item { cost: 31,  damage: 0, armor: 2 },
        Item { cost: 53,  damage: 0, armor: 3 },
        Item { cost: 75,  damage: 0, armor: 4 },
        Item { cost: 102, damage: 0, armor: 5 },
    ];

    let rings = vec![
        Item { cost: 0,   damage: 0, armor: 0 },
        Item { cost: 0,   damage: 0, armor: 0 },
        Item { cost: 25,  damage: 1, armor: 0 },
        Item { cost: 50,  damage: 2, armor: 0 },
        Item { cost: 100, damage: 3, armor: 0 },
        Item { cost: 20,  damage: 0, armor: 1 },
        Item { cost: 40,  damage: 0, armor: 2 },
        Item { cost: 80,  damage: 0, armor: 3 },
    ];

    let mut queue: PriorityQueue<Item, i16> = PriorityQueue::new();
    for w in weapons.iter() {
        for a in armor.iter() {
            for ring_1_index in 0..rings.len() {
                let ring_1 = rings.get(ring_1_index).unwrap();

                queue.push(
                    Item { 
                        cost: w.cost + a.cost + ring_1.cost, 
                        damage: w.damage + ring_1.damage,
                        armor: a.armor + ring_1.armor}, 
                    w.cost + a.cost + ring_1.cost);

                for ring_2_index in 0..rings.len() {
                    if ring_1_index != ring_2_index {
                        let ring_2 = rings.get(ring_2_index).unwrap();

                        queue.push(
                            Item { 
                                cost: w.cost + a.cost + ring_1.cost + ring_2.cost, 
                                damage: w.damage + ring_1.damage + ring_2.damage,
                                armor: a.armor + ring_1.armor + ring_2.armor}, 
                            w.cost + a.cost + ring_1.cost + ring_2.cost);
                    }
                }
            }
        }
    }

    let mut lowest_success: i16 = 10000;
    let mut largest_failure: i16 = -1;

    for item in queue.into_sorted_vec().iter() {
        player.set_damage(item.damage);
        player.set_armor(item.armor);
        run_battle(&mut player, &mut boss);
        
        if boss.is_dead {
            lowest_success = item.cost;
        } else if largest_failure == -1 {
            largest_failure = item.cost;
        }

        player.reset();
        boss.reset();
    }

    println!("lowest success: {}", lowest_success);
    println!("largest failure: {}", largest_failure);
}