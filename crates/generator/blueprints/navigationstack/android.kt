import androidx.compose.runtime.Composable

@Composable
fun NavigationStackView(node: DynamicUINode, dispatch: (String, org.json.JSONObject?) -> Unit, modifier: Modifier = Modifier) {
    val onPop = node.getString("on_pop")
    val onDismissStr = node.getString("on_dismiss")
    val onDismiss = if (onDismissStr.isNotEmpty()) onDismissStr else onPop
    val hasOverlay = node.getBool("has_overlay")
    val overlayKind = node.getString("overlay_kind", "sheet")
    val isForm = overlayKind == "form" || overlayKind == "dialog" || overlayKind == "modal" || overlayKind == "fullscreen"
    val windowCount = ((node.props["window_count"] as? Int) ?: if (hasOverlay) maxOf(1, node.children.size - 1) else node.children.size).coerceIn(0, node.children.size)

    val windowPages = node.children.take(windowCount)
    val overlayPages = node.children.drop(windowCount)
    val lastForm = androidx.compose.runtime.remember { androidx.compose.runtime.mutableStateOf<DynamicUINode?>(null) }
    val formShown = androidx.compose.runtime.remember { androidx.compose.runtime.mutableStateOf(false) }
    val sheetState = androidx.compose.material3.rememberModalBottomSheetState()
    val lastSheet = androidx.compose.runtime.remember { androidx.compose.runtime.mutableStateOf<DynamicUINode?>(null) }
    val sheetShown = androidx.compose.runtime.remember { androidx.compose.runtime.mutableStateOf(false) }

    androidx.activity.compose.BackHandler(enabled = windowPages.size > 1 || (hasOverlay && isForm) || formShown.value || sheetShown.value) {
        if (sheetShown.value) {
            if (onDismiss.isNotEmpty()) dispatch(onDismiss, null)
        } else if (hasOverlay || formShown.value) {
            if (onDismiss.isNotEmpty()) dispatch(onDismiss, null)
        } else if (onPop.isNotEmpty()) {
            dispatch(onPop, null)
        }
    }

    if (hasOverlay && isForm && overlayPages.isNotEmpty()) {
        lastForm.value = overlayPages.first()
    }
    if (hasOverlay && !isForm && overlayPages.isNotEmpty()) {
        lastSheet.value = overlayPages.first()
    }

    androidx.compose.runtime.LaunchedEffect(hasOverlay, isForm, overlayPages.size) {
        formShown.value = hasOverlay && isForm && overlayPages.isNotEmpty()
        if (hasOverlay && !isForm && overlayPages.isNotEmpty()) {
            sheetShown.value = true
        } else if (sheetShown.value) {
            try {
                sheetState.hide()
            } catch (_: Throwable) {
            } finally {
                sheetShown.value = false
            }
        }
    }

    val pageKey = windowPages.lastOrNull()?.getString("title") ?: windowPages.lastOrNull()?.name ?: ""
    val navState = NavStackState(windowPages.size, pageKey, windowPages.lastOrNull())

    androidx.compose.foundation.layout.Box(modifier = androidx.compose.ui.Modifier.fillMaxSize()) {
        androidx.compose.animation.AnimatedContent(
            targetState = navState,
            contentKey = { "${it.depth}_${it.path}" },
            transitionSpec = {
                if (initialState.depth == 0 || targetState.depth == initialState.depth) {
                    androidx.compose.animation.ContentTransform(
                        targetContentEnter = androidx.compose.animation.EnterTransition.None,
                        initialContentExit = androidx.compose.animation.ExitTransition.None
                    )
                } else if (targetState.depth > initialState.depth) {
                    androidx.compose.animation.ContentTransform(
                        targetContentEnter = androidx.compose.animation.slideInHorizontally(
                            animationSpec = androidx.compose.animation.core.tween(300),
                            initialOffsetX = { width -> width }
                        ) + androidx.compose.animation.fadeIn(animationSpec = androidx.compose.animation.core.tween(250)),
                        initialContentExit = androidx.compose.animation.slideOutHorizontally(
                            animationSpec = androidx.compose.animation.core.tween(300),
                            targetOffsetX = { width -> -width / 3 }
                        ) + androidx.compose.animation.fadeOut(animationSpec = androidx.compose.animation.core.tween(250)),
                        targetContentZIndex = 1f
                    )
                } else {
                    androidx.compose.animation.ContentTransform(
                        targetContentEnter = androidx.compose.animation.slideInHorizontally(
                            animationSpec = androidx.compose.animation.core.tween(300),
                            initialOffsetX = { width -> -width / 3 }
                        ) + androidx.compose.animation.fadeIn(animationSpec = androidx.compose.animation.core.tween(250)),
                        initialContentExit = androidx.compose.animation.slideOutHorizontally(
                            animationSpec = androidx.compose.animation.core.tween(300),
                            targetOffsetX = { width -> width }
                        ) + androidx.compose.animation.fadeOut(animationSpec = androidx.compose.animation.core.tween(250)),
                        targetContentZIndex = -1f
                    )
                }
            },
            label = "NavStack"
        ) { state ->
            val currentPage = if (state.depth == navState.depth && state.path == navState.path) {
                windowPages.lastOrNull() ?: state.page
            } else {
                windowPages.getOrNull(state.depth - 1) ?: state.page
            }
            currentPage?.let {
                DynamicNodeView(it, dispatch, androidx.compose.ui.Modifier.fillMaxSize())
            }
        }

        androidx.compose.animation.AnimatedVisibility(
            visible = formShown.value,
            enter = androidx.compose.animation.slideInVertically(
                animationSpec = androidx.compose.animation.core.tween(350, easing = androidx.compose.animation.core.FastOutSlowInEasing),
                initialOffsetY = { fullHeight -> fullHeight }
            ) + androidx.compose.animation.fadeIn(animationSpec = androidx.compose.animation.core.tween(250)),
            exit = androidx.compose.animation.slideOutVertically(
                animationSpec = androidx.compose.animation.core.tween(300, easing = androidx.compose.animation.core.FastOutLinearInEasing),
                targetOffsetY = { fullHeight -> fullHeight }
            ) + androidx.compose.animation.fadeOut(animationSpec = androidx.compose.animation.core.tween(200))
        ) {
            androidx.compose.material3.Surface(
                modifier = androidx.compose.ui.Modifier.fillMaxSize(),
                color = androidx.compose.material3.MaterialTheme.colorScheme.background
            ) {
                lastForm.value?.let {
                    DynamicNodeView(it, dispatch, androidx.compose.ui.Modifier.fillMaxSize())
                }
            }
        }

        if (sheetShown.value && lastSheet.value != null) {
            androidx.compose.material3.ModalBottomSheet(
                sheetState = sheetState,
                onDismissRequest = {
                    sheetShown.value = false
                    if (hasOverlay && onDismiss.isNotEmpty()) {
                        dispatch(onDismiss, null)
                    }
                }
            ) {
                DynamicNodeView(lastSheet.value!!, dispatch, androidx.compose.ui.Modifier.fillMaxWidth())
            }
        }
    }
}
