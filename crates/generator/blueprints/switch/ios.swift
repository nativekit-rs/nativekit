import SwiftUI

@ViewBuilder
func SwitchView(node: DynamicUINode, dispatch: @escaping (String, [String: Any]?) -> Void) -> some View {
    let on_change_str = node.props["on_change"] as? String ?? ""
    let on_change_action: () -> Void = { if !on_change_str.isEmpty { dispatch(on_change_str, nil) } }
    let value = node.props["value"] as? Bool ?? false
    let title = node.props["title"] as? String ?? ""
    if !title.isEmpty {
        Toggle(title, isOn: Binding(get: { value }, set: { _ in on_change_action() }))
    } else {
        Toggle(isOn: Binding(get: { value }, set: { _ in on_change_action() })) { EmptyView() }.labelsHidden()
    }
}
