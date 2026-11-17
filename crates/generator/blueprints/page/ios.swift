import SwiftUI

@ViewBuilder
func PageView(node: DynamicUINode, dispatch: @escaping (String, [String: Any]?) -> Void) -> some View {
    let title = node.props["title"] as? String ?? ""
    let subtitle = node.props["subtitle"] as? String ?? ""
    let largeTitle = node.props["large_title"] as? Bool ?? false
    let topStyle = node.props["top_style"] as? String ?? "soft"
    let toolbarNode = node.children.first { $0.name == "toolbartop" }
    let toolbarItems = toolbarNode?.children ?? []
    let leadingItems = toolbarItems.filter { ($0.props["placement"] as? String ?? $0.children.first?.props["placement"] as? String ?? "trailing") == "leading" }
    let trailingItems = toolbarItems.filter { ($0.props["placement"] as? String ?? $0.children.first?.props["placement"] as? String ?? "trailing") != "leading" }
    let bodyChildren = node.children.filter { $0.name != "toolbartop" && $0.name != "toolbarbottom" }
    let sectionSpacing = CGFloat(node.props["section_spacing"] as? Int ?? 16)

    let content = List {
        ForEach(0..<bodyChildren.count, id: \.self) { i in
            let child = bodyChildren[i]
            if child.name == "section" {
                DynamicNodeView(node: child, dispatch: dispatch)
            } else {
                DynamicNodeView(node: child, dispatch: dispatch)
                    .listRowInsets(EdgeInsets(top: 6, leading: 16, bottom: 6, trailing: 16))
                    .listRowBackground(Color.clear)
                    .listRowSeparator(.hidden)
            }
        }
    }
    .listStyle(.insetGrouped)
    .scrollDismissesKeyboard(.interactively)
    .listSectionSpacing(sectionSpacing)

    Group {
        if topStyle == "hard" {
            content.scrollEdgeEffect(.top, .hard)
        } else {
            content
        }
    }
    .navigationTitle(title)
    .navigationBarTitleDisplayMode(topStyle == "hard" ? .inline : (largeTitle ? .large : .inline))
    .navigationBarBackButtonHidden(!leadingItems.isEmpty)
    .toolbar {
        if !leadingItems.isEmpty {
            ToolbarItem(placement: .topBarLeading) {
                HStack(spacing: 8) {
                    ForEach(0..<leadingItems.count, id: \.self) { i in
                        DynamicNodeView(node: leadingItems[i], dispatch: dispatch)
                    }
                }
            }
        }
        if !subtitle.isEmpty {
            ToolbarItem(placement: .principal) {
                VStack {
                    Text(title).font(.headline)
                    Text(subtitle).font(.caption)
                }
            }
        }
        if !trailingItems.isEmpty {
            ToolbarItem(placement: .topBarTrailing) {
                HStack(spacing: 8) {
                    ForEach(0..<trailingItems.count, id: \.self) { i in
                        DynamicNodeView(node: trailingItems[i], dispatch: dispatch)
                    }
                }
            }
        }
    }
}
