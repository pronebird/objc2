#![allow(unused_imports)]
use alloc::borrow::ToOwned;

use objc2::mutability::IsIdCloneable;
use objc2::rc::Id;
use objc2::Message;

use crate::Foundation::{self, NSCopying, NSMutableCopying};

#[cfg(feature = "NSArray")]
impl<T: Message + IsIdCloneable> ToOwned for Foundation::NSArray<T> {
    type Owned = Id<Self>;
    fn to_owned(&self) -> Self::Owned {
        self.copy()
    }
}

#[cfg(feature = "NSArray")]
impl<T: Message + IsIdCloneable> ToOwned for Foundation::NSMutableArray<T> {
    type Owned = Id<Self>;
    fn to_owned(&self) -> Self::Owned {
        self.mutableCopy()
    }
}

#[cfg(feature = "NSData")]
impl ToOwned for Foundation::NSData {
    type Owned = Id<Self>;
    fn to_owned(&self) -> Self::Owned {
        self.copy()
    }
}

#[cfg(feature = "NSData")]
impl ToOwned for Foundation::NSMutableData {
    type Owned = Id<Self>;
    fn to_owned(&self) -> Self::Owned {
        self.mutableCopy()
    }
}

#[cfg(feature = "NSException")]
impl ToOwned for Foundation::NSException {
    type Owned = Id<Self>;
    fn to_owned(&self) -> Self::Owned {
        self.copy()
    }
}

#[cfg(feature = "NSSet")]
impl<T: Message + IsIdCloneable> ToOwned for Foundation::NSSet<T> {
    type Owned = Id<Self>;
    fn to_owned(&self) -> Self::Owned {
        self.copy()
    }
}

#[cfg(feature = "NSSet")]
impl<T: Message + IsIdCloneable> ToOwned for Foundation::NSMutableSet<T> {
    type Owned = Id<Self>;
    fn to_owned(&self) -> Self::Owned {
        self.mutableCopy()
    }
}

#[cfg(feature = "NSString")]
impl ToOwned for Foundation::NSString {
    type Owned = Id<Self>;
    fn to_owned(&self) -> Self::Owned {
        self.copy()
    }
}

#[cfg(feature = "NSString")]
impl ToOwned for Foundation::NSMutableString {
    type Owned = Id<Self>;
    fn to_owned(&self) -> Self::Owned {
        self.mutableCopy()
    }
}

#[cfg(feature = "NSAttributedString")]
impl ToOwned for Foundation::NSAttributedString {
    type Owned = Id<Self>;
    fn to_owned(&self) -> Self::Owned {
        self.copy()
    }
}

#[cfg(feature = "NSAttributedString")]
impl ToOwned for Foundation::NSMutableAttributedString {
    type Owned = Id<Self>;
    fn to_owned(&self) -> Self::Owned {
        self.mutableCopy()
    }
}

#[cfg(feature = "NSUUID")]
impl ToOwned for Foundation::NSUUID {
    type Owned = Id<Self>;
    fn to_owned(&self) -> Self::Owned {
        self.copy()
    }
}

#[cfg(feature = "NSValue")]
impl ToOwned for Foundation::NSValue {
    type Owned = Id<Self>;
    fn to_owned(&self) -> Self::Owned {
        self.copy()
    }
}

#[cfg(feature = "NSValue")]
impl ToOwned for Foundation::NSNumber {
    type Owned = Id<Self>;
    fn to_owned(&self) -> Self::Owned {
        self.copy()
    }
}
