#![feature(map_try_insert)]

use core::marker::PhantomData;
use std::borrow::Borrow;
use std::collections::HashMap;
use std::sync::{Arc, RwLock, RwLockReadGuard};
use uuid::Uuid;

trait Database {
    type Id;
    type Block: Block<Id = Self::Id>;
    type BlockRef: Borrow<Self::Block>;

    fn block(&self, id: Self::Id) -> Option<&Self::BlockRef>;
    fn add_block(&mut self) -> &Self::BlockRef;
    fn remove_block(&mut self, id: Self::Id) -> bool;
}

trait Block {
    type Id;
    type TextRef: Borrow<str>;
    type ChildRef: Borrow<Self>;
    type Children: IntoIterator<Item = Self::ChildRef>;

    fn id(&self) -> &Self::Id;
    fn spans_text(&self) -> Self::TextRef;
    fn children(&self) -> Self::Children;

    fn set_spans_text<T: Into<String>>(&self, value: T);
    fn add_child(&self, child: Self::ChildRef) -> bool;
    fn remove_child(&self, child: Self::ChildRef) -> bool;
}

/*
trait BlockRef {
    type Block;
    type ChildRef: Borrow<Self::Block>;
    type Children: Borrow<[Self::ChildRef]>;

    fn children(self) -> Self::Children;
}
*/

#[derive(Clone, Debug)]
struct MockDatabase {
    blocks: HashMap<Uuid, Arc<MockBlock>>,
}

#[derive(Debug)]
struct MockBlock {
    id: Uuid,
    spans_text: RwLock<Arc<String>>,
    children: RwLock<Arc<Vec<Arc<Self>>>>,
}

impl Database for MockDatabase {
    type Id = Uuid;
    type Block = MockBlock;
    type BlockRef = Arc<Self::Block>;

    fn block(&self, id: Self::Id) -> Option<&Self::BlockRef> {
        self.blocks.get(&id)
    }

    fn add_block(&mut self) -> &Self::BlockRef {
        use std::collections::hash_map::Entry;

        let id = Uuid::new_v4();
        let block = MockBlock {
            id,
            spans_text: RwLock::new(Arc::new(String::new())),
            children: RwLock::new(Arc::new(Vec::new())),
        };
        let block = Arc::new(block);
        self.blocks.try_insert(id, Arc::clone(&block)).unwrap()
    }

    fn remove_block(&mut self, id: Self::Id) -> bool {
        self.blocks.remove(&id).is_some()
    }
}

impl Block for MockBlock {
    type Id = Uuid;
    type TextRef = ArcString;
    type ChildRef = Arc<Self>;
    type Children = std::vec::IntoIter<Self::ChildRef>;

    fn id(&self) -> &Self::Id {
        &self.id
    }

    fn spans_text(&self) -> Self::TextRef {
        ArcString(Arc::clone(&self.spans_text.read().unwrap()))
    }

    fn children(&self) -> Self::Children {
        let children = self.children.read().unwrap();
        Vec::clone(&children).into_iter()
    }

    fn set_spans_text<T: Into<String>>(&self, value: T) {
        let mut spans_text = self.spans_text.write().unwrap();
        core::mem::replace(&mut *spans_text, Arc::new(value.into()));
    }

    fn add_child(&self, child: Self::ChildRef) -> bool {
        let mut children = self.children.write().unwrap();
        if children.iter().any(|other| Arc::ptr_eq(other, &child)) {
            false
        } else {
            Arc::make_mut(&mut children).push(child);
            true
        }
    }

    fn remove_child(&self, child: Self::ChildRef) -> bool {
        let mut children = self.children.write().unwrap();
        children
            .iter()
            .position(|other| Arc::ptr_eq(other, &child))
            .map(|j| Arc::make_mut(&mut children).remove(j))
            .is_some()
    }
}

/*
impl<'a> BlockRef for &'a MockBlock {
    type Block = MockBlock;
    type ChildRef = &'a Arc<MockBlock>;
    type Children = RefArcRwLockReadGuard<'a, MockBlock>;

    fn children(self) -> Self::Children {
        RefArcRwLockReadGuard(self.children.read().unwrap())
    }
}
*/

#[derive(Clone, Debug)]
struct ArcString(Arc<String>);

impl Borrow<str> for ArcString {
    fn borrow(&self) -> &str {
        &self.0
    }
}

/*
#[derive(Clone, Debug)]
struct RefArc<'a, T>(&'a Arc<T>);

impl<'a, T> Borrow<T> for RefArc<'a, T> {
    fn borrow(&self) -> &T {
        &self.0
    }
}*/

/*
#[derive(Debug)]
struct RefArcRwLockReadGuard<'a, T>(RwLockReadGuard<'a, Arc<Vec<Arc<T>>>>);

impl<'a, T> Borrow<[T]> for RefArcRwLockReadGuard<'a, T> {
    fn borrow(&self) -> &[T] {
        &self.0
    }
}
*/
/*
#[derive(Debug)]
struct LockedArcVec<'a, T> {
    data: RwLockReadGuard<'a, Vec<Arc<T>>>,
    offset: usize,
}

impl<'a, T: 'a> Iterator for LockedArcVec<'a, T> {
    type Item = RefArc<'a, T>;

    fn next(&mut self) -> Option<Self::Item> {
        self.data.get(self.offset).map(|value| {
            self.offset + 1;
            RefArc(value)
        })
    }
}*/

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
