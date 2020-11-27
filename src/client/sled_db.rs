use std::collections::{HashMap, HashSet};
use once_cell::sync::OnceCell;
use std::fs;
use std::path::Path;

static SD: OnceCell<sled::Db> = OnceCell::new();

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum DB {
    Send,
    Rec,
    Task,
}

impl DB {
    pub fn init() {
        let p = "./db";
        if !Path::new(p).exists() {
            fs::create_dir_all(p).expect("create dir fail");
        }
        let db: sled::Db = sled::open(p).unwrap();
        SD.set(db).unwrap();
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
        DB::Send.clear_tree();
        DB::Rec.clear_tree();
        DB::Task.clear_tree();
    }
    fn tree(&self) -> sled::Tree {
        let sd = SD.get().unwrap();
        let name = self.tree_name();
        sd.open_tree(&name.as_bytes().to_vec()).unwrap()
    }

    pub fn insert(&self, k: &Vec<u8>, v: &Vec<u8>) -> bool {
        let res=self.tree().inset(k, v);
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
        self.tree().remove(k);
    }

    pub fn dic(&self) -> HashMap<Vec<u8>, Vec<u8>> {
        let mut iter = self.tree().iter();
        let mut m = HashMap::new();
        for (k, v) in iter {
            m[*k] = *v;
        }
        m
    }
    pub fn next(&self) -> (Vec<u8>, Vec<u8>) {
        let mut iter = self.tree().iter();
        let (k, v) = iter.next().unwrap();
        (*k, *v)
    }

    pub fn clear_tree(&self) -> bool {
        let sd = SD.get().unwrap();
        let name = self.tree_name();
        sd.drop_tree(&name.as_bytes().to_vec()).unwrap()
    }
}

