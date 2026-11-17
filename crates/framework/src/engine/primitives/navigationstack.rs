
// ==========================================
// ⚠️ GENERATED CODE - DO NOT EDIT ⚠️
// ==========================================
#[allow(unused_imports)]
use crate::engine::core::*;
use crate::engine::ast::{Node, PropValue};
use crate::engine::builder::*;

pub struct NavigationStack {
    pub props: Vec<(String, PropValue)>,
    pub children: Vec<Node>,
}
impl NavigationStack {
    pub fn new() -> Self {
        Self {
            props: vec![],
            children: vec![],
        }
    }
    pub fn sheet1_count(mut self, sheet1_count: u32) -> Self {
        self.props.push(("sheet1_count".to_string(), PropValue::Int(sheet1_count)));
        self
    }
    pub fn has_subroutes(mut self, has_subroutes: bool) -> Self {
        self.props.push(("has_subroutes".to_string(), PropValue::Bool(has_subroutes)));
        self
    }
    pub fn top_style(mut self, top_style: impl Into<String>) -> Self {
        self.props.push(("top_style".to_string(), PropValue::String(top_style.into())));
        self
    }
    pub fn sheet2_count(mut self, sheet2_count: u32) -> Self {
        self.props.push(("sheet2_count".to_string(), PropValue::Int(sheet2_count)));
        self
    }
    pub fn window_count(mut self, window_count: u32) -> Self {
        self.props.push(("window_count".to_string(), PropValue::Int(window_count)));
        self
    }
    pub fn has_overlay(mut self, has_overlay: bool) -> Self {
        self.props.push(("has_overlay".to_string(), PropValue::Bool(has_overlay)));
        self
    }
    pub fn has_overlay2(mut self, has_overlay2: bool) -> Self {
        self.props.push(("has_overlay2".to_string(), PropValue::Bool(has_overlay2)));
        self
    }
    pub fn overlay_kind(mut self, overlay_kind: impl Into<String>) -> Self {
        self.props.push(("overlay_kind".to_string(), PropValue::String(overlay_kind.into())));
        self
    }
    pub fn current_path(mut self, current_path: impl Into<String>) -> Self {
        self.props.push(("current_path".to_string(), PropValue::String(current_path.into())));
        self
    }
    pub fn on_dismiss<H, Args, S>(mut self, handler: H) -> Self 
    where 
        H: crate::engine::registry::ActionHandler<Args, S>,
        S: 'static
    {
        let id = crate::engine::registry::register_handler(handler);
        self.props.push(("on_dismiss".to_string(), PropValue::String(id)));
        self
    }
    pub fn on_pop<H, Args, S>(mut self, handler: H) -> Self 
    where 
        H: crate::engine::registry::ActionHandler<Args, S>,
        S: 'static
    {
        let id = crate::engine::registry::register_handler(handler);
        self.props.push(("on_pop".to_string(), PropValue::String(id)));
        self
    }
    pub fn child(mut self, child: impl IntoNode) -> Self {
        self.children.push(child.into_node());
        self
    }
    pub fn children(mut self, children: impl IntoChildren) -> Self {
        self.children.extend(children.into_children());
        self
    }
}
impl IntoNode for NavigationStack {
    fn into_node(self) -> Node {
        Node::Element {
            name: "navigationstack".to_string(),
            props: self.props,
            children: self.children,
        }
    }
}

pub const NAVIGATIONSTACK_ANDROID_SNIPPET: &str = r#"
"navigationstack" -> {
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
"#;

pub const NAVIGATIONSTACK_IOS_SNIPPET: &str = r#"
case "navigationstack":
    let on_pop = node.props["on_pop"] as? String ?? ""
        let on_dismiss = node.props["on_dismiss"] as? String ?? ""
        DynamicNavigationStack(node: node, dispatch: dispatch, onPop: on_pop, onDismiss: on_dismiss)
"#;

pub const NAVIGATIONSTACK_GTK_SNIPPET: &str = r#"
"navigationstack" => {
    crate::build_main_navigation_stack(node, dispatch)
}
"#;

pub const NAVIGATIONSTACK_WEB_SNIPPET: &str = r#"
case "navigationstack": {
    return renderNavigationStack(node);
}
"#;

pub const NAVIGATIONSTACK_WINDOWS_SNIPPET: &str = r#"
case "navigationstack": {
    var navGrid = new Grid { HorizontalAlignment = HorizontalAlignment.Stretch, VerticalAlignment = VerticalAlignment.Stretch };
        if (node.Children != null && node.Children.Count > 0) {
            ExtractNavLayers(node, out var windowPages, out var modalLayers);
    
            if (windowPages.Count > 0) {
                var activeWin = windowPages[windowPages.Count - 1];
                lastNavDepth = windowPages.Count;
                lastPageTitle = (activeWin.Props != null && activeWin.Props.ContainsKey("title")) ? activeWin.Props["title"].ToString() : "";
                var pageEl = RenderNode(activeWin);
                navGrid.Children.Add(pageEl);
                AnimatePageIn(pageEl, true);
            }
    
            for (int i = 0; i < modalLayers.Count; i++) {
                var modalOverlay = BuildModalOverlay(modalLayers[i], i, true, false);
                navGrid.Children.Add(modalOverlay);
            }
        }
        return navGrid;
}
"#;
