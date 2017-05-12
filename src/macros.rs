#[macro_export]
macro_rules! tunapanel {
    (
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
         struct $struct_name {
            $(
                $field_name: $field_ty,
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
            fn widgets() -> $crate::widget::HTML {
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
    }
}
