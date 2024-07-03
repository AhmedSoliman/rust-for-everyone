use std::marker::PhantomData;

pub trait Currency {
    const CODE: &'static str;
    const SYMBOL: &'static str;
    const RATIO: u8;
}

// pub trait Forex<C>
// where
//     Self: Currency,
//     C: Currency,
// {
//     type Output: Currency;
//     fn conversion_ratio() -> f64;
// }

// newtype pattern
#[derive(Debug, Clone, Copy)]
pub struct Amount<C> {
    pub value: i64,
    _phantom: PhantomData<C>,
}

#[derive(Debug)]
pub struct OverflowError;

impl<C: Currency> Amount<C> {
    pub fn new(amount: i64, _currency: C) -> Self {
        Amount {
            value: amount,
            _phantom: PhantomData,
        }
    }
}

trait Forex<A> {
    const INTO_RATIO: f64;
}

impl<A, B> Into<Amount<B>> for Amount<A>
where
    A: Currency,
    B: Currency,
    A: Forex<B>,
{
    fn into(amount: Amount<B>) -> Self {
        Amount {
            value: ((amount.value as f64) / <A as Forex<B>>::INTO_RATIO) as i64,
            _phantom: Default::default(),
        }
    }
}

impl<A, B> std::ops::Add<Amount<B>> for Amount<A>
where
    A: Currency,
    B: Currency,
    Amount<B>: From<Amount<A>>,
{
    type Output = Amount<B>;

    // Currency A can be added to Currency B,
    // iff An implementation exists for trait Forex<A, B>
    // O is associated type Output in Forex<A, B>::Output
    fn add(self, other: Amount<B>) -> Self::Output {
        let converted: Amount<B> = self.into();
        Amount {
            value: converted.value + other.value,
            _phantom: PhantomData,
        }
    }
}

impl<C> std::fmt::Display for Amount<C>
where
    C: Currency,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let major = self.value / C::RATIO as i64;
        let minor = self.value % C::RATIO as i64;
        if f.alternate() {
            return write!(f, "{}{}.{}", C::CODE, major, minor);
        } else {
            write!(f, "{}{}.{}", C::SYMBOL, major, minor)
        }
    }
}
