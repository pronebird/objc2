use std::kinds::marker::ContravariantLifetime;
use std::mem;
use std::ptr;

use runtime::Object;
use {class, Id, IdVector, IntoIdVector};
use super::{INSCopying, INSObject};

pub struct NSRange {
	pub location: uint,
	pub length: uint,
}

pub struct NSEnumerator<'a, T> {
	id: Id<Object>,
	marker: ContravariantLifetime<'a>,
}

impl<'a, T> NSEnumerator<'a, T> {
	unsafe fn from_ptr(ptr: *Object) -> NSEnumerator<'a, T> {
		NSEnumerator { id: Id::from_ptr(ptr), marker: ContravariantLifetime }
	}
}

impl<'a, T> Iterator<&'a T> for NSEnumerator<'a, T> {
	fn next(&mut self) -> Option<&'a T> {
		unsafe {
			let obj = msg_send![self.id nextObject] as *T;
			obj.to_option()
		}
	}
}

pub trait INSArray<T: INSObject> : INSObject {
	fn count(&self) -> uint {
		let result = unsafe {
			msg_send![self count]
		};
		result as uint
	}

	fn object_at<'a>(&'a self, index: uint) -> &'a T {
		unsafe {
			let result = msg_send![self objectAtIndex:index] as *T;
			&*result
		}
	}

	fn object_enumerator<'a>(&'a self) -> NSEnumerator<'a, T> {
		unsafe {
			let result = msg_send![self objectEnumerator];
			NSEnumerator::from_ptr(result)
		}
	}

	unsafe fn from_refs(refs: &[&T]) -> Id<Self> {
		let cls = class::<Self>();
		let obj = msg_send![cls alloc];
		let obj = msg_send![obj initWithObjects:refs.as_ptr() count:refs.len()];
		Id::from_retained_ptr(obj as *Self)
	}

	fn from_vec(vec: Vec<Id<T>>) -> Id<Self> {
		let refs = vec.as_refs_slice();
		unsafe {
			INSArray::from_refs(refs)
		}
	}

	fn objects_in_range<'a>(&'a self, start: uint, len: uint) -> Vec<&'a T> {
		let vec: Vec<*T> = Vec::from_elem(len, ptr::null());
		let range = NSRange { location: start, length: len };
		unsafe {
			msg_send![self getObjects:vec.as_ptr() range:range];
			mem::transmute(vec)
		}
	}

	fn to_vec<'a>(&'a self) -> Vec<&'a T> {
		self.objects_in_range(0, self.count())
	}

	fn into_vec(array: Id<Self>) -> Vec<Id<T>> {
		let vec = array.to_vec();
		unsafe {
			vec.into_id_vec()
		}
	}
}

object_struct!(NSArray<T>)

impl<T: INSObject> INSArray<T> for NSArray<T> { }

impl<T> INSCopying<NSArray<T>> for NSArray<T> { }

impl<T: INSObject> Collection for NSArray<T> {
	fn len(&self) -> uint {
		self.count()
	}
}

#[cfg(test)]
mod tests {
	use {Id};
	use foundation::{INSObject, NSObject};
	use super::{INSArray, NSArray};

	#[test]
	fn test_count() {
		let empty_array: Id<NSArray<NSObject>> = INSObject::new();
		assert!(empty_array.count() == 0);

		let vec: Vec<Id<NSObject>> = Vec::from_fn(4, |_| INSObject::new());
		let array: Id<NSArray<NSObject>> = INSArray::from_vec(vec);
		assert!(array.count() == 4);
	}

	#[test]
	fn test_object_at() {
		let vec: Vec<Id<NSObject>> = Vec::from_fn(4, |_| INSObject::new());
		let array: Id<NSArray<NSObject>> = INSArray::from_vec(vec);
		assert!(array.object_at(0) != array.object_at(3));
	}

	#[test]
	fn test_object_enumerator() {
		let vec: Vec<Id<NSObject>> = Vec::from_fn(4, |_| INSObject::new());
		let array: Id<NSArray<NSObject>> = INSArray::from_vec(vec);

		assert!(array.object_enumerator().count() == 4);
		assert!(array.object_enumerator()
		             .enumerate()
		             .all(|(i, obj)| obj == array.object_at(i)));
	}

	#[test]
	fn test_objects_in_range() {
		let vec: Vec<Id<NSObject>> = Vec::from_fn(4, |_| INSObject::new());
		let array: Id<NSArray<NSObject>> = INSArray::from_vec(vec);

		let middle_objs = array.objects_in_range(1, 2);
		assert!(middle_objs.len() == 2);
		assert!(*middle_objs.get(0) == array.object_at(1));
		assert!(*middle_objs.get(1) == array.object_at(2));

		let empty_objs = array.objects_in_range(1, 0);
		assert!(empty_objs.len() == 0);

		let all_objs = array.objects_in_range(0, 4);
		assert!(all_objs.len() == 4);
	}

	#[test]
	fn test_into_vec() {
		let vec: Vec<Id<NSObject>> = Vec::from_fn(4, |_| INSObject::new());
		let array: Id<NSArray<NSObject>> = INSArray::from_vec(vec);

		let vec = INSArray::into_vec(array);
		assert!(vec.len() == 4);
	}
}