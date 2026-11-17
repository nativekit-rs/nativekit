import androidx.compose.runtime.Composable

@Composable
fun ImageView(node: DynamicUINode, dispatch: (String, org.json.JSONObject?) -> Unit, modifier: Modifier = Modifier) {
    val src = node.getString("src", "")
    val size = node.getInt("size", 0)
    val w = node.getInt("width", size)
    val h = node.getInt("height", if (size > 0) size else 180)
    val radius = node.getInt("corner_radius", 0)
    val mode = node.getString("content_mode", "cover")
    val opacity = node.getInt("opacity", 100).toFloat() / 100f
    DynamicNetworkOrAssetImage(
        src = src,
        widthDp = w,
        heightDp = h,
        cornerRadiusDp = radius,
        isContain = (mode == "contain" || mode == "fit"),
        opacity = opacity,
        modifier = modifier
    )
}
