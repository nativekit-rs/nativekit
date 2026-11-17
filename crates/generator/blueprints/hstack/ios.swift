import SwiftUI

@ViewBuilder
func HStackView(node: DynamicUINode, dispatch: @escaping (String, [String: Any]?) -> Void) -> some View {
    let spacing = node.props["spacing"] as? Int ?? 0
    let widthProp = node.props["width"] as? String ?? ""
    let alignX = node.props["align_x"] as? String ?? "left"
    let alignY = node.props["align_y"] as? String ?? "center"
    let isWrap = node.props["wrap"] as? Bool ?? false
    let isHeaderBar = node.props["is_header_bar"] as? Bool ?? false
    let isSpaceBetween = alignX == "space_between" || (isHeaderBar && node.children.count == 2)
    let hasFlexChild = node.children.contains {
        ($0.props["width"] as? String) == "fill"
            || ($0.name == "spacer" && ($0.props["size"] as? Int ?? 0) == 0)
            || ($0.name == "vstack" && ($0.props["width"] as? String ?? "") != "hug")
    }
    let isFill = widthProp == "fill" || (widthProp != "hug" && (hasFlexChild || isSpaceBetween))
    let vAlign: VerticalAlignment = alignY == "top" ? .top : (alignY == "bottom" ? .bottom : .center)
    let frameAlign: Alignment = {
        switch (alignX, alignY) {
        case ("center", "top"): return .top
        case ("right", "top"): return .topTrailing
        case (_, "top"): return .topLeading
        case ("center", "bottom"): return .bottom
        case ("right", "bottom"): return .bottomTrailing
        case (_, "bottom"): return .bottomLeading
        case ("center", _): return .center
        case ("right", _): return .trailing
        default: return .leading
        }
    }()

    if isWrap {
        NativeFlowLayout(spacing: CGFloat(spacing), alignX: alignX, alignY: alignY) {
            ForEach(0..<node.children.count, id: \.self) { i in
                DynamicNodeView(node: node.children[i], dispatch: dispatch)
            }
        }
        .frame(maxWidth: isFill ? .infinity : nil, alignment: frameAlign)
        .applyContainerStyle(node: node)
    } else {
        HStack(alignment: vAlign, spacing: CGFloat(spacing)) {
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
}
