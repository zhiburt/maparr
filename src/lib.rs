#[allow(unused)]
#[doc(hidden)]
pub use paste as __private_paste;

#[macro_export]
macro_rules! static_map {
    (@ __check_uniq_ident $($idents:ident)*) => {
        {
            #[allow(dead_code, non_camel_case_types)]
            enum Idents { $($idents,)* }
        }
    };
    (@ __check_size_ident $name:ident $($idents:ident)*) => {
        {
            let size = $crate::static_map!(@ __count_ids $($idents),*);
            let expected = $name::len();
            if size > expected {
                panic!(concat!("parameter list is too big"));
            } else if size < expected {
                panic!(concat!("parameter list is too small"));
            }
        }
    };
    (@ __check_order_ident $name:ident $($idents:ident)*) => {
        #[allow(unused_assignments)]
        {
            let mut index = 0;

            $(
                let ident_value = $name::$idents.0;
                if ident_value != index {
                    panic!(concat!("parameter position", "<", stringify!($idents), ">", " does not correspond to it's original position "));
                }

                index += 1;
            )*
        }
    };
    (@ __count_ids $first:ident) => { 1 };
    (@ __count_ids $first:ident, $($rest:ident),*) => { 1 + $crate::static_map!(@ __count_ids $($rest),*) };
    (@ __gen_property $struct_name:path, $index:expr, $first:ident) => {
        /// ID
        #[doc = stringify!($first)]
        pub const $first: $struct_name = $struct_name($index);
    };
    (@ __gen_property $struct_name:path, $index:expr, $first:ident, $($rest:ident),*) => {
        /// ID
        #[doc = stringify!($first)]
       pub const $first: $struct_name = $struct_name($index);

       $crate::static_map!(@ __gen_property $struct_name, $index + 1, $($rest),*);
    };
    (  $(#[$($derive_block:tt)*])* $publicity:vis $name:ident; $($id:ident),* $(,)?) => {
        $crate::__private_paste::paste!{
            /// A small hashmap backed by an array.
            $(
                #[$($derive_block)*]
            )*
            $publicity struct $name<T> {
                list: [T; [<__private_size_ $name>]::SIZE],
            }

            #[doc(hidden)]
            #[allow(non_snake_case)]
            mod [<__private_size_ $name>] {
                pub(super) const SIZE: usize = $crate::static_map!(@ __count_ids $($id),*);
            }

            #[doc(hidden)]
            #[allow(non_snake_case)]
            mod [<__private_id_ $name>] {
                /// An ID type.
                #[derive(Debug, Clone, Copy, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
                pub struct ID(pub usize);

                impl From<ID> for usize {
                    fn from(id: ID) -> usize {
                        id.0
                    }
                }
            }

            impl $name<()> {
                $crate::static_map!(@ __gen_property [<__private_id_ $name>]::ID, 0, $($id),*);
            }

            #[allow(unused)]
            impl<T> $name<T> {
                /// Creates a new structure.
                #[allow(non_snake_case)]
                pub fn new(
                    $(
                        $id: ([<__private_id_ $name>]::ID, T)
                    ),*
                ) -> Self {
                    $(
                        if $id.0.0 != $name::$id.0 {
                            panic!(
                                concat!(
                                    "parameter ", "<", stringify!($id), ">",
                                    " does not correspond to it's value id, expected it being ",
                                    "<", stringify!($name), "::", stringify!($id), ">",
                                )
                            );
                        }
                    )*

                    let list = [
                        $(
                            $id.1
                        ),*
                    ];

                    Self {
                        list,
                    }
                }
            }

            #[allow(unused)]
            impl<T> $name<T> {
                /// Get an object by an id.
                pub const fn get(&self, id: [<__private_id_ $name>]::ID) -> &T {
                    &self.list[id.0]
                }

                /// Get a mutable object by an id.
                pub fn get_mut(&mut self, id: [<__private_id_ $name>]::ID) -> &mut T {
                    &mut self.list[id.0]
                }

                /// Set an object by an id.
                pub fn set(&mut self, id: [<__private_id_ $name>]::ID, value: T) {
                    self.list[id.0] = value
                }

                /// Iterate objects.
                pub fn iter(&self) -> impl IntoIterator<Item=&T> {
                    self.list.iter()
                }

                /// Iterate objects mutable.
                pub fn iter_mut(&mut self) -> impl IntoIterator<Item=&mut T> {
                    self.list.iter_mut()
                }

                /// Get an list of keys supported.
                pub fn keys() -> [[<__private_id_ $name>]::ID; [<__private_size_ $name>]::SIZE] {
                    [
                        $(
                            $name::$id
                        ),*
                    ]
                }
            }

            #[allow(unused)]
            impl $name<()> {
                /// Get a map size.
                pub const fn len() -> usize {
                    [<__private_size_ $name>]::SIZE
                }

                /// Check whether the map is empty.
                pub const fn is_empty() -> bool {
                    [<__private_size_ $name>]::SIZE == 0
                }
            }

            impl<T> core::ops::Index<[<__private_id_ $name>]::ID> for $name<T> {
                type Output = T;

                fn index(&self, index: [<__private_id_ $name>]::ID) -> &Self::Output {
                    self.get(index)
                }
            }

            impl<T> core::ops::IndexMut<[<__private_id_ $name>]::ID> for $name<T> {
                fn index_mut(&mut self, index: [<__private_id_ $name>]::ID) -> &mut Self::Output {
                    self.get_mut(index)
                }
            }

            impl<T> From<$name<T>> for [T; [<__private_size_ $name>]::SIZE] {
                fn from(map: $name<T>) -> [T; [<__private_size_ $name>]::SIZE] {
                    map.list
                }
            }

            impl<T> core::convert::AsRef<[T]> for $name<T> {
                fn as_ref(&self) -> &[T] {
                    &self.list
                }
            }

            impl<T> core::iter::IntoIterator for $name<T> {
                type Item = T;
                type IntoIter = core::array::IntoIter<Self::Item, { [<__private_size_ $name>]::SIZE }>;

                fn into_iter(self) -> Self::IntoIter {
                    self.list.into_iter()
                }
            }
        }
    };

    (  $(#[$($derive_block:tt)*])* $publicity:vis $name:ident<$name_type:ty>; $($id:ident),* $(,)?) => {
        $crate::__private_paste::paste!{
            /// A small hashmap backed by an array.
            $(
                #[$($derive_block)*]
            )*
            $publicity struct $name {
                list: [$name_type; [<__private_size_ $name>]::SIZE],
            }

            #[doc(hidden)]
            #[allow(non_snake_case)]
            mod [<__private_size_ $name>] {
                pub(super) const SIZE: usize = $crate::static_map!(@ __count_ids $($id),*);
            }

            #[doc(hidden)]
            #[allow(non_snake_case)]
            mod [<__private_id_ $name>] {
                /// An ID type.
                #[derive(Debug, Clone, Copy, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
                pub struct ID(pub usize);

                impl From<ID> for usize {
                    fn from(id: ID) -> usize {
                        id.0
                    }
                }
            }

            impl $name {
                $crate::static_map!(@ __gen_property [<__private_id_ $name>]::ID, 0, $($id),*);
            }

            #[allow(unused)]
            impl $name {
                /// Creates a new structure.
                #[allow(non_snake_case)]
                pub fn new(
                    $(
                        $id: ([<__private_id_ $name>]::ID, $name_type)
                    ),*
                ) -> Self {
                    $(
                        if $id.0.0 != $name::$id.0 {
                            panic!(
                                concat!(
                                    "parameter ", "<", stringify!($id), ">",
                                    " does not correspond to it's value id, expected it being ",
                                    "<", stringify!($name), "::", stringify!($id), ">",
                                )
                            );
                        }
                    )*

                    let list = [
                        $(
                            $id.1
                        ),*
                    ];

                    Self {
                        list,
                    }
                }
            }

            #[allow(unused)]
            impl $name {
                /// Get an object by an id.
                pub const fn get(&self, id: [<__private_id_ $name>]::ID) -> & $name_type {
                    &self.list[id.0]
                }

                /// Get a mutable object by an id.
                pub fn get_mut(&mut self, id: [<__private_id_ $name>]::ID) -> &mut $name_type {
                    &mut self.list[id.0]
                }

                /// Set an object by an id.
                pub fn set(&mut self, id: [<__private_id_ $name>]::ID, value: $name_type) {
                    self.list[id.0] = value
                }

                /// Iterate objects.
                pub fn iter(&self) -> impl IntoIterator<Item=& $name_type> {
                    self.list.iter()
                }

                /// Iterate objects mutable.
                pub fn iter_mut(&mut self) -> impl IntoIterator<Item=&mut $name_type> {
                    self.list.iter_mut()
                }

                /// Get an list of keys supported.
                pub fn keys() -> [[<__private_id_ $name>]::ID; [<__private_size_ $name>]::SIZE] {
                    [
                        $(
                            $name::$id
                        ),*
                    ]
                }
            }

            #[allow(unused)]
            impl $name {
                /// Get a map size.
                pub const fn len() -> usize {
                    [<__private_size_ $name>]::SIZE
                }

                /// Check whether the map is empty.
                pub const fn is_empty() -> bool {
                    [<__private_size_ $name>]::SIZE == 0
                }
            }

            impl core::ops::Index<[<__private_id_ $name>]::ID> for $name {
                type Output = $name_type;

                fn index(&self, index: [<__private_id_ $name>]::ID) -> &Self::Output {
                    self.get(index)
                }
            }

            impl core::ops::IndexMut<[<__private_id_ $name>]::ID> for $name {
                fn index_mut(&mut self, index: [<__private_id_ $name>]::ID) -> &mut Self::Output {
                    self.get_mut(index)
                }
            }

            impl From<$name> for [$name_type; [<__private_size_ $name>]::SIZE] {
                fn from(map: $name) -> [$name_type; [<__private_size_ $name>]::SIZE] {
                    map.list
                }
            }

            impl core::convert::AsRef<[$name_type]> for $name {
                fn as_ref(&self) -> &[$name_type] {
                    &self.list
                }
            }

            impl core::iter::IntoIterator for $name {
                type Item = $name_type;
                type IntoIter = core::array::IntoIter<Self::Item, { [<__private_size_ $name>]::SIZE }>;

                fn into_iter(self) -> Self::IntoIter {
                    self.list.into_iter()
                }
            }
        }
    };
    ( $name:ident; $($id:ident = $id_value:expr),* $(,)?) => {
        {
            $crate::static_map!(@ __check_uniq_ident $($id)*);
            $crate::static_map!(@ __check_order_ident $name $($id)*);
            $crate::static_map!(@ __check_size_ident $name $($id)*);

            $name {
                list: [
                    $($id_value),*
                ],
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_static_map_generic() {
        static_map!(Map; ID1, ID2);

        let map = Map::new((Map::ID1, 1), (Map::ID2, 2));
        assert_eq!(map[Map::ID1], 1);
        assert_eq!(map[Map::ID2], 2);
    }

    #[test]
    fn test_static_map() {
        static_map!(Map<usize>; ID1, ID2);

        let map = Map::new((Map::ID1, 1), (Map::ID2, 2));
        assert_eq!(map[Map::ID1], 1);
        assert_eq!(map[Map::ID2], 2);
    }

    #[test]
    fn test_static_map_value() {
        static_map!(Map<usize>; ID1, ID2);
        const MAP: Map = static_map!(Map; ID1 = 1, ID2 = 2);

        assert_eq!(MAP[Map::ID1], 1);
        assert_eq!(MAP[Map::ID2], 2);
    }

    #[test]
    fn test_static_map_generic_value() {
        static_map!(Map; ID1, ID2);
        const MAP: Map<bool> = static_map!(Map; ID1 = false, ID2 = true);

        assert!(!MAP[Map::ID1]);
        assert!(MAP[Map::ID2]);
    }

    #[test]
    #[should_panic]
    fn test_static_map_generic_value_wrong_sorting() {
        static_map!(Map; ID1, ID2);
        let _map = static_map!(Map; ID2 = false, ID1 = false);
    }

    #[test]
    #[should_panic]
    fn test_static_map_value_wrong_sorting() {
        static_map!(Map<bool>; ID1, ID2);
        let _map = static_map!(Map; ID2 = false, ID1 = false);
    }

    #[allow(non_upper_case_globals)]
    #[rustfmt::skip]
    #[test]
    #[ignore]
    fn compile_time_checks() {
        #[derive(Debug, Clone, Hash)]
        pub struct Somestruct0;

        #[derive(Debug, Clone, Hash)]
        pub struct Somestruct1(usize, bool);

        #[derive(Debug, Clone, Hash)]
        pub struct Somestruct2 {
            a: usize,
            b: bool,
        }

        macro_rules! test_type {
            ($($list:ty)*) => {
                $(
                    { static_map!(pub           Map<$list>; ID1, ID2, ID_3, id1, id_1); }
                    { static_map!(              Map<$list>; ID1, ID2, ID_3, id1, id_1); }
                    { static_map!(pub(crate)    Map<$list>; ID1, ID2, ID_3, id1, id_1); }

                    {
                        static_map!(
                            #[derive(Debug)]
                            Map<$list>;
                            ID1, ID2, ID_3, id1, id_1
                        );
                    }

                    {
                        static_map!(
                            #[derive(Debug, Clone)]
                            #[derive(Hash)]
                            Map<$list>;
                            ID1, ID2, ID_3, id1, id_1
                        );
                    }
                )*
            };
        }

        test_type!(u8 u16 u32 u64 u128 usize i8 i16 i32 i64 i128 isize bool);
        test_type!(Somestruct0 Somestruct1 Somestruct2);
    }

    #[rustfmt::skip]
    #[test]
    fn check_interface() {
        static_map!(#[derive(Debug, Clone)] Map; ID_1, ID_2, ID_3, ID_4);

        let mut value = Map::new(
            (Map::ID_1, "Hello"),
            (Map::ID_2, "World"),
            (Map::ID_3, "Halo"),
            (Map::ID_4, "Earth"),
        );

        assert_eq!(Map::len(), 4);
        assert!(!Map::is_empty());

        let dbg = format!("{value:?}");
        assert_eq!(dbg, "Map { list: [\"Hello\", \"World\", \"Halo\", \"Earth\"] }");

        let values = value.iter().into_iter().cloned().collect::<Vec<_>>();
        assert_eq!(values, vec!["Hello", "World", "Halo", "Earth"]);

        let values = value.clone().into_iter().collect::<Vec<_>>();
        assert_eq!(values, vec!["Hello", "World", "Halo", "Earth"]);

        let values = value.as_ref();
        assert_eq!(values, &["Hello", "World", "Halo", "Earth"]);

        let id_value = value.get(Map::ID_1);
        assert_eq!(*id_value, "Hello");

        let id_value = value[Map::ID_1];
        assert_eq!(id_value, "Hello");

        value.set(Map::ID_2, "123456789");
        let id_value = value.get(Map::ID_2);
        assert_eq!(*id_value, "123456789");

        value[Map::ID_2] = "1234567890";
        let id_value = value.get(Map::ID_2);
        assert_eq!(*id_value, "1234567890");
    }

    #[rustfmt::skip]
    #[test]
    fn check_interface_generic() {
        static_map!(#[derive(Debug, Clone)] Map<String>; ID_1, ID_2, ID_3, ID_4);

        let mut value = Map::new(
            (Map::ID_1, String::from("Hello")),
            (Map::ID_2, String::from("World")),
            (Map::ID_3, String::from("Halo")),
            (Map::ID_4, String::from("Earth")),
        );

        assert_eq!(Map::len(), 4);
        assert!(!Map::is_empty());

        let dbg = format!("{value:?}");
        assert_eq!(dbg, "Map { list: [\"Hello\", \"World\", \"Halo\", \"Earth\"] }");

        let values = value.iter().into_iter().cloned().collect::<Vec<_>>();
        assert_eq!(values, vec!["Hello", "World", "Halo", "Earth"]);

        let values = value.clone().into_iter().collect::<Vec<_>>();
        assert_eq!(values, vec!["Hello", "World", "Halo", "Earth"]);

        let values = value.as_ref();
        assert_eq!(values, &["Hello", "World", "Halo", "Earth"]);

        let id_value = value.get(Map::ID_1);
        assert_eq!(*id_value, "Hello");

        let id_value = &value[Map::ID_1];
        assert_eq!(id_value, "Hello");

        value.set(Map::ID_2, String::from("123456789"));
        let id_value = value.get(Map::ID_2);
        assert_eq!(*id_value, "123456789");

        value[Map::ID_2] = String::from("1234567890");
        let id_value = value.get(Map::ID_2);
        assert_eq!(*id_value, "1234567890");
    }
}
