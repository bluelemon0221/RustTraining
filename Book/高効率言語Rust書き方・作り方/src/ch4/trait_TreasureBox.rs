
trait TreasurBox {
    fn open(&self, key_no: i32) -> bool;
    fn check(&self);
}

struct JewelyBox {
    price: i32,
    key_no: i32,
}

impl TreasurBox for JewelyBox {
    fn open(&self, key_no: i32) -> bool {
        self.key_no == key_no
    }
    fn check(&self) {
        println!("宝石箱だった！金貨{}枚入手！", self.price);
    }
}


struct TrapBox {
    damage: i32,
}
impl TreasurBox for TrapBox {
    fn open(&self, _key: i32) -> bool {
        return true;
    }
    fn check(&self) {
        println!("罠だった。{}のダメージ。", self.damage);
    }
}

fn open_box(tbox: &impl TreasurBox, key_no: i32) {
    if !tbox.open(key_no) {
        println!("鍵が合わず宝箱が開きません。");
        return;
    }
    tbox.check();
}

fn main() {
    let box1 = JewelyBox { price: 30, key_no: 1 };
    let my_key = 1;
    open_box(&box1, my_key);
}