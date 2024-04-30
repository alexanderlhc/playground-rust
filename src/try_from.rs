/*
Displays the use of TryFrom and TryInto traits.

Used for:
- Converting between types that may fail.
- They are reciprocal of each other.
  - If you implement TryFrom for a type, you get TryInto for free.
*/

#[derive(Debug, PartialEq)]
struct EvenNumber(i32);

impl std::convert::TryFrom<i32> for EvenNumber {
    type Error = &'static str;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value % 2 == 0 {
            Ok(EvenNumber(value))
        } else {
            Err("Not an even number")
        }
    }
}

// impl std::convert::TryInto<EvenNumber> for i32 {
//     type Error = &'static str;
//     fn try_into(self) -> Result<EvenNumber, Self::Error> {
//         if self % 2 == 0 {
//             Ok(EvenNumber(self))
//         } else {
//             Err("Not an even number")
//         }
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_try_from() {
        assert_eq!(EvenNumber::try_from(8), Ok(EvenNumber(8)));
        assert_eq!(EvenNumber::try_from(5), Err("Not an even number"));
    }
    #[test]
    fn test_try_into() {
        assert_eq!(8.try_into(), Ok(EvenNumber(8)));
        // let res: Result<EvenNumber, &str> = 9.try_into();
        // assert_eq!(5.try_into(), Err("Not an even number"))

        assert_eq!(
            TryInto::<EvenNumber>::try_into(5),
            Err("Not an even number")
        );
    }

    #[test]
    fn test_try_into_unwrap() {
        let result: Result<EvenNumber, &str> = 8.try_into();
        assert_eq!(result.unwrap(), EvenNumber(8));
    }

    #[test]
    fn test_try_into_unwrap_err() {
        let result: Result<EvenNumber, &str> = 5.try_into();
        assert_eq!(result.unwrap_err(), "Not an even number");
    }
}
