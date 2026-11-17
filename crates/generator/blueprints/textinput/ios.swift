import SwiftUI

@ViewBuilder
func TextInputView(node: DynamicUINode, dispatch: @escaping (String, [String: Any]?) -> Void) -> some View {
    let value = node.props["value"] as? String ?? ""
    let placeholder = node.props["placeholder"] as? String ?? ""
    let onChangeStr = node.props["on_change"] as? String ?? ""
    DynamicTextInputWrapper(value: value, placeholder: placeholder, onChangeAction: onChangeStr, dispatch: dispatch)
}
