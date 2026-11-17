import SwiftUI

@ViewBuilder
func SwitchView(node: DynamicUINode, dispatch: @escaping (String, [String: Any]?) -> Void) -> some View {
    let on_change_str = node.props["on_change"] as? String ?? ""
    let value = node.props["value"] as? Bool ?? false
    let title = node.props["title"] as? String ?? ""
    let binding = Binding<Bool>(
        get: { value },
        set: { newVal in
            if !on_change_str.isEmpty {
                dispatch(on_change_str, ["value": newVal])
            }
        }
    )
    if !title.isEmpty {
        Toggle(title, isOn: binding)
    } else {
        Toggle(isOn: binding) { EmptyView() }.labelsHidden()
    }
}
