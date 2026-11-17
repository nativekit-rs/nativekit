import SwiftUI

@ViewBuilder
func NavigationStackView(node: DynamicUINode, dispatch: @escaping (String, [String: Any]?) -> Void) -> some View {
    let on_pop = node.props["on_pop"] as? String ?? ""
    let on_dismiss = node.props["on_dismiss"] as? String ?? ""
    DynamicNavigationStack(node: node, dispatch: dispatch, onPop: on_pop, onDismiss: on_dismiss)
}
