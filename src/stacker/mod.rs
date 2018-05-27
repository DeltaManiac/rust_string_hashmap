mod expull;
pub(crate) mod hashmap;
mod heap;

pub use self::expull::ExpUnrolledLinkedList;
pub use self::hashmap::TermHashMap;
pub use self::hashmap::split_memory;
pub use self::heap::{Heap, HeapAllocable};

#[test]
fn test_unrolled_linked_list() {
    use std::collections;
    let heap = Heap::with_capacity(30_000_000);
    {
        heap.clear();
        let mut ks: Vec<usize> = (1..5).map(|k| k * 100).collect();
        ks.push(2);
        ks.push(3);
        for k in (1..5).map(|k| k * 100) {
            let mut hashmap: TermHashMap = TermHashMap::new(10, &heap);
            for j in 0..k {
                for i in 0..500 {
                    let v: &mut ExpUnrolledLinkedList = hashmap.get_or_create(i.to_string()).1;
                    v.push(i * j, &heap);
                }
            }
            let mut map_addr: collections::HashMap<Vec<u8>, u32> = collections::HashMap::new();
            for (key, addr, _) in hashmap.iter() {
                map_addr.insert(Vec::from(key), addr);
            }

            for i in 0..500 {
                let key: String = i.to_string();
                let addr: u32 = *map_addr.get(key.as_bytes()).unwrap();
                let exp_pull: &ExpUnrolledLinkedList = heap.get_ref(addr);
                let mut it = exp_pull.iter(addr, &heap);
                for j in 0..k {
                    assert_eq!(it.next().unwrap(), i * j);
                }
                assert!(!it.next().is_some());
            }
        }
    }
}


#[test]
fn test_map() {
    use std::collections;
    let heap = Heap::with_capacity(30_000_000);
    let mut hashmap: TermHashMap = TermHashMap::new(10, &heap);
    {
        let el = hashmap.get_or_create::<_, u32>("nice");
        println!("WAAAAA");
        println!("TERMID {:?}", el.0);
        println!("WAS DAS {:?}", el.1);
        *el.1 = 10;

    }
    let el = hashmap.get_or_create::<_, u32>("heiß");

}
