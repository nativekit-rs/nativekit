import SwiftUI

@ViewBuilder
func VStackView(node: DynamicUINode, dispatch: @escaping (String, [String: Any]?) -> Void) -> some View {
    let spacing = node.props["spacing"] as? Int ?? 0
    let widthProp = node.props["width"] as? String ?? ""
    let alignX = node.props["align_x"] as? String ?? "left"
    let alignY = node.props["align_y"] as? String ?? "top"
    let isFill = widthProp != "hug"
    let isSpaceBetween = alignY == "space_between"
    let hAlign: HorizontalAlignment = alignX == "center" ? .center : (alignX == "right" ? .trailing : .leading)
    let frameAlign: Alignment = {
        switch (alignX, alignY) {
        case ("center", "center"): return .center
        case ("right", "center"): return .trailing
        case (_, "center"): return .leading
        case ("center", "bottom"): return .bottom
        case ("right", "bottom"): return .bottomTrailing
        case (_, "bottom"): return .bottomLeading
        case ("center", _): return .top
        case ("right", _): return .topTrailing
        default: return .topLeading
        }
    }()

    VStack(alignment: hAlign, spacing: CGFloat(spacing)) {
        ForEach(0..<node.children.count, id: \.self) { i in
            if isSpaceBetween && i > 0 {
                Spacer(minLength: CGFloat(spacing))
            }
            DynamicNodeView(node: node.children[i], dispatch: dispatch)
        }
    }
    .frame(maxWidth: isFill ? .infinity : nil, alignment: frameAlign)
    .applyContainerStyle(node: node)
}
