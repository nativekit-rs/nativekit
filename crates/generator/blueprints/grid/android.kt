import androidx.compose.runtime.Composable

@Composable
fun GridView(node: DynamicUINode, dispatch: (String, org.json.JSONObject?) -> Unit, modifier: Modifier = Modifier) {
    val cols = maxOf(1, node.getInt("columns", 2))
    val spacing = node.getInt("spacing", 12)
    val hdrNode = node.children.firstOrNull { it.name == "section_header" }?.children?.firstOrNull()
    val ftrNode = node.children.firstOrNull { it.name == "section_footer" }?.children?.firstOrNull()
    val bodyChildren = node.children.filter { it.name != "section_header" && it.name != "section_footer" }
    val rows = bodyChildren.chunked(cols)
    Column(modifier = modifier.fillMaxWidth(), verticalArrangement = Arrangement.spacedBy(spacing.dp)) {
        if (hdrNode != null) {
            Box(modifier = Modifier.fillMaxWidth().padding(horizontal = 4.dp)) { DynamicNodeView(hdrNode, dispatch, Modifier.fillMaxWidth()) }
        }
        rows.forEach { rowItems ->
            Row(modifier = Modifier.fillMaxWidth(), horizontalArrangement = Arrangement.spacedBy(spacing.dp)) {
                for (i in 0 until cols) {
                    if (i < rowItems.size) {
                        Box(modifier = Modifier.weight(1f)) { DynamicNodeView(rowItems[i], dispatch, Modifier.fillMaxWidth()) }
                    } else {
                        Spacer(modifier = Modifier.weight(1f))
                    }
                }
            }
        }
        if (ftrNode != null) {
            Box(modifier = Modifier.fillMaxWidth().padding(horizontal = 4.dp)) { DynamicNodeView(ftrNode, dispatch, Modifier.fillMaxWidth()) }
        }
    }
}
