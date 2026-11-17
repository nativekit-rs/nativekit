import androidx.compose.runtime.Composable

@Composable
fun PageView(node: DynamicUINode, dispatch: (String, org.json.JSONObject?) -> Unit, modifier: Modifier = Modifier) {
    val title = node.getString("title", "")
    val subtitle = node.getString("subtitle", "")
    val largeTitle = node.getBoolean("large_title", false)
    val topStyle = node.getString("top_style", "soft")
    val onBackStr = node.getString("on_back", "")
    val presentation = node.getString("presentation", "push")
    val isSheet = presentation == "sheet"
    val isForm = presentation == "form" || presentation == "dialog" || presentation == "modal" || presentation == "fullscreen"

    val toolbarNode = node.children.firstOrNull { it.name == "toolbartop" }
    val toolbarItems = toolbarNode?.children ?: emptyList()
    val leadingItems = toolbarItems.filter {
        (it.getString("placement", "").ifEmpty { it.children.firstOrNull()?.getString("placement", "") ?: "trailing" }) == "leading"
    }
    val trailingItems = toolbarItems.filter {
        (it.getString("placement", "").ifEmpty { it.children.firstOrNull()?.getString("placement", "") ?: "trailing" }) != "leading"
    }
    val bodyChildren = node.children.filter { it.name != "toolbartop" && it.name != "toolbarbottom" }
    val sectionSpacing = node.getInt("section_spacing", 16)

    if (isSheet) {
        androidx.compose.foundation.layout.Column(
            modifier = androidx.compose.ui.Modifier
                .fillMaxWidth()
                .padding(horizontal = 24.dp)
                .padding(bottom = 32.dp),
            verticalArrangement = androidx.compose.foundation.layout.Arrangement.spacedBy(sectionSpacing.dp)
        ) {
            bodyChildren.forEach { child ->
                DynamicNodeView(child, dispatch, androidx.compose.ui.Modifier.fillMaxWidth())
            }
        }
    } else {
        val titleBlock: @androidx.compose.runtime.Composable () -> Unit = {
            androidx.compose.foundation.layout.Column {
                androidx.compose.material3.Text(title)
                if (subtitle.isNotEmpty()) {
                    androidx.compose.material3.Text(subtitle, style = androidx.compose.material3.MaterialTheme.typography.bodySmall)
                }
            }
        }

        val navIconBlock: @androidx.compose.runtime.Composable () -> Unit = {
            if (leadingItems.isNotEmpty()) {
                androidx.compose.foundation.layout.Row(verticalAlignment = Alignment.CenterVertically) {
                    leadingItems.forEach { item -> DynamicNodeView(item, dispatch) }
                }
            } else if (onBackStr.isNotEmpty()) {
                androidx.compose.material3.IconButton(onClick = { dispatch(onBackStr, null) }) {
                    androidx.compose.material3.Icon(
                        imageVector = if (isForm) AppCloseIcon else AppBackIcon,
                        contentDescription = if (isForm) "Close" else "Back",
                        modifier = androidx.compose.ui.Modifier.size(24.dp)
                    )
                }
            }
        }

        val actionsBlock: @androidx.compose.runtime.Composable androidx.compose.foundation.layout.RowScope.() -> Unit = {
            trailingItems.forEach { item -> DynamicNodeView(item, dispatch) }
        }

        val isLarge = largeTitle && topStyle != "hard"
        val scrollBehavior = if (isLarge) {
            androidx.compose.material3.TopAppBarDefaults.exitUntilCollapsedScrollBehavior()
        } else {
            androidx.compose.material3.TopAppBarDefaults.pinnedScrollBehavior()
        }
        androidx.compose.material3.Scaffold(
            modifier = Modifier.nestedScroll(scrollBehavior.nestedScrollConnection),
            topBar = {
                if (isLarge) {
                    androidx.compose.material3.LargeTopAppBar(
                        title = titleBlock,
                        navigationIcon = navIconBlock,
                        actions = actionsBlock,
                        scrollBehavior = scrollBehavior
                    )
                } else {
                    androidx.compose.material3.TopAppBar(
                        title = titleBlock,
                        navigationIcon = navIconBlock,
                        actions = actionsBlock,
                        scrollBehavior = scrollBehavior
                    )
                }
            }
        ) { innerPadding ->
            androidx.compose.foundation.layout.Box(
                modifier = androidx.compose.ui.Modifier
                    .fillMaxSize()
                    .verticalScroll(androidx.compose.foundation.rememberScrollState())
                    .padding(innerPadding)
            ) {
                androidx.compose.foundation.layout.Column(
                    modifier = androidx.compose.ui.Modifier
                        .fillMaxWidth()
                        .padding(horizontal = 16.dp, vertical = 8.dp),
                    verticalArrangement = androidx.compose.foundation.layout.Arrangement.spacedBy(sectionSpacing.dp)
                ) {
                    bodyChildren.forEach { child ->
                        DynamicNodeView(child, dispatch, androidx.compose.ui.Modifier.fillMaxWidth())
                    }
                }
            }
        }
    }
}
