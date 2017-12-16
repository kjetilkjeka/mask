pub trait Mask<T>
    where T: PartialEq {
    fn mask(&self, value: &T) -> T;
}

impl<T: PartialEq, M: Mask<T>> Filter<T> for M {
    fn is_match(&self, value: &T) -> bool {
        self.mask(value) == *value
    }
}

pub trait Filter<T> {
    fn is_match(&self, value: &T) -> bool;
}




impl Mask<u8> for u8 {
    fn mask(&self, value: &Self) -> Self {
        value & self
    }
}




#[cfg = "tests"]
mod tests {

    use *;
    
    #[test]
    fn test_u8(){
        assert_eq!(Mask::mask(&0xaa, &0xff), 0xaa);
        assert_eq!(Mask::mask(&0xff, &0xaa), 0xaa);
        
        assert_eq!(Filter::is_match(&0xaa, &0xff), false);
        assert_eq!(Filter::is_match(&0xff, &0xaa), true);
    }
}
