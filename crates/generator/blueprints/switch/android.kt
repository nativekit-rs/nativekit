import androidx.compose.runtime.Composable

@Composable
fun SwitchView(node: DynamicUINode, dispatch: (String, org.json.JSONObject?) -> Unit, modifier: Modifier = Modifier) {
    val on_change_str = node.getString("on_change", "")
    val on_change_action: () -> Unit = { if (on_change_str.isNotEmpty()) dispatch(on_change_str, null) }
    val value = node.getBoolean("value", false)
    val title = node.getString("title", "")
    if (title.isNotEmpty()) {
        Row(
            modifier = modifier.fillMaxWidth(),
            horizontalArrangement = Arrangement.SpaceBetween,
            verticalAlignment = Alignment.CenterVertically
        ) {
            Text(title, style = MaterialTheme.typography.bodyLarge)
            Switch(checked = value, onCheckedChange = { on_change_action() })
        }
    } else {
        Switch(checked = value, onCheckedChange = { on_change_action() })
    }
}
