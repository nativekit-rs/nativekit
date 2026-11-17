import androidx.compose.runtime.Composable

@Composable
fun TextInputView(node: DynamicUINode, dispatch: (String, org.json.JSONObject?) -> Unit, modifier: Modifier = Modifier) {
    val externalValue = node.getString("value", "")
    val placeholder = node.getString("placeholder", "")
    val onChangeStr = node.getString("on_change", "")
    var localText by androidx.compose.runtime.remember { androidx.compose.runtime.mutableStateOf(externalValue) }
    var lastSent by androidx.compose.runtime.remember { androidx.compose.runtime.mutableStateOf(externalValue) }
    var isFocused by androidx.compose.runtime.remember { androidx.compose.runtime.mutableStateOf(false) }

    androidx.compose.runtime.LaunchedEffect(externalValue, isFocused) {
        if (!isFocused && externalValue != localText) {
            localText = externalValue
            lastSent = externalValue
        }
    }

    OutlinedTextField(
        value = localText,
        onValueChange = { newText ->
            localText = newText
            if (onChangeStr.isNotEmpty() && newText != lastSent) {
                lastSent = newText
                dispatch(onChangeStr, org.json.JSONObject().put("value", newText))
            }
        },
        placeholder = { Text(placeholder) },
        singleLine = true,
        shape = androidx.compose.foundation.shape.RoundedCornerShape(10.dp),
        modifier = modifier
            .fillMaxWidth()
            .onFocusChanged { isFocused = it.isFocused }
    )
}
