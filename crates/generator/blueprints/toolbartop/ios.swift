import SwiftUI
@ViewBuilder
func ToolbarTopView(node: DynamicUINode, dispatch: @escaping (String, [String: Any]?) -> Void) -> some View {
    ForEach(0..<node.children.count, id: \.self) { i in DynamicNodeView(node: node.children[i], dispatch: dispatch) }
}
