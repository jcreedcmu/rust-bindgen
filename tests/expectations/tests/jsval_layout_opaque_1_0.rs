/* automatically generated by rust-bindgen */


#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]


#[repr(C)]
#[derive(Copy, Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct __BindgenBitfieldUnit<Storage, Align>
where
    Storage: AsRef<[u8]> + AsMut<[u8]>,
{
    storage: Storage,
    align: [Align; 0],
}

impl<Storage, Align> __BindgenBitfieldUnit<Storage, Align>
where
    Storage: AsRef<[u8]> + AsMut<[u8]>,
{
    #[inline]
    pub fn new(storage: Storage) -> Self {
        Self { storage, align: [] }
    }

    #[inline]
    pub fn get_bit(&self, index: usize) -> bool {
        debug_assert!(index / 8 < self.storage.as_ref().len());

        let byte_index = index / 8;
        let byte = self.storage.as_ref()[byte_index];

        let bit_index = index % 8;
        let mask = 1 << bit_index;

        byte & mask == mask
    }

    #[inline]
    pub fn set_bit(&mut self, index: usize, val: bool) {
        debug_assert!(index / 8 < self.storage.as_ref().len());

        let byte_index = index / 8;
        let byte = &mut self.storage.as_mut()[byte_index];

        let bit_index = index % 8;
        let mask = 1 << bit_index;

        if val {
            *byte |= mask;
        } else {
            *byte &= !mask;
        }
    }

    #[inline]
    pub fn get(&self, bit_offset: usize, bit_width: u8) -> u64 {
        debug_assert!(bit_width <= 64);
        debug_assert!(bit_offset / 8 < self.storage.as_ref().len());
        debug_assert!((bit_offset + (bit_width as usize)) / 8 <= self.storage.as_ref().len());

        let mut val = 0;

        for i in 0..(bit_width as usize) {
            if self.get_bit(i + bit_offset) {
                val |= 1 << i;
            }
        }

        val
    }

    #[inline]
    pub fn set(&mut self, bit_offset: usize, bit_width: u8, val: u64) {
        debug_assert!(bit_width <= 64);
        debug_assert!(bit_offset / 8 < self.storage.as_ref().len());
        debug_assert!((bit_offset + (bit_width as usize)) / 8 <= self.storage.as_ref().len());

        for i in 0..(bit_width as usize) {
            let mask = 1 << i;
            let val_bit_is_set = val & mask == mask;
            self.set_bit(i + bit_offset, val_bit_is_set);
        }
    }
}
#[repr(C)]
pub struct __BindgenUnionField<T>(::std::marker::PhantomData<T>);
impl<T> __BindgenUnionField<T> {
    #[inline]
    pub fn new() -> Self {
        __BindgenUnionField(::std::marker::PhantomData)
    }
    #[inline]
    pub unsafe fn as_ref(&self) -> &T {
        ::std::mem::transmute(self)
    }
    #[inline]
    pub unsafe fn as_mut(&mut self) -> &mut T {
        ::std::mem::transmute(self)
    }
}
impl<T> ::std::default::Default for __BindgenUnionField<T> {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}
impl<T> ::std::clone::Clone for __BindgenUnionField<T> {
    #[inline]
    fn clone(&self) -> Self {
        Self::new()
    }
}
impl<T> ::std::marker::Copy for __BindgenUnionField<T> {}
impl<T> ::std::fmt::Debug for __BindgenUnionField<T> {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        fmt.write_str("__BindgenUnionField")
    }
}
impl<T> ::std::hash::Hash for __BindgenUnionField<T> {
    fn hash<H: ::std::hash::Hasher>(&self, _state: &mut H) {}
}
impl<T> ::std::cmp::PartialEq for __BindgenUnionField<T> {
    fn eq(&self, _other: &__BindgenUnionField<T>) -> bool {
        true
    }
}
impl<T> ::std::cmp::Eq for __BindgenUnionField<T> {}
pub const JSVAL_TAG_SHIFT: ::std::os::raw::c_uint = 47;
pub const JSVAL_PAYLOAD_MASK: ::std::os::raw::c_ulonglong = 140737488355327;
pub const JSVAL_TAG_MASK: ::std::os::raw::c_longlong = -140737488355328;
#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum JSValueType {
    JSVAL_TYPE_DOUBLE = 0,
    JSVAL_TYPE_INT32 = 1,
    JSVAL_TYPE_UNDEFINED = 2,
    JSVAL_TYPE_BOOLEAN = 3,
    JSVAL_TYPE_MAGIC = 4,
    JSVAL_TYPE_STRING = 5,
    JSVAL_TYPE_SYMBOL = 6,
    JSVAL_TYPE_NULL = 7,
    JSVAL_TYPE_OBJECT = 8,
    JSVAL_TYPE_UNKNOWN = 32,
    JSVAL_TYPE_MISSING = 33,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum JSValueTag {
    JSVAL_TAG_MAX_DOUBLE = 131056,
    JSVAL_TAG_INT32 = 131057,
    JSVAL_TAG_UNDEFINED = 131058,
    JSVAL_TAG_STRING = 131061,
    JSVAL_TAG_SYMBOL = 131062,
    JSVAL_TAG_BOOLEAN = 131059,
    JSVAL_TAG_MAGIC = 131060,
    JSVAL_TAG_NULL = 131063,
    JSVAL_TAG_OBJECT = 131064,
}
#[repr(u64)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum JSValueShiftedTag {
    JSVAL_SHIFTED_TAG_MAX_DOUBLE = 18444492278190833663,
    JSVAL_SHIFTED_TAG_INT32 = 18444633011384221696,
    JSVAL_SHIFTED_TAG_UNDEFINED = 18444773748872577024,
    JSVAL_SHIFTED_TAG_STRING = 18445195961337643008,
    JSVAL_SHIFTED_TAG_SYMBOL = 18445336698825998336,
    JSVAL_SHIFTED_TAG_BOOLEAN = 18444914486360932352,
    JSVAL_SHIFTED_TAG_MAGIC = 18445055223849287680,
    JSVAL_SHIFTED_TAG_NULL = 18445477436314353664,
    JSVAL_SHIFTED_TAG_OBJECT = 18445618173802708992,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum JSWhyMagic {
    JS_ELEMENTS_HOLE = 0,
    JS_NO_ITER_VALUE = 1,
    JS_GENERATOR_CLOSING = 2,
    JS_NO_CONSTANT = 3,
    JS_THIS_POISON = 4,
    JS_ARG_POISON = 5,
    JS_SERIALIZE_NO_NODE = 6,
    JS_LAZY_ARGUMENTS = 7,
    JS_OPTIMIZED_ARGUMENTS = 8,
    JS_IS_CONSTRUCTING = 9,
    JS_OVERWRITTEN_CALLEE = 10,
    JS_BLOCK_NEEDS_CLONE = 11,
    JS_HASH_KEY_EMPTY = 12,
    JS_ION_ERROR = 13,
    JS_ION_BAILOUT = 14,
    JS_OPTIMIZED_OUT = 15,
    JS_UNINITIALIZED_LEXICAL = 16,
    JS_GENERIC_MAGIC = 17,
    JS_WHY_MAGIC_COUNT = 18,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Hash, PartialEq)]
pub struct jsval_layout {
    pub asBits: __BindgenUnionField<u64>,
    pub debugView: __BindgenUnionField<jsval_layout__bindgen_ty_1>,
    pub s: __BindgenUnionField<jsval_layout__bindgen_ty_2>,
    pub asDouble: __BindgenUnionField<f64>,
    pub asPtr: __BindgenUnionField<*mut ::std::os::raw::c_void>,
    pub asWord: __BindgenUnionField<usize>,
    pub asUIntPtr: __BindgenUnionField<usize>,
    pub bindgen_union_field: u64,
}
#[repr(C)]
#[derive(Debug, Copy, Hash, PartialEq, Eq)]
pub struct jsval_layout__bindgen_ty_1 {
    pub _bitfield_1: __BindgenBitfieldUnit<[u8; 8usize], u64>,
    pub __bindgen_align: [u64; 0usize],
}
#[test]
fn bindgen_test_layout_jsval_layout__bindgen_ty_1() {
    assert_eq!(
        ::std::mem::size_of::<jsval_layout__bindgen_ty_1>(),
        8usize,
        concat!("Size of: ", stringify!(jsval_layout__bindgen_ty_1))
    );
    assert_eq!(
        ::std::mem::align_of::<jsval_layout__bindgen_ty_1>(),
        8usize,
        concat!("Alignment of ", stringify!(jsval_layout__bindgen_ty_1))
    );
}
impl Clone for jsval_layout__bindgen_ty_1 {
    fn clone(&self) -> Self {
        *self
    }
}
impl Default for jsval_layout__bindgen_ty_1 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl jsval_layout__bindgen_ty_1 {
    #[inline]
    pub fn payload47(&self) -> u64 {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(0usize, 47u8) as u64) }
    }
    #[inline]
    pub fn set_payload47(&mut self, val: u64) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            self._bitfield_1.set(0usize, 47u8, val as u64)
        }
    }
    #[inline]
    pub fn tag(&self) -> JSValueTag {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(47usize, 17u8) as u32) }
    }
    #[inline]
    pub fn set_tag(&mut self, val: JSValueTag) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(47usize, 17u8, val as u64)
        }
    }
    #[inline]
    pub fn new_bitfield_1(
        payload47: u64,
        tag: JSValueTag,
    ) -> __BindgenBitfieldUnit<[u8; 8usize], u64> {
        let mut __bindgen_bitfield_unit: __BindgenBitfieldUnit<[u8; 8usize], u64> =
            Default::default();
        __bindgen_bitfield_unit.set(0usize, 47u8, {
            let payload47: u64 = unsafe { ::std::mem::transmute(payload47) };
            payload47 as u64
        });
        __bindgen_bitfield_unit.set(47usize, 17u8, {
            let tag: u32 = unsafe { ::std::mem::transmute(tag) };
            tag as u64
        });
        __bindgen_bitfield_unit
    }
}
#[repr(C)]
#[derive(Debug, Default, Copy, Hash, PartialEq, Eq)]
pub struct jsval_layout__bindgen_ty_2 {
    pub payload: jsval_layout__bindgen_ty_2__bindgen_ty_1,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Hash, PartialEq, Eq)]
pub struct jsval_layout__bindgen_ty_2__bindgen_ty_1 {
    pub i32: __BindgenUnionField<i32>,
    pub u32: __BindgenUnionField<u32>,
    pub why: __BindgenUnionField<JSWhyMagic>,
    pub bindgen_union_field: u32,
}
#[test]
fn bindgen_test_layout_jsval_layout__bindgen_ty_2__bindgen_ty_1() {
    assert_eq!(
        ::std::mem::size_of::<jsval_layout__bindgen_ty_2__bindgen_ty_1>(),
        4usize,
        concat!(
            "Size of: ",
            stringify!(jsval_layout__bindgen_ty_2__bindgen_ty_1)
        )
    );
    assert_eq!(
        ::std::mem::align_of::<jsval_layout__bindgen_ty_2__bindgen_ty_1>(),
        4usize,
        concat!(
            "Alignment of ",
            stringify!(jsval_layout__bindgen_ty_2__bindgen_ty_1)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<jsval_layout__bindgen_ty_2__bindgen_ty_1>())).i32 as *const _
                as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(jsval_layout__bindgen_ty_2__bindgen_ty_1),
            "::",
            stringify!(i32)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<jsval_layout__bindgen_ty_2__bindgen_ty_1>())).u32 as *const _
                as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(jsval_layout__bindgen_ty_2__bindgen_ty_1),
            "::",
            stringify!(u32)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<jsval_layout__bindgen_ty_2__bindgen_ty_1>())).why as *const _
                as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(jsval_layout__bindgen_ty_2__bindgen_ty_1),
            "::",
            stringify!(why)
        )
    );
}
impl Clone for jsval_layout__bindgen_ty_2__bindgen_ty_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[test]
fn bindgen_test_layout_jsval_layout__bindgen_ty_2() {
    assert_eq!(
        ::std::mem::size_of::<jsval_layout__bindgen_ty_2>(),
        4usize,
        concat!("Size of: ", stringify!(jsval_layout__bindgen_ty_2))
    );
    assert_eq!(
        ::std::mem::align_of::<jsval_layout__bindgen_ty_2>(),
        4usize,
        concat!("Alignment of ", stringify!(jsval_layout__bindgen_ty_2))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<jsval_layout__bindgen_ty_2>())).payload as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(jsval_layout__bindgen_ty_2),
            "::",
            stringify!(payload)
        )
    );
}
impl Clone for jsval_layout__bindgen_ty_2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[test]
fn bindgen_test_layout_jsval_layout() {
    assert_eq!(
        ::std::mem::size_of::<jsval_layout>(),
        8usize,
        concat!("Size of: ", stringify!(jsval_layout))
    );
    assert_eq!(
        ::std::mem::align_of::<jsval_layout>(),
        8usize,
        concat!("Alignment of ", stringify!(jsval_layout))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<jsval_layout>())).asBits as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(jsval_layout),
            "::",
            stringify!(asBits)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<jsval_layout>())).debugView as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(jsval_layout),
            "::",
            stringify!(debugView)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<jsval_layout>())).s as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(jsval_layout),
            "::",
            stringify!(s)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<jsval_layout>())).asDouble as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(jsval_layout),
            "::",
            stringify!(asDouble)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<jsval_layout>())).asPtr as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(jsval_layout),
            "::",
            stringify!(asPtr)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<jsval_layout>())).asWord as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(jsval_layout),
            "::",
            stringify!(asWord)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<jsval_layout>())).asUIntPtr as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(jsval_layout),
            "::",
            stringify!(asUIntPtr)
        )
    );
}
impl Clone for jsval_layout {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[derive(Debug, Default, Copy, Hash, PartialEq)]
pub struct Value {
    pub data: jsval_layout,
}
#[test]
fn bindgen_test_layout_Value() {
    assert_eq!(
        ::std::mem::size_of::<Value>(),
        8usize,
        concat!("Size of: ", stringify!(Value))
    );
    assert_eq!(
        ::std::mem::align_of::<Value>(),
        8usize,
        concat!("Alignment of ", stringify!(Value))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Value>())).data as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(Value),
            "::",
            stringify!(data)
        )
    );
}
impl Clone for Value {
    fn clone(&self) -> Self {
        *self
    }
}
