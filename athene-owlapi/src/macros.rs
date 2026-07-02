// ------------------------------------------------------------------------------------------------
// Macros ❯ HasAnnotations
// ------------------------------------------------------------------------------------------------

#[cfg(feature = "std")]
macro_rules! boxit {
    (for $( $any:tt )+) => {
        ::std::boxed::Box$( $any )+
    };
    (new ( $value:expr )) => {
        ::std::boxed::Box::new($value)
    }
}

#[cfg(not(feature = "std"))]
macro_rules! boxit {
    (for $( $any:tt )+) => {
        ::alloc::boxed::Box$( $any )+
    };
    (new ( $value:expr )) => {
        ::alloc::boxed::Box::new($value)
    }
}

macro_rules! impl_has_annotations {
    ($type_name:ident, $member_name:ident) => {
        impl $crate::annotations::HasAnnotations for $type_name {
            fn has_annotations(&self) -> bool {
                !self.$member_name.is_empty()
            }

            fn annotations(&self) -> boxit!(for <dyn Iterator<Item = &$crate::annotations::Annotation> + '_>) {
                boxit!(new (self.$member_name.iter()))
            }

            fn annotations_mut(
                &mut self,
            ) -> boxit!(for <dyn Iterator<Item = &mut $crate::annotations::Annotation> + '_>) {
                boxit!(new(self.$member_name.iter_mut()))
            }
        }
    };

    ($type_name:ident) => {
        impl_has_annotations!($type_name, annotations);
    };

    ($type_name:ident enum $( $var_name:ident ),+ ) => {
           impl $crate::annotations::HasAnnotations for $type_name {
               fn has_annotations(&self) -> bool {
                    match self {
                    $(
                        Self::$var_name(v) => v.has_annotations(),
                    )+
                    }
               }

               fn annotations(&self) -> boxit!(for <dyn Iterator<Item = &$crate::annotations::Annotation> + '_>) {
                    match self {
                    $(
                        Self::$var_name(v) => v.annotations(),
                    )+
                    }
               }

               fn annotations_mut(
                   &mut self,
               ) -> boxit!(for <dyn Iterator<Item = &mut $crate::annotations::Annotation> + '_>) {
                    match self {
                    $(
                        Self::$var_name(v) => v.annotations_mut(),
                    )+
                    }
               }
           }
       };
}

// ------------------------------------------------------------------------------------------------
// Macros ❯ DisplayPretty
// ------------------------------------------------------------------------------------------------

macro_rules! impl_display_pretty {
    // --------------------------------------------------------------------------------------------
    // For Value Types
    // --------------------------------------------------------------------------------------------

    // DisplayPretty for enums of DisplayPretty variants
    ($type_name:ident only self) => {
        impl $crate::fmt::DisplayPretty for $type_name {
            fn fmt_pretty(
                &self,
                f: &mut ::core::fmt::Formatter<'_>,
                _: &$crate::fmt::Indenter,
                _: &::rdftk_iri::map::IriPrefixMap,
            ) -> ::core::fmt::Result {
                write!(f, "{}", self)
            }
        }
    };

    // --------------------------------------------------------------------------------------------
    // For Enums
    // --------------------------------------------------------------------------------------------

    // DisplayPretty for enums of DisplayPretty variants
    ($type_name:ident enum $( $var_name:ident ),+ ) => {
        impl_display_pretty!(@display $type_name);

        impl $crate::fmt::DisplayPretty for $type_name {
            fn fmt_pretty(
                &self,
                f: &mut ::core::fmt::Formatter<'_>,
                indenter: &$crate::fmt::Indenter,
                prefix_map: &::rdftk_iri::map::IriPrefixMap,
            ) -> ::core::fmt::Result {
                match self {
                    $(
                        Self::$var_name(v) => v.fmt_pretty(f, indenter, prefix_map),
                    )+
                }
            }
        }
    };

    // --------------------------------------------------------------------------------------------
    // For Structs
    // --------------------------------------------------------------------------------------------

    // DisplayPretty with zero or more arguments (OWL name same as type name)
    ($type_name:ident ( $( $( @$arg_keyword:ident )? $arg_name:ident ),+ ) ) => {
        impl_display_pretty!($type_name, $type_name ( $( $( @$arg_keyword )? $arg_name ),+ ) );
    };

    // DisplayPretty with zero or more arguments
    ($type_name:ident, $owl_name:ident ( $( $( @$arg_keyword:ident )? $arg_name:ident ),+ )) => {
        impl_display_pretty!(@display $type_name);

        impl $crate::fmt::DisplayPretty for $type_name {
            fn fmt_pretty(
                &self,
                f: &mut ::core::fmt::Formatter<'_>,
                indenter: &$crate::fmt::Indenter,
                prefix_map: &::rdftk_iri::map::IriPrefixMap,
            ) -> ::core::fmt::Result {
                impl_display_pretty!(@fn_start $owl_name, f, indenter);

                let separator = indenter.separator_string(f.alternate());
                $(
                    impl_display_pretty!(@arg $( $arg_keyword )? $arg_name => self, f, separator, indenter, prefix_map);
                )+

                impl_display_pretty!(@fn_end f, indenter)
            }
        }
    };

    // --------------------------------------------------------------------------------------------
    // Internal
    // --------------------------------------------------------------------------------------------

    (@fn_start $owl_name:ident, $f:expr, $indenter:expr) => {{
        write!($f, "{}(", stringify!($owl_name))?;
        if $f.alternate() {
            let _ = $indenter.indent();
        }
    }};

    (@arg $arg_name:ident => $self:expr, $f:expr, $separator:expr, $indenter:expr, $prefix_map:expr) => {
        write!($f, "{}", $separator)?;
        $self.$arg_name.fmt_pretty($f, &$indenter, $prefix_map)?;
    };

    (@arg optional $arg_name:ident => $self:expr, $f:expr, $separator:expr, $indenter:expr, $prefix_map:expr) => {
        if let Some(arg) = &$self.$arg_name {
            write!($f, "{}", $separator)?;
            arg.fmt_pretty($f, &$indenter, $prefix_map)?;
        }
    };

    (@arg display $arg_name:ident => $self:expr, $f:expr, $separator:expr, $indenter:expr, $prefix_map:expr) => {
        write!($f, "{}{}", $separator, $self.$arg_name)?;
    };

    (@arg list $arg_name:ident => $self:expr, $f:expr, $separator:expr, $indenter:expr, $prefix_map:expr) => {
        for arg in &$self.$arg_name {
            write!($f, "{}", $separator)?;
            arg.fmt_pretty($f, &$indenter, $prefix_map)?;
        }
    };

    (@fn_end $f:expr, $indenter:expr) => {{
        if $f.alternate() {
            let _ = $indenter.outdent();
        }
        write!($f, "{})", $indenter.separator_string($f.alternate()))?;
        Ok(())
    }};

    // Only output the Display implementation that forwards to DisplayPretty
    (@display $type_name:ident) => {
        impl ::core::fmt::Display for $type_name {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                self.fmt_pretty(
                    f,
                    &$crate::fmt::Indenter::default(),
                    &::rdftk_iri::map::IriPrefixMap::default()
                )
            }
        }
    };
}

// ------------------------------------------------------------------------------------------------
// Macros ❯ Enum FromForVariant
// ------------------------------------------------------------------------------------------------

macro_rules! impl_from_for_variant {
    ($enum_name:ident, $variant_and_type_name:ident) => {
        impl_from_for_variant!($enum_name, $variant_and_type_name($variant_and_type_name));
    };
    ($enum_name:ident, $variant_name:ident ( from $from_type_name:ident )) => {
        impl From<$from_type_name> for $enum_name {
            fn from(value: $from_type_name) -> Self {
                Self::$variant_name($variant_name::from(value))
            }
        }
    };
    ($enum_name:ident, $variant_name:ident ( $variant_type_name:ident )) => {
        impl From<$variant_type_name> for $enum_name {
            fn from(value: $variant_type_name) -> Self {
                Self::$variant_name(value)
            }
        }
    };
}

// ------------------------------------------------------------------------------------------------
// IRI Constant functions
// ------------------------------------------------------------------------------------------------

macro_rules! make_iri_function {
    ($fn_name:ident => $vocab_name:ident : $type_name:ident) => {
        pub fn $fn_name() -> ::rdftk_iri::Iri {
            $vocab_name
                .iri_as_iri()
                .make_name(::rdftk_iri::Name::new_unchecked(stringify!($type_name)))
                .unwrap()
        }
    };
}

// ------------------------------------------------------------------------------------------------
// Macros ❯ Debugging
// ------------------------------------------------------------------------------------------------
