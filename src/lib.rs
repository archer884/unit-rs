use std::ops::{ Mul, Div };

pub trait Unit: Sized {
    type Data: Mul<Output=Self::Data> + Div<Output=Self::Data>;
    type Base: Unit<Data=Self::Data, Base=Self::Base>;

    fn factor() -> Self::Data;
    fn value(&self) -> Self::Data;
    fn new(data: Self::Data) -> Self;

    fn to<T>(self) -> T where
        T: Unit<Data=Self::Data, Base=Self::Base>
    {
        T::from_base(self.to_base())
    }

    // To get the value contained by the base unit type, we multiply the value contained by the
    // derived unit type.
    fn to_base(self) -> Self::Base {
        <Self::Base as Unit>::new(self.value() / Self::factor())
    }

    // Here we reverse the process we used to get the base unit in the first place.
    fn from_base<T>(base: Self::Base) -> T where
        T: Unit<Data=Self::Data, Base=Self::Base>
    {
        T::new(base.value() * Self::factor())
    }
}

// #[macro_export] // not sure this ought to be exported; it's kind of lame...

// For my viewers, I'll explain what I mean: this macro doesn't derive `Eq` because of the
// possibility that your data type will be `f64`, in which case it's not possible to get
// an implementation of `Eq` at all. If I were going to make a serious library for doing
// this and export a macro to build a unit type for my users, I'd want it to be smart
// enough to deal with non-`Eq` types like that.
macro_rules! unit {
    ($typename:ident, $basetype:ident, $datatype:ty, $factor:expr) => {
        struct $typename($datatype);

        impl Unit for $typename {
            type Data = $datatype;
            type Base = $basetype;

            fn factor() -> Self::Data { $factor }
            fn value(&self) -> Self::Data { self.0 }

            fn new(data: Self::Data) -> Self {
                $typename(data)
            }
        }

        impl<T> From<T> for $typename where
            T: Unit<Data=<$typename as Unit>::Data, Base=<$typename as Unit>::Base>
        {
            fn from(unit: T) -> Self {
                $typename::from_base(unit.to_base())
            }
        }

        impl<T> ::std::ops::Add<T> for $typename where
            T: Unit<Data=<$typename as Unit>::Data, Base=<$typename as Unit>::Base>
        {
            type Output = $typename;
            fn add(self, rhs: T) -> Self::Output {
                Self::new(self.value() + rhs.to::<$typename>().value())
            }
        }

        impl ::std::ops::Add<$datatype> for $typename {
            type Output = $typename;
            fn add(self, rhs: $datatype) -> Self::Output {
                Self::new(self.value() + rhs)
            }
        }

        impl<T> ::std::ops::Sub<T> for $typename where
            T: Unit<Data=<$typename as Unit>::Data, Base=<$typename as Unit>::Base>
        {
            type Output = $typename;
            fn sub(self, rhs: T) -> Self::Output {
                Self::new(self.value() - rhs.to::<$typename>().value())
            }
        }

        impl ::std::ops::Sub<$datatype> for $typename {
            type Output = $typename;
            fn sub(self, rhs: $datatype) -> Self::Output {
                Self::new(self.value() - rhs)
            }
        }

        impl<T> ::std::ops::Mul<T> for $typename where
            T: Unit<Data=<$typename as Unit>::Data, Base=<$typename as Unit>::Base>
        {
            type Output = $typename;
            fn mul(self, rhs: T) -> Self::Output {
                Self::new(self.value() * rhs.to::<$typename>().value())
            }
        }

        impl ::std::ops::Mul<$datatype> for $typename {
            type Output = $typename;
            fn mul(self, rhs: $datatype) -> Self::Output {
                Self::new(self.value() * rhs)
            }
        }

        impl<T> ::std::ops::Div<T> for $typename where
            T: Unit<Data=<$typename as Unit>::Data, Base=<$typename as Unit>::Base>
        {
            type Output = $typename;
            fn div(self, rhs: T) -> Self::Output {
                Self::new(self.value() / rhs.to::<$typename>().value())
            }
        }

        impl ::std::ops::Div<$datatype> for $typename {
            type Output = $typename;
            fn div(self, rhs: $datatype) -> Self::Output {
                Self::new(self.value() / rhs)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Unit;

    unit!(Meters, Self, f64, 1.0);
    unit!(Centimeters, Meters, f64, 100.0);
    unit!(Yards, Meters, f64, 1.09361);
    unit!(Feet, Self, f64, 1.0);
    unit!(Inches, Feet, f64, 12.0);

    #[test]
    fn can_convert_centimeters_to_meters() {
        assert!(2.0 == (Centimeters(100.0).to::<Meters>() + Meters(1.0)).value());
    }

    #[test]
    fn can_convert_inches_to_feet() {
        assert!(2.0 == Inches(24.0).to::<Feet>().value());
    }

    #[test]
    fn can_convert_yards_to_meters() {
        let yards_to_meters = format!("{:.4}", Yards(1.0).to::<Meters>().value());
        assert!("0.9144" == &yards_to_meters);
    }

    // This test doesn't even compile -- correctly so
    // #[test]
    // fn can_convert_feet_to_yards() {
    //     let feet_to_yards = Feet(3.0).to::<Yards>().value();
    //     assert!(1.0 == feet_to_yards);
    // }
}
