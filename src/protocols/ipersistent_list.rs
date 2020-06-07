use crate::value::{Value,ToValue};
use std::rc::Rc;
use crate::persistent_list_map;
use crate::protocol::ProtocolCastable;

define_protocol!(IPersistentMap,PersistentListMap);

impl persistent_list_map::IPersistentMap for IPersistentMap {
    fn get(&self, key: &Rc<Value>) -> Rc<Value> {
        match &*self.value {
            Value::PersistentListMap(plist_map) => {
                plist_map.get(key)
            },
            _ => panic!("Called Iterable iter on non-iterable"),
        }
    }
    fn assoc(&self, key: Rc<Value>, value: Rc<Value>) -> IPersistentMap {
        match &*self.value {
            Value::PersistentListMap(plist_map) => {
                plist_map.assoc(key,value).to_rc_value().as_protocol::<IPersistentMap>()
            },
            _ => panic!("Called Iterable iter on non-iterable"),
        }
    }
}
