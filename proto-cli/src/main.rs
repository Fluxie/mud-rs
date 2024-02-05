use signature_derive::Signable;

#[derive(Signable)]
pub struct TestValue {
    number: u32,
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod test {
    use crate::TestValue;
    use signature::{Signable, Signature};
    use signature_derive::Signable;

    #[derive(Signable)]
    pub struct NestedValue {
        value: TestValue,
    }

    #[test]
    fn signing_produces_expected_signature() {
        let test_value = crate::TestValue { number: 5 };
        let signature = signature::Signable::sign(&test_value);
        assert_eq!(
            signature,
            Signature::from([
                24, 47, 234, 0, 148, 6, 252, 65, 241, 241, 125, 109, 149, 214, 111, 198, 112, 192,
                101, 237, 19, 89, 127, 132, 181, 197, 203, 242, 2, 194, 43, 118
            ])
        );
        let nested = NestedValue { value: test_value };
        let signature = nested.sign();
        assert_eq!(
            signature,
            Signature::from([
                250, 153, 246, 196, 229, 31, 16, 168, 172, 102, 229, 232, 105, 44, 167, 222, 25,
                136, 21, 111, 10, 231, 33, 51, 124, 59, 10, 211, 16, 80, 139, 53
            ])
        );
    }
}
