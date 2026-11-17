import androidx.compose.runtime.Composable

@Composable
fun VStackView(node: DynamicUINode, dispatch: (String, org.json.JSONObject?) -> Unit, modifier: Modifier = Modifier) {
    val spacing = node.getInt("spacing", 0)
    val widthProp = node.getString("width", "")
    val alignX = node.getString("align_x", "left")
    val alignY = node.getString("align_y", "top")
    val bgImg = node.getString("background_image", "")
    val radius = node.getInt("corner_radius", 0)
    val pad = node.getInt("padding", 0)
    val isHug = widthProp == "hug"
    var colMod = (if (isHug) Modifier else modifier).then(Modifier.applyContainerStyle(node))
    if (!isHug && (widthProp == "fill" || modifier == Modifier)) {
        colMod = colMod.fillMaxWidth()
    }
    val hAlign = when (alignX) {
        "center" -> Alignment.CenterHorizontally
        "right" -> Alignment.End
        else -> Alignment.Start
    }
    val vArrangement = when (alignY) {
        "space_between" -> Arrangement.SpaceBetween
        "center" -> Arrangement.spacedBy(spacing.dp, Alignment.CenterVertically)
        "bottom" -> Arrangement.spacedBy(spacing.dp, Alignment.Bottom)
        else -> Arrangement.spacedBy(spacing.dp, Alignment.Top)
    }
    if (bgImg.isNotEmpty()) {
        var boxMod = if (!isHug && (widthProp == "fill" || modifier == Modifier)) modifier.fillMaxWidth() else (if (isHug) Modifier else modifier)
        if (radius > 0) boxMod = boxMod.clip(androidx.compose.foundation.shape.RoundedCornerShape(radius.dp))
        androidx.compose.foundation.layout.Box(modifier = boxMod) {
            DynamicNetworkOrAssetImage(
                src = bgImg, widthDp = 0, heightDp = 0, cornerRadiusDp = radius,
                isContain = false, opacity = 1f, modifier = Modifier.matchParentSize()
            )
            Column(
                modifier = (if (isHug) Modifier else Modifier.fillMaxWidth()).padding(pad.dp),
                verticalArrangement = vArrangement,
                horizontalAlignment = hAlign
            ) {
                node.children.forEach { child -> DynamicNodeView(child, dispatch) }
            }
        }
    } else {
        Column(
            modifier = colMod,
            verticalArrangement = vArrangement,
            horizontalAlignment = hAlign
        ) {
            node.children.forEach { child -> DynamicNodeView(child, dispatch) }
        }
    }
}
