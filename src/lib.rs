use std::marker::PhantomData;

trait Nat {}

struct Zero;

struct Succ<N: Nat>(PhantomData<N>);

impl Nat for Zero {}

impl<N: Nat> Nat for Succ<N> {}

trait IsMod3 {}

impl IsMod3 for Succ<Succ<Succ<Zero>>> {}

impl<N: Nat + IsMod3> IsMod3 for Succ<Succ<Succ<N>>> {}

trait IsMod5 {}

impl IsMod5 for Succ<Succ<Succ<Succ<Succ<Zero>>>>> {}

impl<N: Nat + IsMod5> IsMod5 for Succ<Succ<Succ<Succ<Succ<N>>>>> {}

#[cfg(test)]
mod tests {
    use crate::{IsMod3, IsMod5, Succ, Zero};
    use std::marker::PhantomData;

    type N0 = Zero;
    type N1 = Succ<N0>;
    type N2 = Succ<N1>;
    type N3 = Succ<N2>;
    type N4 = Succ<N3>;
    type N5 = Succ<N4>;
    type N6 = Succ<N5>;
    type N7 = Succ<N6>;
    type N8 = Succ<N7>;
    type N9 = Succ<N8>;
    type N10 = Succ<N9>;
    type N11 = Succ<N10>;
    type N12 = Succ<N11>;
    type N13 = Succ<N12>;
    type N14 = Succ<N13>;
    type N15 = Succ<N14>;

    struct Fizz<N: IsMod3>(PhantomData<N>);

    struct Buzz<N: IsMod5>(PhantomData<N>);

    struct Fizzbuzz<N: IsMod3 + IsMod5>(PhantomData<N>);

    #[test]
    fn test_fizz() {
        let _: Fizz<N3>;
        let _: Fizz<N6>;
    }

    #[test]
    fn test_buzz() {
        let _: Buzz<N5>;
        let _: Buzz<N10>;
    }

    #[test]
    fn test_fizzbuzz() {
        let _: Fizzbuzz<N15>;
    }
}
