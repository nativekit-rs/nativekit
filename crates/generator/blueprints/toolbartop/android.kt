import androidx.compose.runtime.Composable
@Composable
fun ToolbarTopView(node: DynamicUINode, dispatch: (String, org.json.JSONObject?) -> Unit, modifier: Modifier = Modifier) {
    node.children.forEach { DynamicNodeView(it, dispatch) }
}
