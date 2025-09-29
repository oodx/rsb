//! Sanity tests for Object module

#[cfg(feature = "object")]
mod object_sanity {
    use rsb::object::*;

    #[test]
    fn sanity_object_creation() {
        let obj = Object::<()>::new("test");
        assert_eq!(obj.namespace(), "test");
    }

    #[test]
    fn sanity_object_set_get() {
        let mut obj = Object::<()>::new("test");
        obj.set("key", "value");
        assert_eq!(obj.get("key"), "value");
    }

    #[test]
    fn sanity_object_bracket_notation() {
        let mut obj = Object::<()>::new("test");
        obj.set("key", "value");
        assert_eq!(&obj["key"], "value");
    }

    #[test]
    fn sanity_object_has_key() {
        let mut obj = Object::<()>::new("test");
        obj.set("exists", "yes");
        assert!(obj.has("exists"));
        assert!(!obj.has("missing"));
    }

    #[test]
    fn sanity_object_get_or_default() {
        let obj = Object::<()>::new("test");
        assert_eq!(obj.get_or("missing", "default"), "default");
    }

    #[test]
    fn sanity_object_key_normalization() {
        let mut obj = Object::<()>::new("test");
        obj.set("dot.notation", "value1");
        obj.set("kebab-case", "value2");
        obj.set("CamelCase", "value3");

        // Keys are normalized to snake_case
        assert_eq!(obj.get("dot_notation"), "value1");
        assert_eq!(obj.get("kebab_case"), "value2");
        assert_eq!(obj.get("camel_case"), "value3");  // CamelCase → camel_case

        // Can access with original key format too (gets normalized)
        assert_eq!(obj.get("CamelCase"), "value3");
    }

    #[test]
    fn sanity_object_type_aliases() {
        let hub: HubConfig = Object::new("hub");
        assert_eq!(hub.namespace(), "hub");

        let inf: InfConfig = Object::new("inf");
        assert_eq!(inf.namespace(), "inf");

        let rsb: RsbConfig = Object::new("rsb");
        assert_eq!(rsb.namespace(), "rsb");
    }

    #[test]
    fn sanity_object_type_conversion() {
        let mut generic: Object = Object::new("test");
        generic.set("data", "value");

        let typed: Object<HubShape> = generic.as_type();
        assert_eq!(typed.get("data"), "value");
    }

    #[test]
    fn sanity_object_helper_functions() {
        let hub = get_hub();
        assert_eq!(hub.namespace(), "hub");

        let inf = get_inf();
        assert_eq!(inf.namespace(), "inf");

        let rsb = get_rsb();
        assert_eq!(rsb.namespace(), "rsb");
    }

    #[test]
    fn sanity_object_keys_listing() {
        let mut obj = Object::<()>::new("test");
        obj.set("key1", "value1");
        obj.set("key2", "value2");

        let keys = obj.keys();
        assert_eq!(keys.len(), 2);
    }
}