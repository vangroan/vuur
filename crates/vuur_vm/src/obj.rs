//! Dynamic Object
//!
//! References:
//! - https://en.wikipedia.org/wiki/Data_structure_alignment
use std::{alloc::Layout, cmp::Reverse, fmt, rc::Rc};

// TODO: ObjInfo identity to detect cycling types

// ----------------------------------------------------------------------------
// Object Value

// TODO: Reference counting is temporary solution for prototype. Replace with less overhead.
pub struct Obj {
    data: Box<[u8]>,
    ty: Rc<ObjInfo>,
}

impl Obj {
    pub fn new(ty: Rc<ObjInfo>) -> Self {
        debug_assert!(!ty.layout.is_empty(), "zero sized types are not supported");

        Self {
            data: Self::make_storage(ty.as_ref()),
            ty,
        }
    }

    fn make_storage(ty: &ObjInfo) -> Box<[u8]> {
        (0..ty.size).map(|_| 0).collect::<Vec<u8>>().into_boxed_slice()
    }

    pub fn field<T: AsField>(&self, index: usize) -> Option<&T> {
        let field_info = self.ty.layout.get(index)?;
        let kind = T::kind();
        if field_info.kind == kind {
            let [start, end] = field_info.range();
            let data = &self.data[start..end];
            debug_assert_eq!(
                data.len(),
                field_info.kind.size() as usize,
                "data slice must be the same size as the underlying field type"
            );

            Some(T::from_slice(data))
        } else {
            None
        }
    }

    pub fn field_mut<T: AsField>(&mut self, index: usize) -> Option<&mut T> {
        let field_info = self.ty.layout.get(index)?;
        let kind = T::kind();
        if field_info.kind == kind {
            let [start, end] = field_info.range();
            let data = &mut self.data[start..end];
            debug_assert_eq!(
                data.len(),
                field_info.kind.size() as usize,
                "data slice must be the same size as the underlying field type"
            );

            Some(T::from_slice_mut(data))
        } else {
            None
        }
    }

    /// TODO: Errors
    pub fn set_field<T: AsField>(&mut self, field_index: usize, value: T) {
        let field: &mut T = self.field_mut(field_index).expect("invalid field");
        *field = value;
    }
}

impl fmt::Display for Obj {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let name = self.ty.name.as_str();
        let ptr = self.data.as_ptr();
        write!(f, "{name}<{ptr:?}>")
    }
}

// ----------------------------------------------------------------------------
// Construction and Descriptors

pub struct ObjBuilder {
    name: Option<String>,
    scheme: Option<LayoutScheme>,
    fields: Vec<FieldDesc>,
}

impl ObjBuilder {
    #[must_use]
    pub fn new() -> Self {
        Self {
            name: None,
            scheme: None,
            fields: vec![],
        }
    }

    pub fn with_name(mut self, name: &str) -> Self {
        self.name = Some(name.to_string());
        self
    }

    pub fn with_field(mut self, name: &str, kind: FieldKind) -> Self {
        let name = name.to_string();
        self.fields.push(FieldDesc::Value { name, kind });
        self
    }

    pub fn with_nested(mut self, name: &str, obj: Rc<ObjInfo>) -> Self {
        let name = name.to_string();
        self.fields.push(FieldDesc::Nested { name, obj });
        self
    }

    pub fn build(self) -> ObjInfo {
        let Self {
            name, scheme, fields, ..
        } = self;

        let name = name.unwrap_or_else(|| ObjDesc::DEFAULT_NAME.to_string());
        let scheme = scheme.unwrap_or(ObjDesc::DEFAULT_SCHEME);

        let kinds = fields
            .iter()
            .map(|f| {
                if let FieldDesc::Value { kind, .. } = f {
                    *kind
                } else {
                    todo!("nested objects")
                }
            })
            .collect::<Vec<_>>();
        let (layout, size) = ObjInfo::build_layout(scheme, &kinds);
        ObjInfo {
            layout,
            name,
            size,
            scheme,
        }
    }
}

/// Object type descriptor, for use when constructing a new type.
pub struct ObjDesc {
    /// Identifier for the type.
    pub name: Option<String>,
    /// Fully qualified name that disambiguates the type name.
    /// This could be any nesting, or namespacing, like a module path.
    ///
    /// If `None`, the [`ObjDesc::name`] field is used as the fully qualified name.
    pub qual_name: Option<String>,
    pub scheme: LayoutScheme,
    pub fields: Vec<FieldDesc>,
}

/// Field descriptor
pub enum FieldDesc {
    Value { name: String, kind: FieldKind },
    Nested { name: String, obj: Rc<ObjInfo> },
}

impl ObjDesc {
    pub const DEFAULT_NAME: &str = "Unknown";
    pub const DEFAULT_SCHEME: LayoutScheme = LayoutScheme::C99;
}

// ----------------------------------------------------------------------------
// Type Info

/// Type information for object.
pub struct ObjInfo {
    layout: Vec<FieldInfo>,
    name: String,
    /// Size of a value of this type in number of bytes.
    size: usize,
    scheme: LayoutScheme,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LayoutScheme {
    /// Reorder fields to use less space
    Reorder,
    /// Compatible layout with C structs
    C99,
}

#[derive(Debug)]
pub struct FieldInfo {
    offset: u16,
    kind: FieldKind,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum FieldKind {
    U8,
    U16,
    U32,
    U64,
    USize,
    I8,
    I16,
    I32,
    I64,
    ISize,
    F32,
    F64,
    // TODO: ObjInfo (Ptr?) field
}

impl ObjInfo {
    pub fn new(scheme: LayoutScheme, fields: &[FieldKind]) -> Self {
        let name = ObjDesc::DEFAULT_NAME.to_string();
        let (layout, size) = Self::build_layout(scheme, fields);
        Self {
            layout,
            name,
            size,
            scheme,
        }
    }

    fn build_layout(scheme: LayoutScheme, fields: &[FieldKind]) -> (Vec<FieldInfo>, usize) {
        // Descriptors need to be copied, because the layout can
        // optionally be reordered.
        let mut fields = fields.iter().copied().collect::<Vec<_>>();
        if scheme == LayoutScheme::Reorder {
            Self::reorder_layout(&mut fields);
        }

        let mut layout = vec![];
        let mut offset: usize = 0;
        let mut max_align: usize = 0;

        for kind in fields.iter().copied() {
            // Align the offset of this field.
            let align = kind.align();
            let padding = (align - (offset % align)) % align;
            let aligned_offset = offset + padding;

            // TODO: Graceful error when fields are < u16
            debug_assert!((aligned_offset as u16) < u16::MAX, "layout must fit in u16");

            layout.push(FieldInfo {
                offset: aligned_offset as u16,
                kind,
            });

            // Possibly unaligned offset where next field starts.
            offset = aligned_offset + kind.size();

            // Track the largest alignment for determing padding trail later.
            max_align = max_align.max(align);
        }

        // The total size of the object must be a multiple of the largest
        // alignment of any structure member.
        let size = offset + ((max_align - (offset % max_align)) % max_align);

        (layout, size)
    }

    /// Reorder the fields to optimize for space.
    fn reorder_layout(layout: &mut [FieldKind]) {
        // Order largest to smallest.
        layout.sort_by_key(|kind| Reverse(kind.size()));
    }

    /// Size of a value of this type in number of bytes.
    pub fn size(&self) -> usize {
        self.size
    }

    pub fn layout_scheme(&self) -> LayoutScheme {
        self.scheme
    }
}

impl FieldInfo {
    /// Start and end indices of the field in the object's data.
    pub fn range(&self) -> [usize; 2] {
        [self.offset as usize, self.offset as usize + self.kind.size() as usize]
    }
}

impl FieldKind {
    #[rustfmt::skip]
    pub fn size(&self) -> usize {
        match self {
            Self::U8    => std::mem::size_of::<u8>(),
            Self::U16   => std::mem::size_of::<u16>(),
            Self::U32   => std::mem::size_of::<u32>(),
            Self::U64   => std::mem::size_of::<u64>(),
            Self::USize => std::mem::size_of::<usize>(),
            Self::I8    => std::mem::size_of::<i8>(),
            Self::I16   => std::mem::size_of::<i16>(),
            Self::I32   => std::mem::size_of::<i32>(),
            Self::I64   => std::mem::size_of::<i64>(),
            Self::ISize => std::mem::size_of::<isize>(),
            Self::F32   => std::mem::size_of::<f32>(),
            Self::F64   => std::mem::size_of::<f64>(),
        }
    }

    #[rustfmt::skip]
    pub fn align(&self) -> usize {
        match self {
            Self::U8    => std::mem::align_of::<u8>(),
            Self::U16   => std::mem::align_of::<u16>(),
            Self::U32   => std::mem::align_of::<u32>(),
            Self::U64   => std::mem::align_of::<u64>(),
            Self::USize => std::mem::align_of::<usize>(),
            Self::I8    => std::mem::align_of::<i8>(),
            Self::I16   => std::mem::align_of::<i16>(),
            Self::I32   => std::mem::align_of::<i32>(),
            Self::I64   => std::mem::align_of::<i64>(),
            Self::ISize => std::mem::align_of::<isize>(),
            Self::F32   => std::mem::align_of::<f32>(),
            Self::F64   => std::mem::align_of::<f64>(),
        }
    }

    pub fn layout(&self) -> Layout {
        // Error is unreachable because alignments are hardcoded and should be power-of-two.
        Layout::from_size_align(self.size(), self.align()).expect("invalid size and alignment")
    }
}

// ----------------------------------------------------------------------------

// TODO: macro that implements AsField for all supported field types.
pub trait AsField {
    fn kind() -> FieldKind;
    fn from_slice(data: &[u8]) -> &Self;
    fn from_slice_mut(data: &mut [u8]) -> &mut Self;
}

macro_rules! impl_as_field {
    ($tt:ty, $field:ident) => {
        impl AsField for $tt {
            #[inline]
            fn kind() -> FieldKind {
                FieldKind::$field
            }

            #[inline]
            fn from_slice(data: &[u8]) -> &Self {
                &bytemuck::cast_slice::<u8, Self>(data)[0]
            }

            #[inline]
            fn from_slice_mut(data: &mut [u8]) -> &mut Self {
                &mut bytemuck::cast_slice_mut::<u8, Self>(data)[0]
            }
        }
    };
}

impl_as_field!(u8, U8);
impl_as_field!(u16, U16);
impl_as_field!(u32, U32);
impl_as_field!(u64, U64);
impl_as_field!(usize, USize);
impl_as_field!(i8, I8);
impl_as_field!(i16, I16);
impl_as_field!(i32, I32);
impl_as_field!(i64, I64);
impl_as_field!(isize, ISize);
impl_as_field!(f32, F32);
impl_as_field!(f64, F64);

// ----------------------------------------------------------------------------
// Unit Tests

#[cfg(test)]
mod test {
    use super::*;

    // #[repr(C)]
    // struct Foobar {
    //     field1: u8,
    //     field2: u16,
    //     field3: u8,
    //     field4: u32,
    // }

    #[cfg(any(target_arch = "x86_64", target_arch = "powerpc64", target_arch = "aarch64",))]
    const EXPECTED: &[u16] = &[0, 2, 4, 8];

    #[test]
    fn test_build_layout() {
        let (layout, size) = ObjInfo::build_layout(
            LayoutScheme::C99,
            &[FieldKind::U8, FieldKind::U16, FieldKind::U8, FieldKind::U32],
        );

        assert_eq!(layout[0].offset, EXPECTED[0], "first field must be at start of object");
        assert_eq!(layout[1].offset, EXPECTED[1], "second field must be padded");
        assert_eq!(layout[2].offset, EXPECTED[2], "third field must not be padded");
        assert_eq!(layout[3].offset, EXPECTED[3], "third field must be padded");
        assert_eq!(size, 12);
    }

    #[test]
    fn test_reorder_layout() {
        let obj_info = Rc::new(ObjInfo::new(
            LayoutScheme::Reorder,
            &[FieldKind::U8, FieldKind::U16, FieldKind::U8, FieldKind::U32],
        ));

        assert_eq!(obj_info.size, 8);

        let mut obj = Obj::new(obj_info);
        // FIXME: Field indices must not change when reordered
        obj.set_field::<u32>(0, 1234);
        assert_eq!(obj.field::<u32>(0), Some(&1234));
    }

    #[test]
    fn test_field_access() {
        let ty = Rc::new(ObjInfo::new(
            LayoutScheme::C99,
            &[FieldKind::U8, FieldKind::U16, FieldKind::U8, FieldKind::U32],
        ));

        let mut obj = Obj::new(ty);

        let field1: &u16 = obj.field(1).unwrap();
        assert_eq!(*field1, 0);

        obj.set_field(0, 7_u8);
        obj.set_field(1, 42_u16);
        obj.set_field(2, 11_u8);
        obj.set_field(3, 1234_u32);

        assert_eq!(obj.field::<u8>(0), Some(&7));
        assert_eq!(obj.field::<u16>(1), Some(&42));
        assert_eq!(obj.field::<u8>(2), Some(&11));
        assert_eq!(obj.field::<u32>(3), Some(&1234));

        assert_eq!(
            obj.field::<u16>(5),
            None,
            "out-of-bounds access must gracefully return `None`"
        );
    }

    #[test]
    fn test_builder() {
        let obj_info = ObjBuilder::new()
            .with_name("Message")
            .with_field("one", FieldKind::U8)
            .with_field("two", FieldKind::U16)
            .with_field("three", FieldKind::U8)
            .with_field("four", FieldKind::U32)
            .build();

        let obj_info = Rc::new(obj_info);

        let obj = Obj::new(obj_info);
        println!("{obj}");
    }
}
