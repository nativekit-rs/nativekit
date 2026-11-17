import SwiftUI

@ViewBuilder
func GridView(node: DynamicUINode, dispatch: @escaping (String, [String: Any]?) -> Void) -> some View {
    let cols = max(1, node.props["columns"] as? Int ?? 2)
    let spacing = CGFloat(node.props["spacing"] as? Int ?? 12)
    let headerStr = node.props["header"] as? String ?? ""
    let footerStr = node.props["footer"] as? String ?? ""
    let headerNode = node.children.first { $0.name == "section_header" }?.children.first
    let footerNode = node.children.first { $0.name == "section_footer" }?.children.first
    let bodyChildren = node.children.filter { $0.name != "section_header" && $0.name != "section_footer" }
    let gridCols = Array(repeating: GridItem(.flexible(), spacing: spacing), count: cols)

    VStack(alignment: .leading, spacing: 6) {
        if let headerNode = headerNode {
            DynamicNodeView(node: headerNode, dispatch: dispatch)
                .padding(.horizontal, 4)
        } else if !headerStr.isEmpty {
            Text(headerStr.uppercased())
                .font(.footnote)
                .foregroundStyle(Color.secondary)
                .padding(.horizontal, 4)
        }
        LazyVGrid(columns: gridCols, spacing: spacing) {
            ForEach(0..<bodyChildren.count, id: \.self) { i in
                DynamicNodeView(node: bodyChildren[i], dispatch: dispatch)
            }
        }
        if let footerNode = footerNode {
            DynamicNodeView(node: footerNode, dispatch: dispatch)
                .padding(.horizontal, 4)
        } else if !footerStr.isEmpty {
            Text(footerStr)
                .font(.footnote)
                .foregroundStyle(Color.secondary)
                .padding(.horizontal, 4)
        }
    }
}
