#![allow(non_camel_case_types, non_upper_case_globals, dead_code)]

pub mod __props {
    pub struct FontProps {
        pub body: &'static str,
        pub callout: &'static str,
        pub caption1: &'static str,
        pub caption2: &'static str,
        pub footnote: &'static str,
        pub headline: &'static str,
        pub large_title: &'static str,
        pub subhead: &'static str,
        pub subheadline: &'static str,
        pub title1: &'static str,
        pub title2: &'static str,
        pub title3: &'static str,
    }

    pub const font: FontProps = FontProps {
        body: "body",
        callout: "callout",
        caption1: "caption1",
        caption2: "caption2",
        footnote: "footnote",
        headline: "headline",
        large_title: "large_title",
        subhead: "subhead",
        subheadline: "subheadline",
        title1: "title1",
        title2: "title2",
        title3: "title3",
    };

    pub struct ColorProps {
        pub black: &'static str,
        pub blue: &'static str,
        pub brown: &'static str,
        pub cyan: &'static str,
        pub danger: &'static str,
        pub fill: &'static str,
        pub gray: &'static str,
        pub green: &'static str,
        pub grouped_background: &'static str,
        pub indigo: &'static str,
        pub mint: &'static str,
        pub orange: &'static str,
        pub pink: &'static str,
        pub primary: &'static str,
        pub purple: &'static str,
        pub red: &'static str,
        pub secondary: &'static str,
        pub separator: &'static str,
        pub success: &'static str,
        pub surface: &'static str,
        pub surface_secondary: &'static str,
        pub teal: &'static str,
        pub text_primary: &'static str,
        pub text_secondary: &'static str,
        pub text_tertiary: &'static str,
        pub warning: &'static str,
        pub white: &'static str,
        pub yellow: &'static str,
    }

    pub const color: ColorProps = ColorProps {
        black: "black",
        blue: "blue",
        brown: "brown",
        cyan: "cyan",
        danger: "danger",
        fill: "fill",
        gray: "gray",
        green: "green",
        grouped_background: "grouped_background",
        indigo: "indigo",
        mint: "mint",
        orange: "orange",
        pink: "pink",
        primary: "primary",
        purple: "purple",
        red: "red",
        secondary: "secondary",
        separator: "separator",
        success: "success",
        surface: "surface",
        surface_secondary: "surface_secondary",
        teal: "teal",
        text_primary: "text_primary",
        text_secondary: "text_secondary",
        text_tertiary: "text_tertiary",
        warning: "warning",
        white: "white",
        yellow: "yellow",
    };

    pub const background: ColorProps = color;
    pub const stop: ColorProps = color;
    pub const stop_with_opacity: ColorProps = color;
    pub const add_stop: ColorProps = color;

    pub struct ButtonVariantProps {
        pub primary: &'static str,
        pub secondary: &'static str,
        pub tertiary: &'static str,
        pub text: &'static str,
    }

    pub const variant: ButtonVariantProps = ButtonVariantProps {
        primary: "primary",
        secondary: "secondary",
        tertiary: "tertiary",
        text: "text",
    };

    pub struct IconVariantProps {
        pub none: &'static str,
        pub fill: &'static str,
        pub circle: &'static str,
        pub circle_fill: &'static str,
        pub square: &'static str,
        pub square_fill: &'static str,
        pub slash: &'static str,
        pub rounded: &'static str,
        pub outlined: &'static str,
        pub sharp: &'static str,
        pub twotone: &'static str,
    }

    pub const icon_variant: IconVariantProps = IconVariantProps {
        none: "none",
        fill: "fill",
        circle: "circle",
        circle_fill: "circle.fill",
        square: "square",
        square_fill: "square.fill",
        slash: "slash",
        rounded: "rounded",
        outlined: "outlined",
        sharp: "sharp",
        twotone: "twotone",
    };

    pub struct RenderingProps {
        pub monochrome: &'static str,
        pub hierarchical: &'static str,
        pub palette: &'static str,
        pub multicolor: &'static str,
    }

    pub const rendering: RenderingProps = RenderingProps {
        monochrome: "monochrome",
        hierarchical: "hierarchical",
        palette: "palette",
        multicolor: "multicolor",
    };

    pub struct PlacementProps {
        pub leading: &'static str,
        pub trailing: &'static str,
        pub fill: &'static str,
        pub hug: &'static str,
    }

    pub const placement: PlacementProps = PlacementProps {
        leading: "leading",
        trailing: "trailing",
        fill: "fill",
        hug: "hug",
    };

    pub struct WidthProps {
        pub fill: &'static str,
        pub hug: &'static str,
    }

    pub const width: WidthProps = WidthProps {
        fill: "fill",
        hug: "hug",
    };

    pub struct AlignProps {
        pub top: &'static str,
        pub bottom: &'static str,
        pub center: &'static str,
        pub middle: &'static str,
        pub left: &'static str,
        pub right: &'static str,
        pub leading: &'static str,
        pub trailing: &'static str,
        pub space_between: &'static str,
        pub top_left: &'static str,
        pub top_center: &'static str,
        pub top_right: &'static str,
        pub center_left: &'static str,
        pub center_right: &'static str,
        pub middle_left: &'static str,
        pub middle_right: &'static str,
        pub bottom_left: &'static str,
        pub bottom_center: &'static str,
        pub bottom_right: &'static str,
    }

    pub const align: AlignProps = AlignProps {
        top: "top",
        bottom: "bottom",
        center: "center",
        middle: "middle",
        left: "left",
        right: "right",
        leading: "leading",
        trailing: "trailing",
        space_between: "space_between",
        top_left: "top_left",
        top_center: "top_center",
        top_right: "top_right",
        center_left: "center_left",
        center_right: "center_right",
        middle_left: "middle_left",
        middle_right: "middle_right",
        bottom_left: "bottom_left",
        bottom_center: "bottom_center",
        bottom_right: "bottom_right",
    };

    pub struct TopStyleProps {
        pub soft: &'static str,
        pub hard: &'static str,
    }

    pub const top_style: TopStyleProps = TopStyleProps {
        soft: "soft",
        hard: "hard",
    };

    pub struct PresentationProps {
        pub push: &'static str,
        pub sheet: &'static str,
        pub form: &'static str,
        pub dialog: &'static str,
        pub modal: &'static str,
        pub fullscreen: &'static str,
    }

    pub const presentation: PresentationProps = PresentationProps {
        push: "push",
        sheet: "sheet",
        form: "form",
        dialog: "dialog",
        modal: "modal",
        fullscreen: "fullscreen",
    };

    pub struct ContentModeProps {
        pub cover: &'static str,
        pub contain: &'static str,
        pub fit: &'static str,
    }

    pub const content_mode: ContentModeProps = ContentModeProps {
        cover: "cover",
        contain: "contain",
        fit: "fit",
    };

    pub struct PickerStyleProps {
        pub drill_in: &'static str,
        pub inline: &'static str,
        pub segmented: &'static str,
        pub menu: &'static str,
        pub wheel: &'static str,
    }

    pub const style: PickerStyleProps = PickerStyleProps {
        drill_in: "drill-in",
        inline: "inline",
        segmented: "segmented",
        menu: "menu",
        wheel: "wheel",
    };

    pub struct DirectionProps {
        pub to_right: &'static str,
        pub to_left: &'static str,
        pub to_bottom: &'static str,
        pub to_top: &'static str,
        pub to_bottom_right: &'static str,
        pub to_bottom_left: &'static str,
        pub to_top_right: &'static str,
        pub to_top_left: &'static str,
    }

    pub const direction: DirectionProps = DirectionProps {
        to_right: "to_right",
        to_left: "to_left",
        to_bottom: "to_bottom",
        to_top: "to_top",
        to_bottom_right: "to_bottom_right",
        to_bottom_left: "to_bottom_left",
        to_top_right: "to_top_right",
        to_top_left: "to_top_left",
    };

    pub struct GradientKindProps {
        pub linear: &'static str,
        pub radial: &'static str,
        pub angular: &'static str,
    }

    pub const kind: GradientKindProps = GradientKindProps {
        linear: "linear",
        radial: "radial",
        angular: "angular",
    };

    pub struct IconNameProps {
        pub arrow_left: &'static str,
        pub arrow_right: &'static str,
        pub bell: &'static str,
        pub bolt: &'static str,
        pub brand_spark: &'static str,
        pub cart: &'static str,
        pub check: &'static str,
        pub check_circle: &'static str,
        pub chevron_left: &'static str,
        pub chevron_right: &'static str,
        pub close: &'static str,
        pub coffee: &'static str,
        pub edit: &'static str,
        pub heart: &'static str,
        pub home: &'static str,
        pub house: &'static str,
        pub info: &'static str,
        pub lock: &'static str,
        pub mail: &'static str,
        pub navigate_back: &'static str,
        pub orders: &'static str,
        pub plus: &'static str,
        pub qrcode: &'static str,
        pub refresh: &'static str,
        pub search: &'static str,
        pub settings: &'static str,
        pub share: &'static str,
        pub sparkles: &'static str,
        pub star: &'static str,
        pub star_fill: &'static str,
        pub trash: &'static str,
        pub user: &'static str,
        pub warning: &'static str,
    }

    pub const icon: IconNameProps = IconNameProps {
        arrow_left: "arrow_left",
        arrow_right: "arrow_right",
        bell: "bell",
        bolt: "bolt",
        brand_spark: "brand_spark",
        cart: "cart",
        check: "check",
        check_circle: "check_circle",
        chevron_left: "chevron_left",
        chevron_right: "chevron_right",
        close: "close",
        coffee: "coffee",
        edit: "edit",
        heart: "heart",
        home: "home",
        house: "house",
        info: "info",
        lock: "lock",
        mail: "mail",
        navigate_back: "navigate_back",
        orders: "orders",
        plus: "plus",
        qrcode: "qrcode",
        refresh: "refresh",
        search: "search",
        settings: "settings",
        share: "share",
        sparkles: "sparkles",
        star: "star",
        star_fill: "star.fill",
        trash: "trash",
        user: "user",
        warning: "warning",
    };

    macro_rules! impl_props_into_string {
        ($($t:ty),* $(,)?) => {
            $(
                impl From<$t> for String {
                    fn from(_: $t) -> String {
                        String::new()
                    }
                }
                impl crate::engine::primitives::color::IntoBackground for $t {
                    fn into_background(self) -> crate::engine::primitives::color::BackgroundValue {
                        crate::engine::primitives::color::BackgroundValue::Solid(String::new())
                    }
                }
            )*
        };
    }

    impl_props_into_string!(
        FontProps,
        ColorProps,
        ButtonVariantProps,
        IconVariantProps,
        RenderingProps,
        PlacementProps,
        WidthProps,
        AlignProps,
        TopStyleProps,
        PresentationProps,
        ContentModeProps,
        PickerStyleProps,
        DirectionProps,
        GradientKindProps,
        IconNameProps,
    );
}
