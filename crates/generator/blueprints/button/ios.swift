import SwiftUI

@ViewBuilder
func ButtonView(node: DynamicUINode, dispatch: @escaping (String, [String: Any]?) -> Void) -> some View {
    let variant = node.props["variant"] as? String ?? ""
    let colorName = node.props["color"] as? String ?? ""
    let explicitColor = resolveAppColor(colorName)
    let size = node.props["size"] as? String ?? "regular"
    let ctrlSize: ControlSize = size == "small" ? .small : (size == "large" ? .large : .regular)
    let isIconOnly = (node.props["icon_only"] as? Bool ?? false) || (node.children.count == 1 && node.children[0].name == "icon")
    let a11yLabel = node.props["label"] as? String ?? ""
    let borderShape: ButtonBorderShape = isIconOnly ? .circle : .capsule
    let action_str = node.props["action"] as? String ?? ""
    let opacity = Double(node.props["opacity"] as? Int ?? 100) / 100.0
    let isFill = (node.props["width"] as? String) == "fill"
    let widthPx = node.props["width_px"] as? Int ?? 0
    let heightPx = node.props["height_px"] as? Int ?? 0
    let inSection = node.props["in_section"] as? Bool ?? false
    let isPicker = node.props["is_picker"] as? Bool ?? false
    let onTap: () -> Void = { if !action_str.isEmpty { dispatch(action_str, nil) } }

    let labelContent = Group {
        if node.children.count == 1 {
            DynamicNodeView(node: node.children[0], dispatch: dispatch)
        } else {
            HStack(spacing: 6) {
                ForEach(0..<node.children.count, id: \.self) { i in
                    DynamicNodeView(node: node.children[i], dispatch: dispatch)
                }
            }
        }
    }

    let makeButtonLabel = {
        labelContent
            .frame(
                width: widthPx > 0 ? CGFloat(widthPx) : nil,
                height: heightPx > 0 ? CGFloat(heightPx) : nil
            )
            .frame(
                maxWidth: (isFill || widthPx > 0) ? .infinity : nil,
                alignment: .center
            )
            .contentShape(Rectangle())
    }

    let baseBtn = Group {
        if isPicker {
            let pTitle = node.props["picker_title"] as? String ?? "Select"
            let pVal = node.props["picker_value"] as? String ?? ""
            Button(action: onTap) {
                HStack {
                    Text(pTitle).foregroundStyle(Color.primary)
                    Spacer()
                    Text(pVal).foregroundStyle(Color.secondary)
                    Image(systemName: "chevron.right")
                        .font(.caption.weight(.semibold))
                        .foregroundStyle(Color(uiColor: .tertiaryLabel))
                }
                .contentShape(Rectangle())
            }
            .buttonStyle(.borderless)
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
            .buttonStyle(.borderless)
            .foregroundStyle(explicitColor ?? AppTheme.primary)
        } else if variant == "primary" {
            Button(action: onTap) {
                makeButtonLabel()
            }
            .buttonStyle(.borderedProminent)
            .buttonBorderShape(borderShape)
            .controlSize(ctrlSize)
            .accessibilityLabel(a11yLabel)
        } else if variant == "secondary" {
            Button(action: onTap) {
                makeButtonLabel()
            }
            .buttonStyle(.bordered)
            .buttonBorderShape(borderShape)
            .controlSize(ctrlSize)
            .accessibilityLabel(a11yLabel)
        } else {
            Button(action: onTap) {
                makeButtonLabel()
            }
            .buttonStyle(.borderless)
            .buttonBorderShape(borderShape)
            .controlSize(ctrlSize)
            .accessibilityLabel(a11yLabel)
        }
    }
    .opacity(opacity)

    if let explicitColor = explicitColor {
        baseBtn.tint(explicitColor)
    } else {
        baseBtn.tint(AppTheme.primary)
    }
}
