package com.nativeui.cargonativeapp

import androidx.compose.foundation.*
import androidx.compose.foundation.gestures.*
import androidx.compose.foundation.layout.*
import androidx.compose.material3.*
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.clip
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.graphics.asImageBitmap
import androidx.compose.ui.graphics.graphicsLayer
import androidx.compose.ui.input.pointer.pointerInput
import androidx.compose.ui.unit.dp
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.*
import androidx.compose.material.icons.rounded.*
import androidx.compose.material.icons.outlined.*
import androidx.compose.material.icons.sharp.*
import androidx.compose.ui.input.nestedscroll.nestedScroll
import kotlinx.coroutines.*

val runtimeThemeMode = androidx.compose.runtime.mutableStateOf("system")
val runtimeAccentMode = androidx.compose.runtime.mutableStateOf("app")

class DynamicUINode(
    val name: String,
    val props: Map<String, Any>,
    val children: List<DynamicUINode>,
    val inList: Boolean = false
) {
    fun getString(key: String, default: String = "") = props[key] as? String ?: default
    fun getBoolean(key: String, default: Boolean = false) = props[key] as? Boolean ?: default
    fun getBool(key: String, default: Boolean = false) = getBoolean(key, default)
    fun getInt(key: String, default: Int = 0) = props[key] as? Int ?: default
}

data class NavStackState(val depth: Int, val path: String, val page: DynamicUINode?)

fun parseDynamicNode(b64: String, existing: DynamicUINode? = null, inList: Boolean = false): DynamicUINode {
    try {
        val bytes = android.util.Base64.decode(b64, android.util.Base64.DEFAULT)
        val buf = java.nio.ByteBuffer.wrap(bytes).order(java.nio.ByteOrder.LITTLE_ENDIAN)
        
        fun readU8() = buf.get().toInt() and 0xFF
        fun readU16() = buf.short.toInt() and 0xFFFF
        fun readU32() = buf.int.toLong() and 0xFFFFFFFFL
        fun readStr(): String {
            val len = readU16()
            val b = ByteArray(len)
            buf.get(b)
            return String(b, Charsets.UTF_8)
        }
        
        fun decode(isList: Boolean): DynamicUINode {
            if (!buf.hasRemaining()) return DynamicUINode("error", mapOf(), listOf(), isList)
            val type = readU8()
            if (type == 1) {
                val name = readStr().lowercase()
                val propsCount = readU16()
                val props = mutableMapOf<String, Any>()
                for (i in 0 until propsCount) {
                    val k = readStr()
                    val vType = readU8()
                    if (vType == 0) props[k] = readStr()
                    else if (vType == 1) props[k] = readU8() != 0
                    else if (vType == 2) props[k] = readU32().toInt()
                    else if (vType == 3) props[k] = readStr()
                    else if (vType == 4) props[k] = readStr()
                    else if (vType == 5) {
                        val len = readU32().toInt()
                        val b = ByteArray(len)
                        buf.get(b)
                        try {
                            val s = String(b, Charsets.UTF_8).trim()
                            if (s.startsWith("[")) {
                                props[k] = org.json.JSONArray(s)
                            } else {
                                props[k] = org.json.JSONObject(s)
                            }
                        } catch (e: Exception) {}
                    }
                }
                val childrenCount = readU16()
                val children = mutableListOf<DynamicUINode>()
                val isNextList = name == "list"
                for (i in 0 until childrenCount) { children.add(decode(isNextList)) }
                return DynamicUINode(name, props, children, isList)
            } else {
                val len = readU32().toInt()
                buf.position(buf.position() + len)
                return DynamicUINode("raw", mapOf(), listOf(), isList)
            }
        }
        
        fun applyPatch(node: DynamicUINode, path: List<Int>, pathIdx: Int, type: Int): DynamicUINode {
            if (pathIdx == path.size) {
                if (type == 0) return decode(node.inList)
                else if (type == 1) {
                    val newProps = node.props.toMutableMap()
                    val propCount = readU16()
                    for (i in 0 until propCount) {
                        val k = readStr()
                        val vType = readU8()
                        if (vType == 0) newProps[k] = readStr()
                        else if (vType == 1) newProps[k] = readU8() != 0
                        else if (vType == 2) newProps[k] = readU32().toInt()
                        else if (vType == 3) newProps[k] = readStr()
                        else if (vType == 4) newProps[k] = readStr()
                        else if (vType == 5) {
                            val len = readU32().toInt()
                            val b = ByteArray(len); buf.get(b)
                            try {
                                val s = String(b, Charsets.UTF_8).trim()
                                if (s.startsWith("[")) {
                                    newProps[k] = org.json.JSONArray(s)
                                } else {
                                    newProps[k] = org.json.JSONObject(s)
                                }
                            } catch (e: Exception) {}
                        }
                    }
                    return DynamicUINode(node.name, newProps, node.children, node.inList)
                } else if (type == 2) {
                    val newChildren = node.children.toMutableList()
                    newChildren.add(decode(node.name == "list"))
                    return DynamicUINode(node.name, node.props, newChildren, node.inList)
                } else if (type == 3) {
                    val len = readU16()
                    var newChildren = node.children
                    if (len < newChildren.size) newChildren = newChildren.take(len)
                    return DynamicUINode(node.name, node.props, newChildren, node.inList)
                }
                return node
            }
            val childIdx = path[pathIdx]
            val newChildren = node.children.toMutableList()
            if (childIdx < newChildren.size) {
                newChildren[childIdx] = applyPatch(newChildren[childIdx], path, pathIdx + 1, type)
            }
            return DynamicUINode(node.name, node.props, newChildren, node.inList)
        }

        if (!buf.hasRemaining()) return DynamicUINode("error", mapOf(), listOf(), inList)
        val header = readU8()
        if (header == 0) {
            return decode(inList)
        } else if (header == 1) {
            if (existing == null) return DynamicUINode("error", mapOf(), listOf(), inList)
            val patchCount = readU16()
            var currentRoot = existing
            for (i in 0 until patchCount) {
                val type = readU8()
                val pathLen = readU16()
                val path = mutableListOf<Int>()
                for (j in 0 until pathLen) path.add(readU16())
                currentRoot = applyPatch(currentRoot!!, path, 0, type)
            }
            return currentRoot!!
        }
        return DynamicUINode("error", mapOf(), listOf(), inList)
    } catch (e: Exception) {
        return DynamicUINode("error", mapOf(), listOf(), inList)
    }
}





private val imageBitmapCache = android.util.LruCache<String, androidx.compose.ui.graphics.ImageBitmap>(64)

@Composable
fun DynamicNetworkOrAssetImage(
    src: String,
    widthDp: Int,
    heightDp: Int,
    cornerRadiusDp: Int,
    isContain: Boolean,
    opacity: Float,
    modifier: Modifier = Modifier
) {
    val ctx = androidx.compose.ui.platform.LocalContext.current
    val resolvedUrl = if (src.startsWith("http://") || src.startsWith("https://")) src else "http://10.0.2.2:5824/" + src.removePrefix("./").trimStart('/')
    var bmp by remember(src) { mutableStateOf(imageBitmapCache.get(src)) }
    LaunchedEffect(src) {
        if (bmp == null) {
            kotlinx.coroutines.withContext(kotlinx.coroutines.Dispatchers.IO) {
                try {
                    val stem = src.substringAfterLast('/').substringBeforeLast('.').lowercase().replace('-', '_')
                    val resId = if (!src.startsWith("http")) ctx.resources.getIdentifier(stem, "drawable", ctx.packageName) else 0
                    if (resId != 0) {
                        val decoded = android.graphics.BitmapFactory.decodeResource(ctx.resources, resId)
                        if (decoded != null) {
                            val composeBmp = decoded.asImageBitmap()
                            imageBitmapCache.put(src, composeBmp)
                            bmp = composeBmp
                            return@withContext
                        }
                    }
                    val conn = java.net.URL(resolvedUrl).openConnection() as java.net.HttpURLConnection
                    conn.connectTimeout = 5000
                    conn.readTimeout = 5000
                    conn.inputStream.use { stream ->
                        val decoded = android.graphics.BitmapFactory.decodeStream(stream)
                        if (decoded != null) {
                            val composeBmp = decoded.asImageBitmap()
                            imageBitmapCache.put(src, composeBmp)
                            bmp = composeBmp
                        }
                    }
                } catch (_: Exception) {}
            }
        }
    }
    var imgMod = modifier
    imgMod = if (widthDp > 0) imgMod.width(widthDp.dp) else imgMod.fillMaxWidth()
    if (heightDp > 0) imgMod = imgMod.height(heightDp.dp)
    if (cornerRadiusDp > 0) {
        imgMod = imgMod.clip(androidx.compose.foundation.shape.RoundedCornerShape(cornerRadiusDp.dp))
    }
    if (opacity < 1f) {
        imgMod = imgMod.graphicsLayer(alpha = opacity)
    }
    if (bmp != null) {
        androidx.compose.foundation.Image(
            bitmap = bmp!!,
            contentDescription = src,
            contentScale = if (isContain) androidx.compose.ui.layout.ContentScale.Fit else androidx.compose.ui.layout.ContentScale.Crop,
            modifier = imgMod
        )
    } else {
        androidx.compose.foundation.layout.Box(
            modifier = imgMod.background(MaterialTheme.colorScheme.surfaceVariant),
            contentAlignment = Alignment.Center
        ) {
            CircularProgressIndicator(modifier = Modifier.size(24.dp), strokeWidth = 2.dp)
        }
    }
}

@Composable
fun DynamicNodeView(node: DynamicUINode, dispatch: (String, org.json.JSONObject?) -> Unit, modifier: Modifier = Modifier) {
    when (node.name) {
"text", "icon", "button", "hstack", "vstack", "switch", "navigationstack", "page" -> RenderGeneratedPrimitive(node, dispatch, modifier)
"tabbar" -> {
            runtimeThemeMode.value = node.getString("theme_mode", "system")
            runtimeAccentMode.value = node.getString("accent_mode", "app")
            val activeIndex = ((node.props["active_index"] as? Number)?.toInt() ?: 0)
            val tabsList = mutableListOf<Map<String, String>>()
            val rawTabs = node.props["tabs"]
            if (rawTabs is org.json.JSONArray) {
                for (i in 0 until rawTabs.length()) {
                    val obj = rawTabs.optJSONObject(i)
                    if (obj != null) {
                        tabsList.add(mapOf(
                            "title" to obj.optString("title", ""),
                            "icon" to obj.optString("android_icon", obj.optString("icon", "")),
                            "action" to obj.optString("action", "")
                        ))
                    }
                }
            } else if (rawTabs is List<*>) {
                for (item in rawTabs) {
                    if (item is Map<*, *>) {
                        tabsList.add(mapOf(
                            "title" to (item["title"]?.toString() ?: ""),
                            "icon" to (item["icon"]?.toString() ?: ""),
                            "action" to (item["action"]?.toString() ?: "")
                        ))
                    }
                }
            }

            val activeChild = node.children.firstOrNull()
            val isDrilledIn = activeChild?.let { child ->
                val wc = (child.props["window_count"] as? Number)?.toInt() ?: child.getInt("window_count") ?: 1
                val hasOverlay = child.getBoolean("has_overlay", false)
                wc > 1 || hasOverlay
            } ?: false

            androidx.compose.material3.Scaffold(
                contentWindowInsets = androidx.compose.foundation.layout.WindowInsets(0, 0, 0, 0),
                bottomBar = {
                    if (tabsList.isNotEmpty() && !isDrilledIn) {
                        androidx.compose.material3.NavigationBar {
                            for (idx in tabsList.indices) {
                                val tab = tabsList[idx]
                                val title = tab["title"] ?: ""
                                val iconName = tab["icon"] ?: ""
                                val action = tab["action"] ?: ""
                                NavigationBarItem(
                                    selected = (idx == activeIndex),
                                    onClick = {
                                        if (action.isNotEmpty()) dispatch(action, null)
                                    },
                                    icon = {
                                        Icon(imageVector = appTabIcon(iconName), contentDescription = title)
                                    },
                                    label = { Text(title) }
                                )
                            }
                        }
                    }
                }
            ) { paddingValues ->
                androidx.compose.foundation.layout.Box(
                    modifier = androidx.compose.ui.Modifier
                        .fillMaxSize()
                        .padding(paddingValues)
                ) {
                    if (node.children.isNotEmpty()) {
                        DynamicNodeView(node.children[0], dispatch, androidx.compose.ui.Modifier.fillMaxSize())
                    }
                }
            }
        }
        "if" -> {
            val cond = node.getBoolean("condition", false)
            if (cond) {
                Column(modifier = modifier) {
                    node.children.forEach { child -> DynamicNodeView(child, dispatch, Modifier.fillMaxWidth()) }
                }
            }
        }
        else -> RenderGeneratedPrimitive(node, dispatch, modifier)
    }
}





















interface RustEngineLibrary : com.sun.jna.Library {
    fun init_app(): com.sun.jna.Pointer?
    fun get_ui(app_ptr: com.sun.jna.Pointer?): String?
    fun dispatch_event(app_ptr: com.sun.jna.Pointer?, action: String?, payload: String?): String?
    fun register_refresh_callback(callback: RefreshCallback)
}
interface RefreshCallback : com.sun.jna.Callback { fun invoke() }
private val RustLib: RustEngineLibrary? = try { com.sun.jna.Native.load("demo_app", RustEngineLibrary::class.java) } catch (e: Throwable) { null }
private var activeRefreshCallback: RefreshCallback? = null
private val dispatchExecutor = java.util.concurrent.Executors.newSingleThreadExecutor()
private val mainHandler = android.os.Handler(android.os.Looper.getMainLooper())
private val lastAppliedTs = java.util.concurrent.atomic.AtomicLong(0L)

@androidx.compose.runtime.Composable
fun EngineView() {
    val isProdFfi = RustLib != null
    var appPtr by androidx.compose.runtime.remember {
        androidx.compose.runtime.mutableStateOf(if (isProdFfi) RustLib!!.init_app() else null)
    }
    var liveUiNode by androidx.compose.runtime.remember {
        val initialNode = if (isProdFfi && appPtr != null) {
            RustLib!!.get_ui(appPtr)?.let { b64 -> parseDynamicNode(b64, null) }
        } else null
        androidx.compose.runtime.mutableStateOf<DynamicUINode?>(initialNode)
    }

    val fetchUI = {
        dispatchExecutor.submit {
            try {
                val url = java.net.URL("http://10.0.2.2:5824/api/ui_layout")
                val conn = url.openConnection() as java.net.HttpURLConnection
                conn.connectTimeout = 1000
                conn.readTimeout = 1000
                if (conn.responseCode == 200) {
                    val hdrTs = conn.getHeaderField("X-Reload-Timestamp")?.toLongOrNull()
                    if (hdrTs != null) lastAppliedTs.set(hdrTs)
                    val b64 = conn.inputStream.bufferedReader().use { it.readText() }
                    val parsed = parseDynamicNode(b64, liveUiNode)
                    mainHandler.post {
                        liveUiNode = parsed
                    }
                }
            } catch (e: Exception) {}
        }
    }

    androidx.compose.runtime.LaunchedEffect(Unit) {
        if (isProdFfi) {
            kotlinx.coroutines.withContext(kotlinx.coroutines.Dispatchers.IO) {
                val lib = RustLib!!
                val ptr = appPtr ?: lib.init_app()
                val cb = object : RefreshCallback {
                    override fun invoke() {
                        dispatchExecutor.submit {
                            lib.get_ui(ptr)?.let { b64 ->
                                val parsed = parseDynamicNode(b64, liveUiNode)
                                mainHandler.post { liveUiNode = parsed }
                            }
                        }
                    }
                }
                activeRefreshCallback = cb
                lib.register_refresh_callback(cb)
                lib.get_ui(ptr)?.let { b64 ->
                    val parsed = parseDynamicNode(b64, liveUiNode)
                    mainHandler.post { appPtr = ptr; liveUiNode = parsed }
                }
            }
            return@LaunchedEffect
        }
        fetchUI()
        kotlinx.coroutines.withContext(kotlinx.coroutines.Dispatchers.IO) {
            while (true) {
                try {
                    val waitTs = lastAppliedTs.get()
                    val url = java.net.URL("http://10.0.2.2:5824/api/live_reload?wait_for=${waitTs}")
                    val conn = url.openConnection() as java.net.HttpURLConnection
                    conn.connectTimeout = 2000
                    conn.readTimeout = 20000
                    if (conn.responseCode == 200) {
                        val jsonStr = conn.inputStream.bufferedReader().use { it.readText() }
                        val json = org.json.JSONObject(jsonStr)
                        val ts = json.getLong("timestamp")
                        if (ts > lastAppliedTs.get()) {
                            lastAppliedTs.set(ts)
                            fetchUI()
                        } else {
                            kotlinx.coroutines.delay(150)
                        }
                    } else {
                        kotlinx.coroutines.delay(250)
                    }
                } catch (e: Exception) {
                    kotlinx.coroutines.delay(250)
                }
            }
        }
    }

    val dispatchEvent: (String, org.json.JSONObject?) -> Unit = { action, payload ->
        if (action.isNotEmpty()) {
            dispatchExecutor.submit {
                if (isProdFfi) {
                    appPtr?.let { ptr ->
                        RustLib!!.dispatch_event(ptr, action, payload?.toString())?.let { b64 ->
                            val parsed = parseDynamicNode(b64, liveUiNode)
                            mainHandler.post { liveUiNode = parsed }
                        }
                    }
                    return@submit
                }
                try {
                    val url = java.net.URL("http://10.0.2.2:5824/api/dispatch")
                    val conn = url.openConnection() as java.net.HttpURLConnection
                    conn.requestMethod = "POST"
                    conn.doOutput = true
                    conn.setRequestProperty("Content-Type", "application/json")
                    conn.connectTimeout = 1000
                    conn.readTimeout = 1000

                    val body = org.json.JSONObject()
                    body.put("action", action)
                    body.put("payload", payload ?: org.json.JSONObject())

                    conn.outputStream.write(body.toString().toByteArray())

                    if (conn.responseCode == 200) {
                        val hdrTs = conn.getHeaderField("X-Reload-Timestamp")?.toLongOrNull()
                        if (hdrTs != null) lastAppliedTs.set(hdrTs)
                        val b64 = conn.inputStream.bufferedReader().use { it.readText() }
                        val parsed = parseDynamicNode(b64, liveUiNode)
                        mainHandler.post {
                            liveUiNode = parsed
                        }
                    }
                } catch (e: Exception) {}
            }
        }
    }

    val node = liveUiNode
    if (node != null) {
        DynamicNodeView(node, dispatchEvent, Modifier.fillMaxSize())
    } else {
        androidx.compose.foundation.layout.Box(modifier = Modifier.fillMaxSize()) {
            Text("Connecting to Rust Dev Server...", modifier = Modifier.align(Alignment.Center))
        }
    }
}
