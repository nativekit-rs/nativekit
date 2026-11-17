import SwiftUI

@ViewBuilder
func SpacerView(node: DynamicUINode, dispatch: @escaping (String, [String: Any]?) -> Void) -> some View {
    let size = node.props["size"] as? Int ?? 0
    if size > 0 {
        Spacer().frame(width: CGFloat(size), height: CGFloat(size))
    } else {
        Spacer(minLength: 0)
    }
}
