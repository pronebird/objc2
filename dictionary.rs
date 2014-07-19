use std::cmp::min;

use {class, Id, IdVector};
use super::{INSArray, INSCopying, INSObject, NSArray};

pub trait INSDictionary<K: INSObject, V: INSObject> : INSObject {
	fn count(&self) -> uint {
		let result = unsafe {
			msg_send![self count]
		};
		result as uint
	}

	fn object_for<'a>(&'a self, key: &K) -> Option<&'a V> {
		unsafe {
			let obj = msg_send![self objectForKey:key.as_ptr()] as *V;
			obj.to_option()
		}
	}

	fn all_keys<'a>(&'a self) -> Vec<&'a K> {
		let keys = unsafe {
			&*(msg_send![self allKeys] as *NSArray<K>)
		};
		keys.to_vec()
	}

	fn all_values<'a>(&'a self) -> Vec<&'a V> {
		let vals = unsafe {
			&*(msg_send![self allValues] as *NSArray<V>)
		};
		vals.to_vec()
	}

	unsafe fn from_refs<T: INSCopying<K>>(keys: &[&T], vals: &[&V]) -> Id<Self> {
		let cls = class::<Self>();
		let count = min(keys.len(), vals.len());
		let obj = msg_send![cls alloc];
		let obj = msg_send![obj initWithObjects:vals.as_ptr()
		                                forKeys:keys.as_ptr()
		                                  count:count];
		Id::from_retained_ptr(obj as *Self)
	}

	fn from_keys_and_objects<T: INSCopying<K>>(keys: &[&T], vals: Vec<Id<V>>) -> Id<Self> {
		let vals_refs = vals.as_refs_slice();
		unsafe {
			INSDictionary::from_refs(keys, vals_refs)
		}
	}
}

object_struct!(NSDictionary<K, V>)

impl<K: INSObject, V: INSObject> INSDictionary<K, V> for NSDictionary<K, V> { }

impl<K: INSObject, V: INSObject> Collection for NSDictionary<K, V> {
	fn len(&self) -> uint {
		self.count()
	}
}

impl<K: INSObject, V: INSObject> Map<K, V> for NSDictionary<K, V> {
	fn find<'a>(&'a self, key: &K) -> Option<&'a V> {
		self.object_for(key)
	}
}

#[cfg(test)]
mod tests {
	use {Id};
	use foundation::{INSObject, INSString, NSObject, NSString};
	use super::{INSDictionary, NSDictionary};

	fn sample_dict(key: &str) -> Id<NSDictionary<NSString, NSObject>> {
		let string: Id<NSString> = INSString::from_str(key);
		let obj: Id<NSObject> = INSObject::new();
		INSDictionary::from_keys_and_objects(&[&*string], vec![obj])
	}

	#[test]
	fn test_count() {
		let dict = sample_dict("abcd");
		assert!(dict.count() == 1);
	}

	#[test]
	fn test_object_for() {
		let dict = sample_dict("abcd");

		let string: Id<NSString> = INSString::from_str("abcd");
		assert!(dict.object_for(&*string).is_some());

		let string: Id<NSString> = INSString::from_str("abcde");
		assert!(dict.object_for(&*string).is_none());
	}

	#[test]
	fn test_all_keys() {
		let dict = sample_dict("abcd");
		let keys = dict.all_keys();

		assert!(keys.len() == 1);
		assert!(keys.get(0).as_str() == "abcd");
	}

	#[test]
	fn test_all_values() {
		let dict = sample_dict("abcd");
		let vals = dict.all_values();

		assert!(vals.len() == 1);
	}
}