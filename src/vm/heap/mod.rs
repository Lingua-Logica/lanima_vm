use std::rc::Rc;

use crate::vm::heap::heap_object::HeapVal;

pub mod heap_object;

#[derive(Clone, Debug)]
pub struct Heap {
    heap: Vec<HeapVal>,

    /*
    已被释放的对象在原列表的索引集合 {
        我不知道 lamina 那帮人咋想的，居然用 hashmap。
        明明只有被释放的才要在这个列表里,
        可能是因为线性查找比较慢?
    }
    */
    free: Vec<usize>,
}

impl Heap {
    pub fn new() -> Self {
        Self {
            heap: vec![],
            free: vec![]
        }
    }

    #[inline(always)]
    pub fn contains(&self, i: usize) -> Result<(), Rc<str>> {
        if self.released(i) {
            return Err(format!("use after free. index {i}").into());
        }

        if !self.in_heap(i) {
            return Err(format!(
                "invalid index {i}. heap has {} {}",
                self.heap.len(),
                if self.heap.len() != 1 {
                    "values"
                } else {
                    "value"
                }
            )
            .into());
        }

        Ok(())
    }

    #[inline(always)]
    pub fn in_heap(&self, i: usize) -> bool {
        self.heap.len() > i
    }

    #[inline(always)]
    pub fn released(&self, i: usize) -> bool {
        self.free.contains(&i)
    }

    #[inline]
    pub fn get(&self, i: usize) -> Result<HeapVal, Rc<str>> {
        self.contains(i)?;
        Ok(self.heap[i].clone())
    }

    #[inline]
    pub fn get_ref(&self, i: usize) -> Result<&HeapVal, Rc<str>> {
        self.contains(i)?;
        Ok(&self.heap[i])
    }

    #[inline(always)]
    pub fn push(&mut self, obj: HeapVal) -> usize {
        let i = self.heap.len();
        self.heap.push(obj);

        i
    }

    #[inline]
    pub fn free(&mut self, i: usize) -> Result<HeapVal, Rc<str>> {
        self.contains(i)?;
        self.free.push(i);
        Ok(self.heap.remove(i))
    }

    #[inline]
    pub fn update(&mut self, i: usize, new: HeapVal) -> Result<(), Rc<str>> {
        self.contains(i)?;
        self.heap[i] = new;

        Ok(())
    }
}
