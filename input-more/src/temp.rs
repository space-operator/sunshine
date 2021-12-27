mod global_state {
    use crate::{
        define_markers, define_struct_take_and_with_field, State, StructTakeField, StructWithField,
    };
    pub struct GlobalState<Mo, TsKe, TsMo, ShKeLo, ShKeCl, ShMoLo, ShMoCl> {
        pub modifiers: Mo,
        pub keyboard_timed_state: TsKe,
        pub mouse_timed_state: TsMo,
        pub keyboard_long_press_scheduler: ShKeLo,
        pub keyboard_click_exact_scheduler: ShKeCl,
        pub mouse_long_press_scheduler: ShMoLo,
        pub mouse_click_exact_scheduler: ShMoCl,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl<
            Mo: ::core::clone::Clone,
            TsKe: ::core::clone::Clone,
            TsMo: ::core::clone::Clone,
            ShKeLo: ::core::clone::Clone,
            ShKeCl: ::core::clone::Clone,
            ShMoLo: ::core::clone::Clone,
            ShMoCl: ::core::clone::Clone,
        > ::core::clone::Clone for GlobalState<Mo, TsKe, TsMo, ShKeLo, ShKeCl, ShMoLo, ShMoCl>
    {
        #[inline]
        fn clone(&self) -> GlobalState<Mo, TsKe, TsMo, ShKeLo, ShKeCl, ShMoLo, ShMoCl> {
            match *self {
                GlobalState {
                    modifiers: ref __self_0_0,
                    keyboard_timed_state: ref __self_0_1,
                    mouse_timed_state: ref __self_0_2,
                    keyboard_long_press_scheduler: ref __self_0_3,
                    keyboard_click_exact_scheduler: ref __self_0_4,
                    mouse_long_press_scheduler: ref __self_0_5,
                    mouse_click_exact_scheduler: ref __self_0_6,
                } => GlobalState {
                    modifiers: ::core::clone::Clone::clone(&(*__self_0_0)),
                    keyboard_timed_state: ::core::clone::Clone::clone(&(*__self_0_1)),
                    mouse_timed_state: ::core::clone::Clone::clone(&(*__self_0_2)),
                    keyboard_long_press_scheduler: ::core::clone::Clone::clone(&(*__self_0_3)),
                    keyboard_click_exact_scheduler: ::core::clone::Clone::clone(&(*__self_0_4)),
                    mouse_long_press_scheduler: ::core::clone::Clone::clone(&(*__self_0_5)),
                    mouse_click_exact_scheduler: ::core::clone::Clone::clone(&(*__self_0_6)),
                },
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl<
            Mo: ::core::fmt::Debug,
            TsKe: ::core::fmt::Debug,
            TsMo: ::core::fmt::Debug,
            ShKeLo: ::core::fmt::Debug,
            ShKeCl: ::core::fmt::Debug,
            ShMoLo: ::core::fmt::Debug,
            ShMoCl: ::core::fmt::Debug,
        > ::core::fmt::Debug for GlobalState<Mo, TsKe, TsMo, ShKeLo, ShKeCl, ShMoLo, ShMoCl>
    {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match *self {
                GlobalState {
                    modifiers: ref __self_0_0,
                    keyboard_timed_state: ref __self_0_1,
                    mouse_timed_state: ref __self_0_2,
                    keyboard_long_press_scheduler: ref __self_0_3,
                    keyboard_click_exact_scheduler: ref __self_0_4,
                    mouse_long_press_scheduler: ref __self_0_5,
                    mouse_click_exact_scheduler: ref __self_0_6,
                } => {
                    let debug_trait_builder =
                        &mut ::core::fmt::Formatter::debug_struct(f, "GlobalState");
                    let _ = ::core::fmt::DebugStruct::field(
                        debug_trait_builder,
                        "modifiers",
                        &&(*__self_0_0),
                    );
                    let _ = ::core::fmt::DebugStruct::field(
                        debug_trait_builder,
                        "keyboard_timed_state",
                        &&(*__self_0_1),
                    );
                    let _ = ::core::fmt::DebugStruct::field(
                        debug_trait_builder,
                        "mouse_timed_state",
                        &&(*__self_0_2),
                    );
                    let _ = ::core::fmt::DebugStruct::field(
                        debug_trait_builder,
                        "keyboard_long_press_scheduler",
                        &&(*__self_0_3),
                    );
                    let _ = ::core::fmt::DebugStruct::field(
                        debug_trait_builder,
                        "keyboard_click_exact_scheduler",
                        &&(*__self_0_4),
                    );
                    let _ = ::core::fmt::DebugStruct::field(
                        debug_trait_builder,
                        "mouse_long_press_scheduler",
                        &&(*__self_0_5),
                    );
                    let _ = ::core::fmt::DebugStruct::field(
                        debug_trait_builder,
                        "mouse_click_exact_scheduler",
                        &&(*__self_0_6),
                    );
                    ::core::fmt::DebugStruct::finish(debug_trait_builder)
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl<
            Mo: ::core::default::Default,
            TsKe: ::core::default::Default,
            TsMo: ::core::default::Default,
            ShKeLo: ::core::default::Default,
            ShKeCl: ::core::default::Default,
            ShMoLo: ::core::default::Default,
            ShMoCl: ::core::default::Default,
        > ::core::default::Default for GlobalState<Mo, TsKe, TsMo, ShKeLo, ShKeCl, ShMoLo, ShMoCl>
    {
        #[inline]
        fn default() -> GlobalState<Mo, TsKe, TsMo, ShKeLo, ShKeCl, ShMoLo, ShMoCl> {
            GlobalState {
                modifiers: ::core::default::Default::default(),
                keyboard_timed_state: ::core::default::Default::default(),
                mouse_timed_state: ::core::default::Default::default(),
                keyboard_long_press_scheduler: ::core::default::Default::default(),
                keyboard_click_exact_scheduler: ::core::default::Default::default(),
                mouse_long_press_scheduler: ::core::default::Default::default(),
                mouse_click_exact_scheduler: ::core::default::Default::default(),
            }
        }
    }
    pub struct GlobalModifiersMarker;
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for GlobalModifiersMarker {
        #[inline]
        fn clone(&self) -> GlobalModifiersMarker {
            {
                *self
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::marker::Copy for GlobalModifiersMarker {}
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::fmt::Debug for GlobalModifiersMarker {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match *self {
                GlobalModifiersMarker => {
                    ::core::fmt::Formatter::write_str(f, "GlobalModifiersMarker")
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::default::Default for GlobalModifiersMarker {
        #[inline]
        fn default() -> GlobalModifiersMarker {
            GlobalModifiersMarker {}
        }
    }
    impl ::core::marker::StructuralEq for GlobalModifiersMarker {}
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::cmp::Eq for GlobalModifiersMarker {
        #[inline]
        #[doc(hidden)]
        #[no_coverage]
        fn assert_receiver_is_total_eq(&self) -> () {
            {}
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::hash::Hash for GlobalModifiersMarker {
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            match *self {
                GlobalModifiersMarker => {}
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::cmp::Ord for GlobalModifiersMarker {
        #[inline]
        fn cmp(&self, other: &GlobalModifiersMarker) -> ::core::cmp::Ordering {
            match *other {
                GlobalModifiersMarker => match *self {
                    GlobalModifiersMarker => ::core::cmp::Ordering::Equal,
                },
            }
        }
    }
    impl ::core::marker::StructuralPartialEq for GlobalModifiersMarker {}
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::cmp::PartialEq for GlobalModifiersMarker {
        #[inline]
        fn eq(&self, other: &GlobalModifiersMarker) -> bool {
            match *other {
                GlobalModifiersMarker => match *self {
                    GlobalModifiersMarker => true,
                },
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::cmp::PartialOrd for GlobalModifiersMarker {
        #[inline]
        fn partial_cmp(
            &self,
            other: &GlobalModifiersMarker,
        ) -> ::core::option::Option<::core::cmp::Ordering> {
            match *other {
                GlobalModifiersMarker => match *self {
                    GlobalModifiersMarker => {
                        ::core::option::Option::Some(::core::cmp::Ordering::Equal)
                    }
                },
            }
        }
    }
    pub struct KeyboardTimedStateMarker;
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for KeyboardTimedStateMarker {
        #[inline]
        fn clone(&self) -> KeyboardTimedStateMarker {
            {
                *self
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::marker::Copy for KeyboardTimedStateMarker {}
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::fmt::Debug for KeyboardTimedStateMarker {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match *self {
                KeyboardTimedStateMarker => {
                    ::core::fmt::Formatter::write_str(f, "KeyboardTimedStateMarker")
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::default::Default for KeyboardTimedStateMarker {
        #[inline]
        fn default() -> KeyboardTimedStateMarker {
            KeyboardTimedStateMarker {}
        }
    }
    impl ::core::marker::StructuralEq for KeyboardTimedStateMarker {}
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::cmp::Eq for KeyboardTimedStateMarker {
        #[inline]
        #[doc(hidden)]
        #[no_coverage]
        fn assert_receiver_is_total_eq(&self) -> () {
            {}
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::hash::Hash for KeyboardTimedStateMarker {
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            match *self {
                KeyboardTimedStateMarker => {}
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::cmp::Ord for KeyboardTimedStateMarker {
        #[inline]
        fn cmp(&self, other: &KeyboardTimedStateMarker) -> ::core::cmp::Ordering {
            match *other {
                KeyboardTimedStateMarker => match *self {
                    KeyboardTimedStateMarker => ::core::cmp::Ordering::Equal,
                },
            }
        }
    }
    impl ::core::marker::StructuralPartialEq for KeyboardTimedStateMarker {}
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::cmp::PartialEq for KeyboardTimedStateMarker {
        #[inline]
        fn eq(&self, other: &KeyboardTimedStateMarker) -> bool {
            match *other {
                KeyboardTimedStateMarker => match *self {
                    KeyboardTimedStateMarker => true,
                },
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::cmp::PartialOrd for KeyboardTimedStateMarker {
        #[inline]
        fn partial_cmp(
            &self,
            other: &KeyboardTimedStateMarker,
        ) -> ::core::option::Option<::core::cmp::Ordering> {
            match *other {
                KeyboardTimedStateMarker => match *self {
                    KeyboardTimedStateMarker => {
                        ::core::option::Option::Some(::core::cmp::Ordering::Equal)
                    }
                },
            }
        }
    }
    pub struct MouseTimedStateMarker;
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for MouseTimedStateMarker {
        #[inline]
        fn clone(&self) -> MouseTimedStateMarker {
            {
                *self
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::marker::Copy for MouseTimedStateMarker {}
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::fmt::Debug for MouseTimedStateMarker {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match *self {
                MouseTimedStateMarker => {
                    ::core::fmt::Formatter::write_str(f, "MouseTimedStateMarker")
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::default::Default for MouseTimedStateMarker {
        #[inline]
        fn default() -> MouseTimedStateMarker {
            MouseTimedStateMarker {}
        }
    }
    impl ::core::marker::StructuralEq for MouseTimedStateMarker {}
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::cmp::Eq for MouseTimedStateMarker {
        #[inline]
        #[doc(hidden)]
        #[no_coverage]
        fn assert_receiver_is_total_eq(&self) -> () {
            {}
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::hash::Hash for MouseTimedStateMarker {
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            match *self {
                MouseTimedStateMarker => {}
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::cmp::Ord for MouseTimedStateMarker {
        #[inline]
        fn cmp(&self, other: &MouseTimedStateMarker) -> ::core::cmp::Ordering {
            match *other {
                MouseTimedStateMarker => match *self {
                    MouseTimedStateMarker => ::core::cmp::Ordering::Equal,
                },
            }
        }
    }
    impl ::core::marker::StructuralPartialEq for MouseTimedStateMarker {}
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::cmp::PartialEq for MouseTimedStateMarker {
        #[inline]
        fn eq(&self, other: &MouseTimedStateMarker) -> bool {
            match *other {
                MouseTimedStateMarker => match *self {
                    MouseTimedStateMarker => true,
                },
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::cmp::PartialOrd for MouseTimedStateMarker {
        #[inline]
        fn partial_cmp(
            &self,
            other: &MouseTimedStateMarker,
        ) -> ::core::option::Option<::core::cmp::Ordering> {
            match *other {
                MouseTimedStateMarker => match *self {
                    MouseTimedStateMarker => {
                        ::core::option::Option::Some(::core::cmp::Ordering::Equal)
                    }
                },
            }
        }
    }
    pub struct KeyboardLongPressSchedulerMarker;
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for KeyboardLongPressSchedulerMarker {
        #[inline]
        fn clone(&self) -> KeyboardLongPressSchedulerMarker {
            {
                *self
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::marker::Copy for KeyboardLongPressSchedulerMarker {}
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::fmt::Debug for KeyboardLongPressSchedulerMarker {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match *self {
                KeyboardLongPressSchedulerMarker => {
                    ::core::fmt::Formatter::write_str(f, "KeyboardLongPressSchedulerMarker")
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::default::Default for KeyboardLongPressSchedulerMarker {
        #[inline]
        fn default() -> KeyboardLongPressSchedulerMarker {
            KeyboardLongPressSchedulerMarker {}
        }
    }
    impl ::core::marker::StructuralEq for KeyboardLongPressSchedulerMarker {}
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::cmp::Eq for KeyboardLongPressSchedulerMarker {
        #[inline]
        #[doc(hidden)]
        #[no_coverage]
        fn assert_receiver_is_total_eq(&self) -> () {
            {}
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::hash::Hash for KeyboardLongPressSchedulerMarker {
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            match *self {
                KeyboardLongPressSchedulerMarker => {}
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::cmp::Ord for KeyboardLongPressSchedulerMarker {
        #[inline]
        fn cmp(&self, other: &KeyboardLongPressSchedulerMarker) -> ::core::cmp::Ordering {
            match *other {
                KeyboardLongPressSchedulerMarker => match *self {
                    KeyboardLongPressSchedulerMarker => ::core::cmp::Ordering::Equal,
                },
            }
        }
    }
    impl ::core::marker::StructuralPartialEq for KeyboardLongPressSchedulerMarker {}
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::cmp::PartialEq for KeyboardLongPressSchedulerMarker {
        #[inline]
        fn eq(&self, other: &KeyboardLongPressSchedulerMarker) -> bool {
            match *other {
                KeyboardLongPressSchedulerMarker => match *self {
                    KeyboardLongPressSchedulerMarker => true,
                },
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::cmp::PartialOrd for KeyboardLongPressSchedulerMarker {
        #[inline]
        fn partial_cmp(
            &self,
            other: &KeyboardLongPressSchedulerMarker,
        ) -> ::core::option::Option<::core::cmp::Ordering> {
            match *other {
                KeyboardLongPressSchedulerMarker => match *self {
                    KeyboardLongPressSchedulerMarker => {
                        ::core::option::Option::Some(::core::cmp::Ordering::Equal)
                    }
                },
            }
        }
    }
    pub struct KeyboardClickExactSchedulerMarker;
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for KeyboardClickExactSchedulerMarker {
        #[inline]
        fn clone(&self) -> KeyboardClickExactSchedulerMarker {
            {
                *self
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::marker::Copy for KeyboardClickExactSchedulerMarker {}
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::fmt::Debug for KeyboardClickExactSchedulerMarker {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match *self {
                KeyboardClickExactSchedulerMarker => {
                    ::core::fmt::Formatter::write_str(f, "KeyboardClickExactSchedulerMarker")
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::default::Default for KeyboardClickExactSchedulerMarker {
        #[inline]
        fn default() -> KeyboardClickExactSchedulerMarker {
            KeyboardClickExactSchedulerMarker {}
        }
    }
    impl ::core::marker::StructuralEq for KeyboardClickExactSchedulerMarker {}
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::cmp::Eq for KeyboardClickExactSchedulerMarker {
        #[inline]
        #[doc(hidden)]
        #[no_coverage]
        fn assert_receiver_is_total_eq(&self) -> () {
            {}
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::hash::Hash for KeyboardClickExactSchedulerMarker {
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            match *self {
                KeyboardClickExactSchedulerMarker => {}
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::cmp::Ord for KeyboardClickExactSchedulerMarker {
        #[inline]
        fn cmp(&self, other: &KeyboardClickExactSchedulerMarker) -> ::core::cmp::Ordering {
            match *other {
                KeyboardClickExactSchedulerMarker => match *self {
                    KeyboardClickExactSchedulerMarker => ::core::cmp::Ordering::Equal,
                },
            }
        }
    }
    impl ::core::marker::StructuralPartialEq for KeyboardClickExactSchedulerMarker {}
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::cmp::PartialEq for KeyboardClickExactSchedulerMarker {
        #[inline]
        fn eq(&self, other: &KeyboardClickExactSchedulerMarker) -> bool {
            match *other {
                KeyboardClickExactSchedulerMarker => match *self {
                    KeyboardClickExactSchedulerMarker => true,
                },
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::cmp::PartialOrd for KeyboardClickExactSchedulerMarker {
        #[inline]
        fn partial_cmp(
            &self,
            other: &KeyboardClickExactSchedulerMarker,
        ) -> ::core::option::Option<::core::cmp::Ordering> {
            match *other {
                KeyboardClickExactSchedulerMarker => match *self {
                    KeyboardClickExactSchedulerMarker => {
                        ::core::option::Option::Some(::core::cmp::Ordering::Equal)
                    }
                },
            }
        }
    }
    pub struct MouseLongPressSchedulerMarker;
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for MouseLongPressSchedulerMarker {
        #[inline]
        fn clone(&self) -> MouseLongPressSchedulerMarker {
            {
                *self
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::marker::Copy for MouseLongPressSchedulerMarker {}
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::fmt::Debug for MouseLongPressSchedulerMarker {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match *self {
                MouseLongPressSchedulerMarker => {
                    ::core::fmt::Formatter::write_str(f, "MouseLongPressSchedulerMarker")
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::default::Default for MouseLongPressSchedulerMarker {
        #[inline]
        fn default() -> MouseLongPressSchedulerMarker {
            MouseLongPressSchedulerMarker {}
        }
    }
    impl ::core::marker::StructuralEq for MouseLongPressSchedulerMarker {}
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::cmp::Eq for MouseLongPressSchedulerMarker {
        #[inline]
        #[doc(hidden)]
        #[no_coverage]
        fn assert_receiver_is_total_eq(&self) -> () {
            {}
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::hash::Hash for MouseLongPressSchedulerMarker {
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            match *self {
                MouseLongPressSchedulerMarker => {}
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::cmp::Ord for MouseLongPressSchedulerMarker {
        #[inline]
        fn cmp(&self, other: &MouseLongPressSchedulerMarker) -> ::core::cmp::Ordering {
            match *other {
                MouseLongPressSchedulerMarker => match *self {
                    MouseLongPressSchedulerMarker => ::core::cmp::Ordering::Equal,
                },
            }
        }
    }
    impl ::core::marker::StructuralPartialEq for MouseLongPressSchedulerMarker {}
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::cmp::PartialEq for MouseLongPressSchedulerMarker {
        #[inline]
        fn eq(&self, other: &MouseLongPressSchedulerMarker) -> bool {
            match *other {
                MouseLongPressSchedulerMarker => match *self {
                    MouseLongPressSchedulerMarker => true,
                },
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::cmp::PartialOrd for MouseLongPressSchedulerMarker {
        #[inline]
        fn partial_cmp(
            &self,
            other: &MouseLongPressSchedulerMarker,
        ) -> ::core::option::Option<::core::cmp::Ordering> {
            match *other {
                MouseLongPressSchedulerMarker => match *self {
                    MouseLongPressSchedulerMarker => {
                        ::core::option::Option::Some(::core::cmp::Ordering::Equal)
                    }
                },
            }
        }
    }
    pub struct MouseClickExactSchedulerMarker;
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for MouseClickExactSchedulerMarker {
        #[inline]
        fn clone(&self) -> MouseClickExactSchedulerMarker {
            {
                *self
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::marker::Copy for MouseClickExactSchedulerMarker {}
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::fmt::Debug for MouseClickExactSchedulerMarker {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match *self {
                MouseClickExactSchedulerMarker => {
                    ::core::fmt::Formatter::write_str(f, "MouseClickExactSchedulerMarker")
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::default::Default for MouseClickExactSchedulerMarker {
        #[inline]
        fn default() -> MouseClickExactSchedulerMarker {
            MouseClickExactSchedulerMarker {}
        }
    }
    impl ::core::marker::StructuralEq for MouseClickExactSchedulerMarker {}
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::cmp::Eq for MouseClickExactSchedulerMarker {
        #[inline]
        #[doc(hidden)]
        #[no_coverage]
        fn assert_receiver_is_total_eq(&self) -> () {
            {}
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::hash::Hash for MouseClickExactSchedulerMarker {
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            match *self {
                MouseClickExactSchedulerMarker => {}
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::cmp::Ord for MouseClickExactSchedulerMarker {
        #[inline]
        fn cmp(&self, other: &MouseClickExactSchedulerMarker) -> ::core::cmp::Ordering {
            match *other {
                MouseClickExactSchedulerMarker => match *self {
                    MouseClickExactSchedulerMarker => ::core::cmp::Ordering::Equal,
                },
            }
        }
    }
    impl ::core::marker::StructuralPartialEq for MouseClickExactSchedulerMarker {}
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::cmp::PartialEq for MouseClickExactSchedulerMarker {
        #[inline]
        fn eq(&self, other: &MouseClickExactSchedulerMarker) -> bool {
            match *other {
                MouseClickExactSchedulerMarker => match *self {
                    MouseClickExactSchedulerMarker => true,
                },
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::cmp::PartialOrd for MouseClickExactSchedulerMarker {
        #[inline]
        fn partial_cmp(
            &self,
            other: &MouseClickExactSchedulerMarker,
        ) -> ::core::option::Option<::core::cmp::Ordering> {
            match *other {
                MouseClickExactSchedulerMarker => match *self {
                    MouseClickExactSchedulerMarker => {
                        ::core::option::Option::Some(::core::cmp::Ordering::Equal)
                    }
                },
            }
        }
    }
    impl<Mo, TsKe, TsMo, ShKeLo, ShKeCl, ShMoLo, ShMoCl>
        crate::StructTakeField<Mo, GlobalModifiersMarker>
        for GlobalState<Mo, TsKe, TsMo, ShKeLo, ShKeCl, ShMoLo, ShMoCl>
    {
        type Rest = GlobalState<(), TsKe, TsMo, ShKeLo, ShKeCl, ShMoLo, ShMoCl>;
        fn take_field(self) -> (Mo, Self::Rest) {
            (
                self.modifiers,
                GlobalState {
                    modifiers: (),
                    keyboard_timed_state: self.keyboard_timed_state,
                    mouse_timed_state: self.mouse_timed_state,
                    keyboard_long_press_scheduler: self.keyboard_long_press_scheduler,
                    keyboard_click_exact_scheduler: self.keyboard_click_exact_scheduler,
                    mouse_long_press_scheduler: self.mouse_long_press_scheduler,
                    mouse_click_exact_scheduler: self.mouse_click_exact_scheduler,
                },
            )
        }
    }
    impl<Mo, TsKe, TsMo, ShKeLo, ShKeCl, ShMoLo, ShMoCl>
        crate::StructTakeField<TsKe, KeyboardTimedStateMarker>
        for GlobalState<Mo, TsKe, TsMo, ShKeLo, ShKeCl, ShMoLo, ShMoCl>
    {
        type Rest = GlobalState<Mo, (), TsMo, ShKeLo, ShKeCl, ShMoLo, ShMoCl>;
        fn take_field(self) -> (TsKe, Self::Rest) {
            (
                self.keyboard_timed_state,
                GlobalState {
                    modifiers: self.modifiers,
                    keyboard_timed_state: (),
                    mouse_timed_state: self.mouse_timed_state,
                    keyboard_long_press_scheduler: self.keyboard_long_press_scheduler,
                    keyboard_click_exact_scheduler: self.keyboard_click_exact_scheduler,
                    mouse_long_press_scheduler: self.mouse_long_press_scheduler,
                    mouse_click_exact_scheduler: self.mouse_click_exact_scheduler,
                },
            )
        }
    }
    impl<Mo, TsKe, TsMo, ShKeLo, ShKeCl, ShMoLo, ShMoCl>
        crate::StructTakeField<TsMo, MouseTimedStateMarker>
        for GlobalState<Mo, TsKe, TsMo, ShKeLo, ShKeCl, ShMoLo, ShMoCl>
    {
        type Rest = GlobalState<Mo, TsKe, (), ShKeLo, ShKeCl, ShMoLo, ShMoCl>;
        fn take_field(self) -> (TsMo, Self::Rest) {
            (
                self.mouse_timed_state,
                GlobalState {
                    modifiers: self.modifiers,
                    keyboard_timed_state: self.keyboard_timed_state,
                    mouse_timed_state: (),
                    keyboard_long_press_scheduler: self.keyboard_long_press_scheduler,
                    keyboard_click_exact_scheduler: self.keyboard_click_exact_scheduler,
                    mouse_long_press_scheduler: self.mouse_long_press_scheduler,
                    mouse_click_exact_scheduler: self.mouse_click_exact_scheduler,
                },
            )
        }
    }
    impl<Mo, TsKe, TsMo, ShKeLo, ShKeCl, ShMoLo, ShMoCl>
        crate::StructTakeField<ShKeLo, KeyboardLongPressSchedulerMarker>
        for GlobalState<Mo, TsKe, TsMo, ShKeLo, ShKeCl, ShMoLo, ShMoCl>
    {
        type Rest = GlobalState<Mo, TsKe, TsMo, (), ShKeCl, ShMoLo, ShMoCl>;
        fn take_field(self) -> (ShKeLo, Self::Rest) {
            (
                self.keyboard_long_press_scheduler,
                GlobalState {
                    modifiers: self.modifiers,
                    keyboard_timed_state: self.keyboard_timed_state,
                    mouse_timed_state: self.mouse_timed_state,
                    keyboard_long_press_scheduler: (),
                    keyboard_click_exact_scheduler: self.keyboard_click_exact_scheduler,
                    mouse_long_press_scheduler: self.mouse_long_press_scheduler,
                    mouse_click_exact_scheduler: self.mouse_click_exact_scheduler,
                },
            )
        }
    }
    impl<Mo, TsKe, TsMo, ShKeLo, ShKeCl, ShMoLo, ShMoCl>
        crate::StructTakeField<ShKeCl, KeyboardClickExactSchedulerMarker>
        for GlobalState<Mo, TsKe, TsMo, ShKeLo, ShKeCl, ShMoLo, ShMoCl>
    {
        type Rest = GlobalState<Mo, TsKe, TsMo, ShKeLo, (), ShMoLo, ShMoCl>;
        fn take_field(self) -> (ShKeCl, Self::Rest) {
            (
                self.keyboard_click_exact_scheduler,
                GlobalState {
                    modifiers: self.modifiers,
                    keyboard_timed_state: self.keyboard_timed_state,
                    mouse_timed_state: self.mouse_timed_state,
                    keyboard_long_press_scheduler: self.keyboard_long_press_scheduler,
                    keyboard_click_exact_scheduler: (),
                    mouse_long_press_scheduler: self.mouse_long_press_scheduler,
                    mouse_click_exact_scheduler: self.mouse_click_exact_scheduler,
                },
            )
        }
    }
    impl<Mo, TsKe, TsMo, ShKeLo, ShKeCl, ShMoLo, ShMoCl>
        crate::StructTakeField<ShMoLo, MouseLongPressSchedulerMarker>
        for GlobalState<Mo, TsKe, TsMo, ShKeLo, ShKeCl, ShMoLo, ShMoCl>
    {
        type Rest = GlobalState<Mo, TsKe, TsMo, ShKeLo, ShKeCl, (), ShMoCl>;
        fn take_field(self) -> (ShMoLo, Self::Rest) {
            (
                self.mouse_long_press_scheduler,
                GlobalState {
                    modifiers: self.modifiers,
                    keyboard_timed_state: self.keyboard_timed_state,
                    mouse_timed_state: self.mouse_timed_state,
                    keyboard_long_press_scheduler: self.keyboard_long_press_scheduler,
                    keyboard_click_exact_scheduler: self.keyboard_click_exact_scheduler,
                    mouse_long_press_scheduler: (),
                    mouse_click_exact_scheduler: self.mouse_click_exact_scheduler,
                },
            )
        }
    }
    impl<Mo, TsKe, TsMo, ShKeLo, ShKeCl, ShMoLo, ShMoCl>
        crate::StructTakeField<ShMoCl, MouseClickExactSchedulerMarker>
        for GlobalState<Mo, TsKe, TsMo, ShKeLo, ShKeCl, ShMoLo, ShMoCl>
    {
        type Rest = GlobalState<Mo, TsKe, TsMo, ShKeLo, ShKeCl, ShMoLo, ()>;
        fn take_field(self) -> (ShMoCl, Self::Rest) {
            (
                self.mouse_click_exact_scheduler,
                GlobalState {
                    modifiers: self.modifiers,
                    keyboard_timed_state: self.keyboard_timed_state,
                    mouse_timed_state: self.mouse_timed_state,
                    keyboard_long_press_scheduler: self.keyboard_long_press_scheduler,
                    keyboard_click_exact_scheduler: self.keyboard_click_exact_scheduler,
                    mouse_long_press_scheduler: self.mouse_long_press_scheduler,
                    mouse_click_exact_scheduler: (),
                },
            )
        }
    }
    impl<Mo, TsKe, TsMo, ShKeLo, ShKeCl, ShMoLo, ShMoCl>
        crate::StructWithField<Mo, GlobalModifiersMarker>
        for GlobalState<(), TsKe, TsMo, ShKeLo, ShKeCl, ShMoLo, ShMoCl>
    {
        type Output = GlobalState<Mo, TsKe, TsMo, ShKeLo, ShKeCl, ShMoLo, ShMoCl>;
        fn with_field(self, value: Mo) -> Self::Output {
            GlobalState {
                modifiers: value,
                keyboard_timed_state: self.keyboard_timed_state,
                mouse_timed_state: self.mouse_timed_state,
                keyboard_long_press_scheduler: self.keyboard_long_press_scheduler,
                keyboard_click_exact_scheduler: self.keyboard_click_exact_scheduler,
                mouse_long_press_scheduler: self.mouse_long_press_scheduler,
                mouse_click_exact_scheduler: self.mouse_click_exact_scheduler,
            }
        }
    }
    impl<Mo, TsKe, TsMo, ShKeLo, ShKeCl, ShMoLo, ShMoCl>
        crate::StructWithField<TsKe, KeyboardTimedStateMarker>
        for GlobalState<Mo, (), TsMo, ShKeLo, ShKeCl, ShMoLo, ShMoCl>
    {
        type Output = GlobalState<Mo, TsKe, TsMo, ShKeLo, ShKeCl, ShMoLo, ShMoCl>;
        fn with_field(self, value: TsKe) -> Self::Output {
            GlobalState {
                modifiers: self.modifiers,
                keyboard_timed_state: value,
                mouse_timed_state: self.mouse_timed_state,
                keyboard_long_press_scheduler: self.keyboard_long_press_scheduler,
                keyboard_click_exact_scheduler: self.keyboard_click_exact_scheduler,
                mouse_long_press_scheduler: self.mouse_long_press_scheduler,
                mouse_click_exact_scheduler: self.mouse_click_exact_scheduler,
            }
        }
    }
    impl<Mo, TsKe, TsMo, ShKeLo, ShKeCl, ShMoLo, ShMoCl>
        crate::StructWithField<TsMo, MouseTimedStateMarker>
        for GlobalState<Mo, TsKe, (), ShKeLo, ShKeCl, ShMoLo, ShMoCl>
    {
        type Output = GlobalState<Mo, TsKe, TsMo, ShKeLo, ShKeCl, ShMoLo, ShMoCl>;
        fn with_field(self, value: TsMo) -> Self::Output {
            GlobalState {
                modifiers: self.modifiers,
                keyboard_timed_state: self.keyboard_timed_state,
                mouse_timed_state: value,
                keyboard_long_press_scheduler: self.keyboard_long_press_scheduler,
                keyboard_click_exact_scheduler: self.keyboard_click_exact_scheduler,
                mouse_long_press_scheduler: self.mouse_long_press_scheduler,
                mouse_click_exact_scheduler: self.mouse_click_exact_scheduler,
            }
        }
    }
    impl<Mo, TsKe, TsMo, ShKeLo, ShKeCl, ShMoLo, ShMoCl>
        crate::StructWithField<ShKeLo, KeyboardLongPressSchedulerMarker>
        for GlobalState<Mo, TsKe, TsMo, (), ShKeCl, ShMoLo, ShMoCl>
    {
        type Output = GlobalState<Mo, TsKe, TsMo, ShKeLo, ShKeCl, ShMoLo, ShMoCl>;
        fn with_field(self, value: ShKeLo) -> Self::Output {
            GlobalState {
                modifiers: self.modifiers,
                keyboard_timed_state: self.keyboard_timed_state,
                mouse_timed_state: self.mouse_timed_state,
                keyboard_long_press_scheduler: value,
                keyboard_click_exact_scheduler: self.keyboard_click_exact_scheduler,
                mouse_long_press_scheduler: self.mouse_long_press_scheduler,
                mouse_click_exact_scheduler: self.mouse_click_exact_scheduler,
            }
        }
    }
    impl<Mo, TsKe, TsMo, ShKeLo, ShKeCl, ShMoLo, ShMoCl>
        crate::StructWithField<ShKeCl, KeyboardClickExactSchedulerMarker>
        for GlobalState<Mo, TsKe, TsMo, ShKeLo, (), ShMoLo, ShMoCl>
    {
        type Output = GlobalState<Mo, TsKe, TsMo, ShKeLo, ShKeCl, ShMoLo, ShMoCl>;
        fn with_field(self, value: ShKeCl) -> Self::Output {
            GlobalState {
                modifiers: self.modifiers,
                keyboard_timed_state: self.keyboard_timed_state,
                mouse_timed_state: self.mouse_timed_state,
                keyboard_long_press_scheduler: self.keyboard_long_press_scheduler,
                keyboard_click_exact_scheduler: value,
                mouse_long_press_scheduler: self.mouse_long_press_scheduler,
                mouse_click_exact_scheduler: self.mouse_click_exact_scheduler,
            }
        }
    }
    impl<Mo, TsKe, TsMo, ShKeLo, ShKeCl, ShMoLo, ShMoCl>
        crate::StructWithField<ShMoLo, MouseLongPressSchedulerMarker>
        for GlobalState<Mo, TsKe, TsMo, ShKeLo, ShKeCl, (), ShMoCl>
    {
        type Output = GlobalState<Mo, TsKe, TsMo, ShKeLo, ShKeCl, ShMoLo, ShMoCl>;
        fn with_field(self, value: ShMoLo) -> Self::Output {
            GlobalState {
                modifiers: self.modifiers,
                keyboard_timed_state: self.keyboard_timed_state,
                mouse_timed_state: self.mouse_timed_state,
                keyboard_long_press_scheduler: self.keyboard_long_press_scheduler,
                keyboard_click_exact_scheduler: self.keyboard_click_exact_scheduler,
                mouse_long_press_scheduler: value,
                mouse_click_exact_scheduler: self.mouse_click_exact_scheduler,
            }
        }
    }
    impl<Mo, TsKe, TsMo, ShKeLo, ShKeCl, ShMoLo, ShMoCl>
        crate::StructWithField<ShMoCl, MouseClickExactSchedulerMarker>
        for GlobalState<Mo, TsKe, TsMo, ShKeLo, ShKeCl, ShMoLo, ()>
    {
        type Output = GlobalState<Mo, TsKe, TsMo, ShKeLo, ShKeCl, ShMoLo, ShMoCl>;
        fn with_field(self, value: ShMoCl) -> Self::Output {
            GlobalState {
                modifiers: self.modifiers,
                keyboard_timed_state: self.keyboard_timed_state,
                mouse_timed_state: self.mouse_timed_state,
                keyboard_long_press_scheduler: self.keyboard_long_press_scheduler,
                keyboard_click_exact_scheduler: self.keyboard_click_exact_scheduler,
                mouse_long_press_scheduler: self.mouse_long_press_scheduler,
                mouse_click_exact_scheduler: value,
            }
        }
    }
    impl<Mo, TsKe, TsMo, ShKeLo, ShKeCl, ShMoLo, ShMoCl>
        GlobalState<Mo, TsKe, TsMo, ShKeLo, ShKeCl, ShMoLo, ShMoCl>
    {
        pub fn new(
            modifiers: Mo,
            keyboard_timed_state: TsKe,
            mouse_timed_state: TsMo,
            keyboard_long_press_scheduler: ShKeLo,
            keyboard_click_exact_scheduler: ShKeCl,
            mouse_long_press_scheduler: ShMoLo,
            mouse_click_exact_scheduler: ShMoCl,
        ) -> Self {
            Self {
                modifiers,
                keyboard_timed_state,
                mouse_timed_state,
                keyboard_long_press_scheduler,
                keyboard_click_exact_scheduler,
                mouse_long_press_scheduler,
                mouse_click_exact_scheduler,
            }
        }
        pub fn take_state<Ts, Sh, Re1, Ma1, Re2, Ma2, Re3, Ma3>(self) -> (State<Mo, Ts, Sh>, Re3)
        where
            Self: StructTakeField<Mo, Ma1, Rest = Re1>,
            Re1: StructTakeField<Ts, Ma2, Rest = Re2>,
            Re2: StructTakeField<Sh, Ma3, Rest = Re3>,
        {
            let (modifiers, rest) = self.take_field();
            let (timed_state, rest) = rest.take_field();
            let (scheduler, rest) = rest.take_field();
            (State::new(modifiers, timed_state, scheduler), rest)
        }
    }
    impl<TsKe, TsMo, ShKeLo, ShKeCl, ShMoLo, ShMoCl>
        GlobalState<(), TsKe, TsMo, ShKeLo, ShKeCl, ShMoLo, ShMoCl>
    {
        pub fn with_state<Mo, Ts, Sh, Re1, Ma1, Re2, Ma2, Re3, Ma3>(
            self,
            state: State<Mo, Ts, Sh>,
        ) -> Re3
        where
            Self: StructWithField<Mo, Ma1, Output = Re1>,
            Re1: StructWithField<Ts, Ma2, Output = Re2>,
            Re2: StructWithField<Sh, Ma3, Output = Re3>,
        {
            self.with_field(state.modifiers)
                .with_field(state.timed_state)
                .with_field(state.scheduler)
        }
    }
}
