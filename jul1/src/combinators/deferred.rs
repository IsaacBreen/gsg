// src/combinators/deferred.rs
use std::any::{Any, TypeId};
use std::cell::RefCell;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt::{Debug, Formatter};
use std::hash::{Hash, Hasher};
use std::panic::Location;
use std::rc::Rc;
use crate::*;
use once_cell::unsync::OnceCell;

thread_local! {
    static DEFERRED_CACHE: RefCell<HashMap<CacheKey, CacheEntry>> = RefCell::new(HashMap::new());
}

#[derive(Debug)]
pub struct Deferred<T: CombinatorTrait + 'static> {
    deferred_fn: Box<dyn DeferredFnTrait<T>>,
    inner: OnceCell<T>,
}

struct DeferredFn<T: CombinatorTrait + 'static, F: Fn() -> T> {
    pub f: F,
    pub key: CacheKey,
}

impl<T: CombinatorTrait + Clone + 'static, F: Fn() -> T> Debug for DeferredFn<T, F> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("DeferredFn").field(&self.key).finish()
    }
}

// todo: this trait is really messy. Any way to clean it up?
// Trait for evaluating the deferred function
trait DeferredFnTrait<T: CombinatorTrait + 'static>: Debug {
    // todo: the fact that we need this struct right now is dumb. Aim to remove it. But we don't want to have to return a (bool, T) tuple - that's even worse and it's what we're trying to avoid by using the struct.
    fn evaluate_to_combinator(&self) -> EvaluateToCombinatorResult<T>;
    fn get_addr(&self) -> usize;
}

struct EvaluateToCombinatorResult<T> {
    combinator: T,
    cache_hit: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct CacheKey {
    addr: usize,
    // TODO: it really seems wrong to use TypeId here. It works fine with just addr when we use opt level 0.
    //  But at opt level 2, we get a collision between two keys with the same address but different typeids.
    //  Obviously something weird is going on with function pointers that I don't quite understand.
    type_id: TypeId,
}

// CacheEntry struct to hold the CombinatorTrait and caller locations
#[derive(Debug)]
struct CacheEntry {
    value: Box<dyn CombinatorTrait>,
    caller_locations: RefCell<HashSet<String>>,
}

impl<T: CombinatorTrait + Clone + 'static, F: Fn() -> T> DeferredFnTrait<T> for DeferredFn<T, F> {
    fn evaluate_to_combinator(&self) -> EvaluateToCombinatorResult<T> {
        DEFERRED_CACHE.with(|cache| {
            if cache.borrow().contains_key(&self.key) {
                count_hit!("deferred cache hit");
                let borrowed = cache.borrow();
                let entry = borrowed.get(&self.key).unwrap();
                if let Some(value) = entry.value.as_any().downcast_ref::<T>() {
                    return EvaluateToCombinatorResult {
                        combinator: value.clone(),
                        cache_hit: true,
                    };
                } else {
                    // Richer error printing
                    eprintln!("Deferred Cache: {:#?}", borrowed);
                    eprintln!("Key: {:?}", self.key);
                    eprintln!("Conflicting Entry: {:?}", entry);
                    eprintln!("Existing Type Name: {:?}", entry.value.type_name());
                    eprintln!("Expected Type Name: {}", std::any::type_name::<T>());
                    panic!("Expected value at address {} to be of typeid {:?}, but it had typeid {:?}", self.key.addr, TypeId::of::<T>(), entry.value.as_any().type_id());
                }
            } else {
                count_hit!("deferred cache miss");
                let value = (self.f)();
                cache.borrow_mut().insert(self.key, CacheEntry {
                    value: Box::new(value.clone()),
                    caller_locations: RefCell::new(HashSet::new()),
                });
                return EvaluateToCombinatorResult {
                    combinator: value,
                    cache_hit: false,
                };
            }
        })
    }
    fn get_addr(&self) -> usize {
        self.key.addr
    }
}

impl<T: CombinatorTrait + 'static> PartialEq for Deferred<T> {
    fn eq(&self, other: &Self) -> bool {
        std::ptr::eq(&self.deferred_fn, &other.deferred_fn)
    }
}

impl<T: CombinatorTrait + 'static> Eq for Deferred<T> {}

impl<T: CombinatorTrait + 'static> Hash for Deferred<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        std::ptr::hash(&self.deferred_fn, state);
    }
}

impl<T: CombinatorTrait + 'static> CombinatorTrait for Deferred<T> {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn apply(&self, f: &mut dyn FnMut(&dyn CombinatorTrait)) {
        f(self.inner.get_or_init(|| self.deferred_fn.evaluate_to_combinator().combinator))
    }

    fn one_shot_parse(&self, right_data: RightData, bytes: &[u8]) -> UnambiguousParseResults {
        todo!()
    }

    fn parse(&self, right_data: RightData, bytes: &[u8]) -> (Parser, ParseResults) {
        // let combinator = self.inner.get_or_init(|| self.deferred_fn.evaluate_to_combinator());
        let combinator = self.inner.get().expect("inner combinator not initialized");
        combinator.parse(right_data, bytes)
    }

    fn compile_inner(&self) {
        // Force evaluation and compilation of the inner combinator
        let _ = self.inner.get_or_init(|| {
            let result = self.deferred_fn.evaluate_to_combinator();
            if !result.cache_hit {
                result.combinator.compile_inner();
            }
            result.combinator
        });
    }
}

// Public function for creating a Deferred combinator
#[track_caller]
pub fn deferred<T: CombinatorTrait + 'static>(f: fn() -> T) -> Deferred<StrongRef<T>> {
    let addr = f as *const () as usize;
    let location = Location::caller();
    let caller_location_str = location.to_string();
    let key = CacheKey {
        addr,
        type_id: TypeId::of::<T>(),
    };
    DEFERRED_CACHE.with(|cache| {
        let mut cache = cache.borrow_mut();
        if let Some(entry) = cache.get_mut(&key) {
            entry.caller_locations.borrow_mut().insert(caller_location_str);
        }
    });
    let f = move || StrongRef::new(f());
    Deferred {
        deferred_fn: Box::new(DeferredFn {
            f,
            key
        }),
        inner: OnceCell::new(),
    }
}