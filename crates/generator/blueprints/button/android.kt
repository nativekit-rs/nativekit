import androidx.compose.runtime.Composable

@Composable
fun ButtonView(node: DynamicUINode, dispatch: (String, org.json.JSONObject?) -> Unit, modifier: Modifier = Modifier) {
    val variant = node.getString("variant", "")
    val actionStr = node.getString("action", "")
    val colorName = node.getString("color", "")
    val opacity = node.getInt("opacity", 100).toFloat() / 100f
    val isFill = node.getString("width") == "fill"
    val customColor = resolveAppColor(colorName)
    var btnMod = if (isFill && modifier == Modifier) modifier.fillMaxWidth() else modifier
    if (opacity < 1f) btnMod = btnMod.graphicsLayer(alpha = opacity)
    val onClick: () -> Unit = { if (actionStr.isNotEmpty()) dispatch(actionStr, null) }

    when (variant) {
        "primary" -> Button(
            onClick = onClick,
            modifier = btnMod,
            colors = if (customColor != null) ButtonDefaults.buttonColors(containerColor = customColor) else ButtonDefaults.buttonColors()
        ) { node.children.forEach { DynamicNodeView(it, dispatch) } }
        "secondary" -> FilledTonalButton(
            onClick = onClick,
            modifier = btnMod,
            colors = if (customColor != null) ButtonDefaults.filledTonalButtonColors(containerColor = customColor) else ButtonDefaults.filledTonalButtonColors()
        ) { node.children.forEach { DynamicNodeView(it, dispatch) } }
        else -> TextButton(
            onClick = onClick,
            modifier = btnMod,
            colors = if (customColor != null) ButtonDefaults.textButtonColors(contentColor = customColor) else ButtonDefaults.textButtonColors()
        ) { node.children.forEach { DynamicNodeView(it, dispatch) } }
    }
}
