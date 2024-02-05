use byteorder::ByteOrder;
use sha3::digest::Update;
use sha3::Digest;

#[derive(Eq, PartialEq, Debug)]
pub struct Signature {
    signature: [u8; 32],
}

/// A trait to signal the struct can be signed.
pub trait Signable {
    /// TODO: Add "key" parameter and update the signature accordingly.
    fn sign(&self) -> Signature;
}

impl From<[u8; 32]> for Signature {
    fn from(value: [u8; 32]) -> Self {
        Signature { signature: value }
    }
}

// TODO: proc_macro for easily generating implementation for different primitives.
impl Signable for u32 {
    fn sign(&self) -> Signature {
        // TODO: How to ensure number signed with different signature functions produce different signatures?
        let mut buf = [0; 4];
        byteorder::NetworkEndian::write_u32(&mut buf, *self);

        let mut hasher = sha3::Sha3_256::new();
        Update::update(&mut hasher, &buf);
        Signature {
            signature: hasher.finalize().into(),
        }
    }
}

// TODO: proc_macro for easily generating implementation for different "array" types.
impl Signable for &u32 {
    fn sign(&self) -> Signature {
        // TODO: How to ensure number signed with different signature functions produce different signatures?
        let mut buf = [0; 4];
        byteorder::NetworkEndian::write_u32(&mut buf, **self);

        let mut hasher = sha3::Sha3_256::new();
        Update::update(&mut hasher, &buf);
        Signature {
            signature: hasher.finalize().into(),
        }
    }
}

impl Signable for &[Signature] {
    fn sign(&self) -> Signature {
        sign_signatures(self.iter())
    }
}

impl Signable for [Signature] {
    fn sign(&self) -> Signature {
        sign_signatures(self.iter())
    }
}

fn sign_signatures<'a, I>(iter: I) -> Signature
where
    I: Iterator<Item = &'a Signature>,
{
    let hasher = iter.fold(sha3::Sha3_256::new(), |mut a, v| {
        Update::update(&mut a, v.signature.as_slice());
        a
    });
    Signature {
        signature: hasher.finalize().into(),
    }
}

#[cfg(test)]
mod tests {}
