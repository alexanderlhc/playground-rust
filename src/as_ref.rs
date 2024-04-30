/*
As Ref is a trait that is used to convert a value to a reference.

This is useful when you want to write a function that takes a reference as an argument but you want to pass a value to it.

Similar to From or AsMut.
From is used to convert a value to another type.

Used for:
- Converting a value to a reference
- Avoiding copying values
- Writing functions that take references as arguments
*/

struct Dkk(i32);

impl AsRef<i32> for Dkk {
    fn as_ref(&self) -> &i32 {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_as_ref() {
        let dkk = Dkk(42);
        let reference = dkk.as_ref();
        assert_eq!(&42, reference);
    }

    #[test]
    fn test_as_ref_function() {
        let dkk = Dkk(42);
        foo(dkk);
    }

    fn foo<T: AsRef<i32>>(x: T) {
        assert_eq!(&42, x.as_ref());
    }
}
