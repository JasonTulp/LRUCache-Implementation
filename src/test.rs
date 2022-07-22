

#[cfg(test)]
mod tests {
    use crate::{LRUCache, Cache};

    // Setup some test structs and enums for keys
    #[derive(PartialEq, Clone)]
    enum TestEnum {
        Val1,
        Val2(TestStruct),
    }
    #[derive(PartialEq, Default, Copy,  Clone)]
    struct TestStruct {
        test_val_1: u64,
        test_val_2: char,
    }

    #[test]
    fn set_over_max_size() {
        let max_size = usize::MAX;
        let mut cache: Cache<u64, u64> = LRUCache::initialize(max_size);

        let key1: u64 = 5;
        let value1: u64 = 10;
        cache.set(key1, value1);
        assert_eq!(cache.get(key1), Some(value1));

        let key2: u64 = 7;
        let value2: u64 = 11;
        cache.set(key2, value2);

        assert_eq!(cache.get(key1), None);
        assert_eq!(cache.get(key2), Some(value2));

        let value2: u64 = 12;
        cache.set(key2, value2);
        assert_eq!(cache.get(key2), Some(value2));
    }

    #[test]
    fn set_with_different_types() {
        let max_size = 10;
        let mut cache: Cache<&str, usize> = LRUCache::initialize(max_size);
        cache.set("Im a key, hello", 1000_usize);
        assert_eq!(cache.get("Im a key, hello"), Some(1000_usize));

        // let max_size = 10;
        // let mut cache: Cache<String, usize> = LRUCache::initialize(max_size);
        // cache.set(String::from("Im a stringy key, hello"), 1000_usize);
        // assert_eq!(cache.get(String::from("Im a stringy key, hello"), Some(1000_usize));



        let mut cache: Cache<TestEnum, TestStruct> = LRUCache::initialize(max_size);
        let test_struct = TestStruct {
            test_val_1: 0,
            test_val_2: 'a'
        };
        let test_enum = TestEnum::Val2(TestStruct {
            test_val_1: u64::MAX,
            test_val_2: 'b'
        });

        cache.set(TestEnum::Val1, TestStruct::default());
        cache.set(test_enum.clone(), test_struct.clone());
        assert_eq!(cache.get(test_enum), Some(test_struct));

    }

}