import androidx.compose.runtime.Composable

@Composable
fun SpacerView(node: DynamicUINode, dispatch: (String, org.json.JSONObject?) -> Unit, modifier: Modifier = Modifier) {
    val size = node.getInt("size", 0)
    if (size > 0) {
        androidx.compose.foundation.layout.Spacer(modifier = modifier.size(size.dp))
    } else {
        androidx.compose.foundation.layout.Spacer(modifier = modifier)
    }
}
