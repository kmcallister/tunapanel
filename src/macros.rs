//! tunapanel macros.

/// The core tunapanel macro.
#[macro_export]
macro_rules! tunapanel {
    (
        #[title = $title:expr]
        $(#[$attr:meta])*
        struct $struct_name:ident $body:tt
    ) => {
        tunapanel! { @VIS(struct),
            #[title = $title]
            $(#[$attr])*
            struct $struct_name $body
        }
    };

    (
        #[title = $title:expr]
        $(#[$attr:meta])*
        pub struct $struct_name:ident $body:tt
    ) => {
        tunapanel! { @VIS(pub struct),
            #[title = $title]
            $(#[$attr])*
            struct $struct_name $body
        }
    };

    (@VIS($($vis:tt)*),
        #[title = $title:expr]
        $(#[$attr:meta])*
        struct $struct_name:ident {
            $(
                #[label = $field_label:expr]
                $field_name:ident : $field_ty:ty = $field_init:expr,
            )*
        }
    ) => {
        $(#[$attr])*
        #[derive(Deserialize)]
        $($vis)* $struct_name {
           $(
               pub $field_name: $field_ty,
           )*
        }

        impl ::std::default::Default for $struct_name {
            fn default() -> $struct_name {
                $struct_name {
                    $(
                        $field_name: $field_init,
                    )*
                }
            }
        }

        impl $crate::Panel for $struct_name {
            fn title() -> &'static str {
                $title
            }

            fn widgets() -> $crate::HTML {
                use $crate::widget::Controllable;

                let panel: $struct_name = Default::default();
                let mut html = String::new();

                $(
                    html.push_str(&panel.$field_name.widget(
                        stringify!($field_name),
                        $field_label));
                )*

                html
            }
        }
    };
}
