import SwiftUI

@ViewBuilder
func ButtonView(node: DynamicUINode, dispatch: @escaping (String, [String: Any]?) -> Void) -> some View {
    let variant = node.props["variant"] as? String ?? ""
    let action_str = node.props["action"] as? String ?? ""
    let colorName = node.props["color"] as? String ?? ""
    let opacity = Double(node.props["opacity"] as? Int ?? 100) / 100.0
    let isFill = (node.props["width"] as? String) == "fill"
    let inSection = node.props["in_section"] as? Bool ?? false
    let isPicker = node.props["is_picker"] as? Bool ?? false
    let customColor = resolveAppColor(colorName)
    let onTap: () -> Void = { if !action_str.isEmpty { dispatch(action_str, nil) } }

    if isPicker {
        let pTitle = node.props["picker_title"] as? String ?? "Select"
        let pVal = node.props["picker_value"] as? String ?? ""
        Button(action: onTap) {
            HStack {
                Text(pTitle).foregroundStyle(Color.primary)
                Spacer()
                Text(pVal).foregroundStyle(Color.secondary)
                Image(systemName: "chevron.right").font(.caption.weight(.semibold)).foregroundStyle(Color(uiColor: .tertiaryLabel))
            }
            .contentShape(Rectangle())
        }
        .buttonStyle(.plain)
    } else if inSection {
        Button(action: onTap) {
            HStack(spacing: 8) {
                ForEach(0..<node.children.count, id: \.self) { i in
                    DynamicNodeView(node: node.children[i], dispatch: dispatch)
                }
                Spacer()
            }
            .contentShape(Rectangle())
        }
        .foregroundStyle(customColor ?? AppTheme.primary)
        .opacity(opacity)
    } else if variant == "primary" {
        Button(action: onTap) {
            HStack(spacing: 6) {
                ForEach(0..<node.children.count, id: \.self) { i in DynamicNodeView(node: node.children[i], dispatch: dispatch) }
            }
            .fixedSize(horizontal: !isFill, vertical: false)
            .frame(maxWidth: isFill ? .infinity : nil, alignment: .center)
        }
        .fixedSize(horizontal: !isFill, vertical: false)
        .frame(maxWidth: isFill ? .infinity : nil, alignment: .center)
        .buttonStyle(.borderedProminent)
        .tint(customColor)
        .opacity(opacity)
    } else if variant == "secondary" {
        Button(action: onTap) {
            HStack(spacing: 6) {
                ForEach(0..<node.children.count, id: \.self) { i in DynamicNodeView(node: node.children[i], dispatch: dispatch) }
            }
            .fixedSize(horizontal: !isFill, vertical: false)
            .frame(maxWidth: isFill ? .infinity : nil, alignment: .center)
        }
        .fixedSize(horizontal: !isFill, vertical: false)
        .frame(maxWidth: isFill ? .infinity : nil, alignment: .center)
        .buttonStyle(.bordered)
        .tint(customColor)
        .opacity(opacity)
    } else {
        Button(action: onTap) {
            HStack(spacing: 6) {
                ForEach(0..<node.children.count, id: \.self) { i in DynamicNodeView(node: node.children[i], dispatch: dispatch) }
            }
            .fixedSize(horizontal: !isFill, vertical: false)
            .frame(maxWidth: isFill ? .infinity : nil, alignment: .center)
        }
        .fixedSize(horizontal: !isFill, vertical: false)
        .frame(maxWidth: isFill ? .infinity : nil, alignment: .center)
        .buttonStyle(.borderless)
        .tint(customColor)
        .opacity(opacity)
    }
}
