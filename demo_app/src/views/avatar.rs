#[component(
    variant: .glass | .regular = .regular,
    initials?: string,
    size: .large | .medium | number = .medium,
    background: Color = .primary,
    show_full_name: bool = true,
    full_name: string = "John Doe",
    header?: View,
    on_press?: Action,
    children: Children,
)]
pub fn Avatar() {
    Return! {
        let px = match size {
            .large => 48,
            .medium => 36,
            .custom(n) => n,
        };

        let bg_opacity = match variant {
            .glass => 0.85,
            .regular => 1.0,
        };

        let badge_text = initials
            .unwrap_or_else(|| full_name.chars().take(2).collect())
            .to_uppercase();

        VStack.spacing(8).padding(12).corner_radius(12).background(.surface_secondary) {
            if let Some(hdr) = header {
                hdr
            }
            HStack.spacing(10) {
                VStack
                    .padding(px / 4)
                    .corner_radius(px / 2)
                    .background(background)
                    .opacity(bg_opacity)
                {
                    Text(badge_text).font(.headline).color(.white)
                }
                if show_full_name {
                    Text(full_name).font(.body).color(.text_primary)
                }
                if let Some(act) = on_press {
                    Button("Tap").variant(.secondary).action(act)
                }
            }
            children
        }
    }
}
