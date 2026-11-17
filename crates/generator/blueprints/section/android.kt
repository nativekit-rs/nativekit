import androidx.compose.runtime.Composable

@Composable
fun SectionView(node: DynamicUINode, dispatch: (String, org.json.JSONObject?) -> Unit, modifier: Modifier = Modifier) {
    val hdrNode = node.children.firstOrNull { it.name == "section_header" }?.children?.firstOrNull()
    val ftrNode = node.children.firstOrNull { it.name == "section_footer" }?.children?.firstOrNull()
    val bodyChildren = node.children.filter { it.name != "section_header" && it.name != "section_footer" }
    val count = bodyChildren.size
    Column(modifier = modifier.fillMaxWidth(), verticalArrangement = Arrangement.spacedBy(6.dp)) {
        if (hdrNode != null) {
            Box(modifier = Modifier.fillMaxWidth().padding(horizontal = 8.dp)) {
                DynamicNodeView(hdrNode, dispatch, Modifier.fillMaxWidth())
            }
        }
        Column(modifier = Modifier.fillMaxWidth(), verticalArrangement = Arrangement.spacedBy(2.dp)) {
            for (i in 0 until count) {
                val child = bodyChildren[i]
                val shape = when {
                    count == 1 -> androidx.compose.foundation.shape.RoundedCornerShape(16.dp)
                    i == 0 -> androidx.compose.foundation.shape.RoundedCornerShape(topStart = 16.dp, topEnd = 16.dp, bottomStart = 4.dp, bottomEnd = 4.dp)
                    i == count - 1 -> androidx.compose.foundation.shape.RoundedCornerShape(topStart = 4.dp, topEnd = 4.dp, bottomStart = 16.dp, bottomEnd = 16.dp)
                    else -> androidx.compose.foundation.shape.RoundedCornerShape(4.dp)
                }
                val isBtn = child.name == "button"
                val isPicker = child.getBoolean("is_picker", false)
                val actStr = child.getString("action", "")
                val rowBgName = child.getString("background", "")
                val rowBgColor = resolveAppColor(rowBgName)
                val rowBgGrad = child.props["background_gradient"] as? org.json.JSONObject
                val rowBrush = if (rowBgGrad != null) resolveAppGradientBrush(rowBgGrad) else null
                val hasRowBg = rowBgColor != null || rowBrush != null
                val cleanChild = if (hasRowBg) {
                    DynamicUINode(
                        child.name,
                        child.props.filterKeys { it != "background" && it != "background_gradient" },
                        child.children,
                        child.inList
                    )
                } else {
                    child
                }
                Surface(
                    shape = shape,
                    color = if (rowBrush != null) Color.Transparent else (rowBgColor ?: MaterialTheme.colorScheme.surfaceVariant.copy(alpha = 0.55f)),
                    modifier = Modifier.fillMaxWidth().clip(shape)
                        .let { m -> if (rowBrush != null) m.background(brush = rowBrush) else m }
                        .let { m -> if (isBtn && actStr.isNotEmpty()) m.clickable { dispatch(actStr, null) } else m }
                ) {
                    Box(modifier = Modifier.fillMaxWidth().padding(horizontal = 16.dp, vertical = 14.dp)) {
                        if (isPicker) {
                            Row(modifier = Modifier.fillMaxWidth(), horizontalArrangement = Arrangement.SpaceBetween, verticalAlignment = Alignment.CenterVertically) {
                                Text(child.getString("picker_title", ""), style = MaterialTheme.typography.bodyLarge)
                                Text(child.getString("picker_value", "") + "  ›", style = MaterialTheme.typography.bodyMedium, color = MaterialTheme.colorScheme.onSurfaceVariant)
                            }
                        } else if (isBtn) {
                            Row(verticalAlignment = Alignment.CenterVertically, horizontalArrangement = Arrangement.spacedBy(8.dp)) {
                                CompositionLocalProvider(LocalContentColor provides (resolveAppColor(child.getString("color", "")) ?: MaterialTheme.colorScheme.primary)) {
                                    cleanChild.children.forEach { DynamicNodeView(it, dispatch) }
                                }
                            }
                        } else {
                            DynamicNodeView(cleanChild, dispatch, Modifier.fillMaxWidth())
                        }
                    }
                }
            }
        }
        if (ftrNode != null) {
            Box(modifier = Modifier.fillMaxWidth().padding(horizontal = 12.dp)) {
                DynamicNodeView(ftrNode, dispatch, Modifier.fillMaxWidth())
            }
        }
    }
}
