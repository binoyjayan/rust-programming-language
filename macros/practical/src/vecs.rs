#[macro_export]
macro_rules! avec {
    () => {
        Vec::new()
    };
    // $(,)? helps have a trailing comma
    ($($elem:expr),+ $(,)?) => {
        {
            let mut vs = Vec::new();
            $(vs.push($elem);)+
            vs
        }
    };
    ($val:expr; $n:expr) => {
        {
            let mut vs = Vec::with_capacity($n);
            vs.resize($n, $val);
            vs
        }
    };
}

trait MaxValue {
    fn max_value(&self) -> usize;
}

#[macro_export]
macro_rules! impl_max {
    ($t:ty) => {
        impl MaxValue for $t {
            fn max_value(&self) -> usize {
                <$t>::MAX as usize
            }
        }
    };
}

impl_max!(i8);
impl_max!(u8);
impl_max!(u16);

pub fn test() {
    let v1: Vec<usize> = avec![];
    println!("{:?}", v1);
    let v2: Vec<usize> = avec![1, 2, 3];
    println!("{:?}", v2);
    let v3: Vec<usize> = avec![1; 3];
    println!("{:?}", v3);

    println!("{}", 0i8.max_value());
    println!("{}", 0u8.max_value());
    println!("{}", 0u16.max_value());
}
