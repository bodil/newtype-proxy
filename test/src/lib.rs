#[macro_use] extern crate newtype_proxy;

#[derive(NewtypeProxy)]
struct MyVec<A>(Vec<A>);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let some_vec: Vec<i64> = vec![1, 2, 3];
        let mut my_vec = MyVec::from(some_vec.clone());
        assert_eq!(3, my_vec.len());
        assert_eq!(some_vec, *my_vec);
        my_vec.push(4);
        assert_eq!(4, my_vec.len());
    }
}
