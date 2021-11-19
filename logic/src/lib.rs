use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use uuid::Uuid;

trait DatabaseRef<'a> {
    type Id;
    type BlockRef: BlockRef<'a, Id = Self::Id>;

    fn block(self, id: Self::Id) -> Option<Self::BlockRef>;
    fn add_block(self) -> Self::BlockRef;
    fn remove_block(self, id: Self::Id) -> bool;
}

trait BlockRef<'a> {
    type Id;
    type Children: 'a + Iterator<Item = Self>;

    fn id(self) -> Self::Id;
    fn spans_text(self) -> &'a str;
    fn children(self) -> Self::Children;

    fn set_spans_text<T: ToOwned<Owned = String>>(self, value: T);
    fn add_child(self, child: Self) -> bool;
    fn remove_child(self, child: Self) -> bool;
}

#[derive(Clone, Debug)]
struct Database {
    blocks: HashMap<Uuid, Arc<Block>>,
}

#[derive(Debug)]
struct Block {
    id: Uuid,
    spans_text: RwLock<String>,
    children: RwLock<Vec<Arc<Block>>>,
}

impl<'a> DatabaseRef<'a> for &'a mut Database {
    type Id = Uuid;
    type BlockRef = &'a Arc<Block>;

    fn block(self, id: Self::Id) -> Option<Self::BlockRef> {
        self.blocks.get(&id)
    }

    fn add_block(self) -> Self::BlockRef {
        use std::collections::hash_map::Entry;

        let id = Uuid::new_v4();
        let block = Block {
            id,
            spans_text: RwLock::new(String::new()),
            children: RwLock::new(Vec::new()),
        };
        let block = Arc::new(block);
        let prev = self.blocks.insert(id, Arc::clone(&block));
        assert!(prev.is_none());
        //&block
        todo!()
    }

    fn remove_block(self, id: Self::Id) -> bool {
        self.blocks.remove(&id).is_some()
    }
}

impl<'a> BlockRef<'a> for &'a Arc<Block> {
    type Id = Uuid;
    type Children = std::slice::Iter<'a, Arc<Block>>;

    fn id(self) -> Self::Id {
        self.id
    }

    fn spans_text(self) -> &'a str {
        //self.spans_text.read().unwrap()
        todo!()
    }

    fn children(self) -> Self::Children {
        //self.children.read().unwrap().iter()
        todo!()
    }

    fn set_spans_text<T: ToOwned<Owned = String>>(self, value: T) {
        todo!()
    }

    fn add_child(self, child: Self) -> bool {
        todo!()
    }

    fn remove_child(self, child: Self) -> bool {
        todo!()
    }
}

#[test]
fn test() {
    /*
    let mut block01 = Arc::new(Block {
        id: "01",
        spans_text: "test".to_owned(),
        children: vec![],
    });
    let mut block02 = Arc::new(Block {
        id: "02",
        spans_text: "foo".to_owned(),
        children: vec![],
    });
    let mut block03 = Arc::new(Block {
        id: "03",
        spans_text: "bar".to_owned(),
        children: vec![],
    });

    Arc::get_mut(&mut block02)
        .unwrap()
        .children
        .push(Arc::clone(&block03));
    Arc::get_mut(&mut block01)
        .unwrap()
        .children
        .push(Arc::clone(&block02));
    Arc::get_mut(&mut block01)
        .unwrap()
        .children
        .push(Arc::clone(&block03));
    let db = Database {
        blocks: [block01, block02, block03]
            .into_iter()
            .map(|block| (block.id, block))
            .collect(),
    };

    dbg!(db.block("01").unwrap().spans_text());
    dbg!(db
        .block("01")
        .unwrap()
        .children()
        .map(|block| block.id())
        .collect::<Vec<_>>());
    panic!();
    */
}
