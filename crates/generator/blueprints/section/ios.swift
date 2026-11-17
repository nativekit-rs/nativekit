import SwiftUI

@ViewBuilder
func SectionView(node: DynamicUINode, dispatch: @escaping (String, [String: Any]?) -> Void) -> some View {
    let headerStr = (node.props["header"] as? String) ?? (node.props["title"] as? String) ?? ""
    let footerStr = node.props["footer"] as? String ?? ""
    let isProminent = node.props["prominent"] as? Bool ?? false
    let headerNode = node.children.first { $0.name == "section_header" }?.children.first
    let footerNode = node.children.first { $0.name == "section_footer" }?.children.first
    let bodyChildren = node.children.filter { $0.name != "section_header" && $0.name != "section_footer" }

    Section {
        ForEach(0..<bodyChildren.count, id: \.self) { i in
            let child = bodyChildren[i]
            let delAction = child.props["on_delete"] as? String ?? ""
            let rowBgName = child.props["background"] as? String ?? ""
            let rowBgGrad = child.props["background_gradient"] as? [String: Any]
            let hasRowBg = !rowBgName.isEmpty || rowBgGrad != nil
            let cleanChild = hasRowBg
                ? DynamicUINode(
                    name: child.name,
                    props: child.props.filter { $0.key != "background" && $0.key != "background_gradient" },
                    children: child.children
                )
                : child
            if !delAction.isEmpty {
                DynamicNodeView(node: cleanChild, dispatch: dispatch)
                    .applyListRowBackground(node: child)
                    .swipeActions(edge: .trailing, allowsFullSwipe: true) {
                        Button(role: .destructive) {
                            dispatch(delAction, nil)
                        } label: {
                            Label("Delete", systemImage: "trash")
                        }
                    }
            } else {
                DynamicNodeView(node: cleanChild, dispatch: dispatch)
                    .applyListRowBackground(node: child)
            }
        }
    } header: {
        if let headerNode = headerNode {
            DynamicNodeView(node: headerNode, dispatch: dispatch)
        } else if !headerStr.isEmpty {
            Text(headerStr)
        }
    } footer: {
        if let footerNode = footerNode {
            DynamicNodeView(node: footerNode, dispatch: dispatch)
        } else if !footerStr.isEmpty {
            Text(footerStr)
        }
    }
    .headerProminence(isProminent ? .increased : .standard)
}
