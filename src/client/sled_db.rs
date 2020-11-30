use std::collections::HashMap;
use once_cell::sync::OnceCell;
use std::fs;
use std::path::Path;
use super::conf::Conf;

static SD: OnceCell<sled::Db> = OnceCell::new();

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum DB {
    Send,
    Rec,
    Task,
}

impl DB {
    pub fn init() {
        let conf = Conf::get();
        let p = &conf.db_path;
        if !Path::new(p).exists() {
            fs::create_dir_all(p).expect("create dir fail");
        }
        let db: sled::Db = sled::open(p).unwrap();
        SD.set(db).unwrap();
    }
    pub fn gen_id()->u64{

        let db = SD.get().expect("!!!!db not init");
        let a=db.generate_id().unwrap();
        a
    }

    fn tree_name(&self) -> String {
        let name = match self {
            DB::Send => "send",
            DB::Rec => "rec",
            DB::Task => "task",
        };
        name.to_string()
    }

    pub fn clear_db() {
        let db = SD.get().expect("!!!!db not init");
        let names = db.tree_names().into_iter();
        for name in names.filter(|n| n.as_ref() != b"__sled__default") {
            db.drop_tree(&name).unwrap();
        }
    }
    fn tree(&self) -> sled::Tree {
        let sd = SD.get().expect("!!!!db not init");
        let name = self.tree_name();
        sd.open_tree(&name.as_bytes().to_vec()).expect("!!!tree can not open")
    }

    pub fn insert(&self, k: &Vec<u8>, v: &Vec<u8>) -> bool {
        let v = v.to_owned();
        let res = self.tree().insert(k, v);
        if let Ok(_) = res {
            return true;
        }
        false
    }

    pub fn get_or_empty(&self, k: &Vec<u8>) -> Vec<u8> {
        let res = self.tree().get(k);
        if let Ok(res1) = res {
            if let Some(res2) = res1 {
                let mut res3 = vec![];
                for i in res2.iter() {
                    res3.push(*i);
                }
                return res3;
            }
        }
        vec![]
    }

    pub fn remove(&self, k: &Vec<u8>) {
        match self.tree().remove(k).expect("!!!db not init"){
            Some(_)=>{},
            None=>{},
        }
    }

    pub fn dic(&self) -> HashMap<Vec<u8>, Vec<u8>> {
        let mut iter = self.tree().iter();
        let mut m = HashMap::new();
        loop {
            if let Some(item) = iter.next() {
                match item {
                    Ok(i) => {
                        let k = (*i.0).to_vec();
                        let v = (*i.1).to_vec();
                        m.insert(k, v);
                    }
                    Err(e) => {
                        dbg!(e);
                        break;
                    }
                }
            } else {
                break;
            }
        }
        m
    }
    pub fn pop(&self) -> (Vec<u8>, Vec<u8>) {
        let mut iter = self.tree().iter();
        if let Some(res1) = iter.next() {
            if let Ok(res2) = res1 {
                let (k, v) = res2;
                let key = (*k).to_vec();
                self.remove(&key);
                return (key, (*v).to_vec());
            }
        }
        (vec![], vec![])
    }

    pub fn clear_tree(&self) -> bool {
        let sd = SD.get().unwrap();
        let name = self.tree_name();
        sd.drop_tree(&name.as_bytes().to_vec()).unwrap()
    }
}

#[test]
fn test_db() {
    DB::init();
    DB::clear_db();

    let k = [1, 2].to_vec();
    let x = [4, 6].to_vec();
    let kk = [3, 7].to_vec();
    DB::Send.insert(&k, &x);
    DB::Send.insert(&k, &kk);
    let res = DB::Send.dic();
    dbg!(res);
    let n=DB::gen_id();
    dbg!(n);

    let n=DB::gen_id();
    dbg!(n);
    DB::gen_id();

    let n=DB::gen_id();
    dbg!(n);
    // assert_eq!(x,res);
}
