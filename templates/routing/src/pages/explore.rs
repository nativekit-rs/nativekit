#![allow(non_snake_case)]
use native_kit::*;

pub fn ExploreScreen() -> impl IntoNode {
    Return! {
        Page.title("Explore").large_title(true).top_style(.soft) {
            Section.header("Featured") {
                HStack.spacing(12) {
                    Icon(.sparkles).color(.primary)
                    VStack.spacing(4) {
                        Text("5 Native Platforms").font(.headline)
                        Text("SwiftUI, Jetpack Compose, GTK4, WinUI 3, and Web DOM.")
                            .font(.caption1)
                            .color(.text_secondary)
                    }
                }
            }
        }
    }
}
