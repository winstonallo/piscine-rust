pub type GoldNugget = u16;
pub type Iron = u32;
pub type Mercure = u64;

pub struct PhilosopherStone;

impl PhilosopherStone {
    fn transmute_iron(self, iron: Iron) -> [GoldNugget; 2] {
        // SAFETY
        // We know that both `u16` and `u32` allow all bit patterns
        unsafe { std::mem::transmute(iron) }
    }

    fn transmute_mercure(self, mercure: Mercure) -> [GoldNugget; 4] {
        // SAFETY
        // We know that both `u16` and `u32` allow all bit patterns
        unsafe { std::mem::transmute(mercure) }
    }
}

type Gold = [GoldNugget];

/// ## SAFETY
/// It is the user's responsability to guarantee that the transmutation
/// from `Self` to `[GoldNugget; x]` is valid.
pub unsafe trait Metal {}

impl PhilosopherStone {
    pub fn transmute_metal<M: Metal>(self, metal: &M) -> &Gold {
        let len = std::mem::size_of_val(metal) / std::mem::size_of::<GoldNugget>();

        // SAFETY:
        // The `Metal` trait ensures that we can perform this transmutation.
        unsafe { std::slice::from_raw_parts(metal as *const M as *const GoldNugget, len) }
    }
}

// SAFETY
// `GoldNugget` allows every bit pattern and can be transmuted into another `u16`.
unsafe impl Metal for GoldNugget {}

// SAFETY
// `Iron` allows every bit pattern and can be transmuted into two `u16`s.
unsafe impl Metal for Iron {}

// SAFETY
// `Mercure` allows every bit pattern and can be transmuted into four `u16`s.
unsafe impl Metal for Mercure {}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn transmute_both() {
        let iron = 0x01234567;
        assert_eq!(PhilosopherStone.transmute_iron(iron), [0x4567, 0x0123]);
        let mercure = 0x0123456789ABCDEF;
        assert_eq!(
            PhilosopherStone.transmute_mercure(mercure),
            [0xCDEF, 0x89AB, 0x4567, 0x0123],
        );
    }

    #[test]
    fn transmute_metal() {
        let nugget: GoldNugget = 0x1234;
        assert_eq!(PhilosopherStone.transmute_metal(&nugget), &[0x1234]);

        let iron: Iron = 0x12345678;
        assert_eq!(PhilosopherStone.transmute_metal(&iron), &[0x5678, 0x1234]);
        let mercure: Mercure = 0x0123456789ABCDEF;
        assert_eq!(
            PhilosopherStone.transmute_metal(&mercure),
            &[0xCDEF, 0x89AB, 0x4567, 0x0123],
        );
    }
}
