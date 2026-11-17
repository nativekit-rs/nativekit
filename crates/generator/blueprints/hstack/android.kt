import androidx.compose.runtime.Composable

@Composable
fun HStackView(node: DynamicUINode, dispatch: (String, org.json.JSONObject?) -> Unit, modifier: Modifier = Modifier) {
    val spacing = node.getInt("spacing", 0)
    val widthProp = node.getString("width", "")
    val alignX = node.getString("align_x", "left")
    val alignY = node.getString("align_y", "center")
    val isWrap = node.getBoolean("wrap", false)
    val isHeaderBar = node.getBoolean("is_header_bar", false)
    val isSpaceBetween = alignX == "space_between" || (isHeaderBar && node.children.size == 2)
    val hasFlexChild = node.children.any {
        it.getString("width", "") == "fill"
            || (it.name == "spacer" && it.getInt("size", 0) == 0)
            || (it.name == "vstack" && it.getString("width", "") != "hug")
    }
    val isFill = widthProp == "fill" || (widthProp != "hug" && (hasFlexChild || isSpaceBetween))
    var rowMod = (if (widthProp == "hug") Modifier else modifier).then(Modifier.applyContainerStyle(node))
    if (isFill) {
        rowMod = rowMod.fillMaxWidth()
    }
    val vAlign = when (alignY) {
        "top" -> Alignment.Top
        "bottom" -> Alignment.Bottom
        else -> Alignment.CenterVertically
    }
    val hArrangement = when {
        isSpaceBetween -> Arrangement.SpaceBetween
        alignX == "center" -> Arrangement.spacedBy(spacing.dp, Alignment.CenterHorizontally)
        alignX == "right" -> Arrangement.spacedBy(spacing.dp, Alignment.End)
        else -> Arrangement.spacedBy(spacing.dp, Alignment.Start)
    }

    if (isWrap) {
        @OptIn(androidx.compose.foundation.layout.ExperimentalLayoutApi::class)
        androidx.compose.foundation.layout.FlowRow(
            modifier = rowMod,
            horizontalArrangement = hArrangement,
            verticalArrangement = Arrangement.spacedBy(spacing.dp)
        ) {
            node.children.forEach { child ->
                DynamicNodeView(child, dispatch, Modifier.widthIn(min = 0.dp))
            }
        }
    } else {
        Row(
            modifier = rowMod,
            horizontalArrangement = hArrangement,
            verticalAlignment = vAlign
        ) {
            node.children.forEach { child ->
                val childWidth = child.getString("width", "")
                val childIsFlex = childWidth == "fill"
                    || (child.name == "spacer" && child.getInt("size", 0) == 0)
                    || (child.name == "vstack" && childWidth != "hug")
                val childMod = if (childIsFlex) Modifier.weight(1f) else Modifier.widthIn(min = 0.dp)
                DynamicNodeView(child, dispatch, childMod)
            }
        }
    }
}
