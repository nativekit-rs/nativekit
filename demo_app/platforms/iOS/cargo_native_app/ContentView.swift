import SwiftUI

var runtimeAccentMode: String = "app"
var runtimePrimaryHex: String = ""
var runtimeSecondaryHex: String = ""

extension Data {
    func readU8(at offset: inout Int) -> UInt8 { 
        guard offset < self.count else { return 0 }
        let v = self[offset]; offset += 1; return v 
    }
    func readU16(at offset: inout Int) -> UInt16 { 
        guard offset + 1 < self.count else { return 0 }
        let b0 = UInt16(self[offset])
        let b1 = UInt16(self[offset+1])
        offset += 2; return b0 | (b1 << 8)
    }
    func readU32(at offset: inout Int) -> UInt32 { 
        guard offset + 3 < self.count else { return 0 }
        let b0 = UInt32(self[offset])
        let b1 = UInt32(self[offset+1])
        let b2 = UInt32(self[offset+2])
        let b3 = UInt32(self[offset+3])
        offset += 4; return b0 | (b1 << 8) | (b2 << 16) | (b3 << 24)
    }
    func readString(at offset: inout Int) -> String {
        let len = Int(readU16(at: &offset))
        guard offset + len <= self.count else { return "" }
        let str = String(data: self.subdata(in: offset..<(offset+len)), encoding: .utf8) ?? ""
        offset += len; return str
    }
}

enum AppScrollEdgeEffectStyle {
    case hard
    case soft
    case automatic
}

extension View {
    @ViewBuilder
    func scrollEdgeEffect(_ edge: Edge.Set, _ style: AppScrollEdgeEffectStyle) -> some View {
        if #available(iOS 26.0, *) {
            let nativeStyle: ScrollEdgeEffectStyle = {
                switch style {
                case .hard: return .hard
                case .soft: return .soft
                case .automatic: return .automatic
                }
            }()
            self.scrollEdgeEffectStyle(nativeStyle, for: edge)
        } else {
            self
        }
    }
}

class DynamicUINode {
    let name: String
    let props: [String: Any]
    let children: [DynamicUINode]
    
    init(name: String, props: [String: Any], children: [DynamicUINode]) {
        self.name = name; self.props = props; self.children = children
    }
    
    convenience init(b64: String, existing: DynamicUINode? = nil) {
        guard let data = Data(base64Encoded: b64, options: .ignoreUnknownCharacters) else {
            self.init(name: "error", props: [:], children: [])
            return
        }
        var offset = 0
        guard offset < data.count else {
            self.init(name: "error", props: [:], children: [])
            return
        }
        let header = data.readU8(at: &offset)
        if header == 0 {
            let decoded = DynamicUINode.decode(from: data, offset: &offset)
            self.init(name: decoded.name, props: decoded.props, children: decoded.children)
        } else if header == 1 {
            let patchCount = Int(data.readU16(at: &offset))
            if let existing = existing {
                let patched = existing.applying(patches: data, offset: &offset, numPatches: patchCount)
                self.init(name: patched.name, props: patched.props, children: patched.children)
            } else {
                self.init(name: "error", props: [:], children: [])
            }
        } else {
            self.init(name: "error", props: [:], children: [])
        }
    }
    
    func applying(patches data: Data, offset: inout Int, numPatches: Int) -> DynamicUINode {
        var currentRoot = DynamicUINode(name: self.name, props: self.props, children: self.children)
        for _ in 0..<numPatches {
            let type = data.readU8(at: &offset)
            let pathLen = Int(data.readU16(at: &offset))
            var path = [Int]()
            for _ in 0..<pathLen { path.append(Int(data.readU16(at: &offset))) }
            currentRoot = currentRoot.applyPatch(path: path, pathIdx: 0, type: type, data: data, offset: &offset)
        }
        return currentRoot
    }
    
    private func applyPatch(path: [Int], pathIdx: Int, type: UInt8, data: Data, offset: inout Int) -> DynamicUINode {
        if pathIdx == path.count {
            if type == 0 { return DynamicUINode.decode(from: data, offset: &offset) }
            else if type == 1 {
                var newProps = self.props
                let propCount = Int(data.readU16(at: &offset))
                for _ in 0..<propCount {
                    let k = data.readString(at: &offset)
                    let vType = data.readU8(at: &offset)
                    if vType == 0 { newProps[k] = data.readString(at: &offset) }
                    else if vType == 1 { newProps[k] = data.readU8(at: &offset) != 0 }
                    else if vType == 2 { newProps[k] = Int(data.readU32(at: &offset)) }
                    else if vType == 3 { newProps[k] = data.readString(at: &offset) }
                    else if vType == 4 { newProps[k] = data.readString(at: &offset) }
                    else if vType == 5 {
                        let len = Int(data.readU32(at: &offset))
                        if let j = try? JSONSerialization.jsonObject(with: data.subdata(in: offset..<(offset+len))) { newProps[k] = j }
                        offset += len
                    }
                }
                return DynamicUINode(name: self.name, props: newProps, children: self.children)
            } else if type == 2 {
                var newChildren = self.children
                newChildren.append(DynamicUINode.decode(from: data, offset: &offset))
                return DynamicUINode(name: self.name, props: self.props, children: newChildren)
            } else if type == 3 {
                let len = Int(data.readU16(at: &offset))
                var newChildren = self.children
                if len < newChildren.count { newChildren.removeLast(newChildren.count - len) }
                return DynamicUINode(name: self.name, props: self.props, children: newChildren)
            }
            return self
        }
        let childIdx = path[pathIdx]
        var newChildren = self.children
        if childIdx < newChildren.count {
            newChildren[childIdx] = newChildren[childIdx].applyPatch(path: path, pathIdx: pathIdx + 1, type: type, data: data, offset: &offset)
        }
        return DynamicUINode(name: self.name, props: self.props, children: newChildren)
    }

    static func decode(from data: Data, offset: inout Int) -> DynamicUINode {
        guard offset < data.count else { return DynamicUINode(name: "error", props: [:], children: []) }
        let type = data.readU8(at: &offset)
        if type == 1 {
            let name = data.readString(at: &offset).lowercased()
            let propsCount = Int(data.readU16(at: &offset))
            var props = [String: Any]()
            for _ in 0..<propsCount {
                let k = data.readString(at: &offset)
                let vType = data.readU8(at: &offset)
                if vType == 0 { props[k] = data.readString(at: &offset) }
                else if vType == 1 { props[k] = data.readU8(at: &offset) != 0 }
                else if vType == 2 { props[k] = Int(data.readU32(at: &offset)) }
                else if vType == 3 { props[k] = data.readString(at: &offset) }
                else if vType == 4 { props[k] = data.readString(at: &offset) }
                else if vType == 5 {
                    let len = Int(data.readU32(at: &offset))
                    if offset + len <= data.count {
                        if let j = try? JSONSerialization.jsonObject(with: data.subdata(in: offset..<(offset+len))) {
                            props[k] = j
                        }
                    }
                    offset += len
                }
            }
            let childrenCount = Int(data.readU16(at: &offset))
            var children = [DynamicUINode]()
            for _ in 0..<childrenCount {
                children.append(decode(from: data, offset: &offset))
            }
            return DynamicUINode(name: name, props: props, children: children)
        } else {
            let len = Int(data.readU32(at: &offset))
            offset += len
            return DynamicUINode(name: "raw", props: [:], children: [])
        }
    }
}

struct RetainedDestinationView: View {
    let liveNode: DynamicUINode?
    let hideTabBar: Bool
    let dispatch: (String, [String: Any]?) -> Void
    @State private var cachedNode: DynamicUINode?

    var body: some View {
        if let node = liveNode ?? cachedNode {
            DynamicNodeView(node: node, dispatch: dispatch)
                .toolbar(hideTabBar ? .hidden : .automatic, for: .tabBar)
                .onAppear {
                    if let liveNode = liveNode {
                        cachedNode = liveNode
                    }
                }
                .onChange(of: liveNode != nil) { _, hasLive in
                    if hasLive, let liveNode = liveNode {
                        cachedNode = liveNode
                    }
                }
        }
    }
}

struct DynamicNavigationStack: View {
    let node: DynamicUINode
    let dispatch: (String, [String: Any]?) -> Void
    let onPop: String
    let onDismiss: String

    @State private var windowPath: [Int] = []
    @State private var overlayPath: [Int] = []
    @State private var showingSheet: Bool = false
    @State private var isSyncingWindow: Bool = false
    @State private var isSyncingOverlay: Bool = false
    @State private var syncedWindowDepth: Int = 0
    @State private var syncedOverlayDepth: Int = 0

    var windowCount: Int {
        if let wc = node.props["window_count"] as? Int {
            return wc
        }
        return hasOverlay ? max(1, node.children.count - 1) : node.children.count
    }

    var hasOverlay: Bool {
        if let h = node.props["has_overlay"] as? Bool {
            return h
        }
        guard node.children.count > 1, let top = node.children.last else { return false }
        return (top.props["presentation"] as? String) == "sheet"
    }

    var windowPages: [DynamicUINode] {
        let count = min(windowCount, node.children.count)
        return count > 0 ? Array(node.children[0..<count]) : []
    }

    var overlayPages: [DynamicUINode] {
        let count = min(windowCount, node.children.count)
        return node.children.count > count ? Array(node.children[count...]) : []
    }

    var body: some View {
        NavigationStack(path: $windowPath) {
            if !windowPages.isEmpty {
                DynamicNodeView(node: windowPages[0], dispatch: dispatch)
                    .toolbar(.visible, for: .tabBar)
                    .navigationDestination(for: Int.self) { index in
                        RetainedDestinationView(
                            liveNode: index < windowPages.count ? windowPages[index] : nil,
                            hideTabBar: true,
                            dispatch: dispatch
                        )
                    }
            }
        }
        .sheet(isPresented: $showingSheet) {
            let kind = (node.props["overlay_kind"] as? String) ?? "sheet"
            let isForm = (kind == "form" || kind == "dialog" || kind == "modal" || kind == "fullscreen")
            Group {
                if let overlay = overlayPages.first {
                    if overlay.name == "navigationstack" {
                        DynamicNodeView(node: overlay, dispatch: dispatch)
                    } else {
                        NavigationStack(path: $overlayPath) {
                            DynamicNodeView(node: overlay, dispatch: dispatch)
                                .navigationDestination(for: Int.self) { index in
                                    RetainedDestinationView(
                                        liveNode: index < overlayPages.count ? overlayPages[index] : nil,
                                        hideTabBar: false,
                                        dispatch: dispatch
                                    )
                                }
                        }
                    }
                }
            }
            .presentationDetents(isForm ? [.large] : [.medium, .large])
            .presentationDragIndicator(.visible)
        }
        .onAppear {
            let winTarget = max(0, windowPages.count - 1)
            syncedWindowDepth = winTarget
            if windowPath.count != winTarget {
                isSyncingWindow = true
                windowPath = winTarget > 0 ? Array(1..<windowPages.count) : []
                DispatchQueue.main.async { isSyncingWindow = false }
            }
            let ovTarget = max(0, overlayPages.count - 1)
            syncedOverlayDepth = ovTarget
            if overlayPath.count != ovTarget {
                isSyncingOverlay = true
                overlayPath = ovTarget > 0 ? Array(1..<overlayPages.count) : []
                DispatchQueue.main.async { isSyncingOverlay = false }
            }
            showingSheet = hasOverlay
        }
        .onChange(of: windowPages.count) { _, newCount in
            let targetCount = max(0, newCount - 1)
            syncedWindowDepth = targetCount
            if windowPath.count != targetCount {
                isSyncingWindow = true
                windowPath = targetCount > 0 ? Array(1..<newCount) : []
                DispatchQueue.main.async { isSyncingWindow = false }
            }
        }
        .onChange(of: overlayPages.count) { _, newCount in
            let targetCount = max(0, newCount - 1)
            syncedOverlayDepth = targetCount
            if overlayPath.count != targetCount {
                isSyncingOverlay = true
                overlayPath = targetCount > 0 ? Array(1..<newCount) : []
                DispatchQueue.main.async { isSyncingOverlay = false }
            }
        }
        .onChange(of: hasOverlay) { _, newHasOverlay in
            if showingSheet != newHasOverlay {
                showingSheet = newHasOverlay
            }
        }
        .onChange(of: showingSheet) { _, isShowing in
            if !isShowing && hasOverlay {
                dispatch("dismiss", nil)
            }
        }
        .onChange(of: windowPath) { _, newPath in
            if isSyncingWindow { return }
            if newPath.count < syncedWindowDepth {
                syncedWindowDepth = newPath.count
                let expectedCount = newPath.count
                DispatchQueue.main.asyncAfter(deadline: .now() + 0.35) {
                    if windowPath.count == expectedCount {
                        dispatch("pop", nil)
                    }
                }
            }
        }
        .onChange(of: overlayPath) { _, newPath in
            if isSyncingOverlay { return }
            if newPath.count < syncedOverlayDepth {
                syncedOverlayDepth = newPath.count
                let expectedCount = newPath.count
                DispatchQueue.main.asyncAfter(deadline: .now() + 0.35) {
                    if overlayPath.count == expectedCount {
                        dispatch("pop", nil)
                    }
                }
            }
        }
    }
}

struct DynamicTabBarView: View {
    let node: DynamicUINode
    let dispatch: (String, [String: Any]?) -> Void

    var activeIndex: Int {
        node.props["active_index"] as? Int ?? 0
    }

    var tabsList: [[String: String]] {
        (node.props["tabs"] as? [[String: Any]])?.compactMap { t in
            [
                "title": t["title"] as? String ?? "",
                "icon": (t["ios_icon"] as? String) ?? (t["icon"] as? String) ?? "",
                "action": t["action"] as? String ?? ""
            ]
        } ?? []
    }

    @State private var selectedTab: Int = 0

    var body: some View {
        TabView(selection: Binding(
            get: { selectedTab },
            set: { newIdx in
                selectedTab = newIdx
                if newIdx < tabsList.count {
                    let action = tabsList[newIdx]["action"] ?? ""
                    if !action.isEmpty {
                        dispatch(action, [:])
                    }
                }
            }
        )) {
            ForEach(0..<tabsList.count, id: \.self) { idx in
                Group {
                    if idx == activeIndex, let firstChild = node.children.first {
                        DynamicNodeView(node: firstChild, dispatch: dispatch)
                    } else {
                        Color.clear
                    }
                }
                .tabItem {
                    Label(tabsList[idx]["title"] ?? "", systemImage: sfSymbol(for: tabsList[idx]["icon"] ?? ""))
                }
                .tag(idx)
            }
        }
        .tint(AppTheme.primary)
        .preferredColorScheme({
            let mode = node.props["theme_mode"] as? String ?? "system"
            if mode == "light" { return .light }
            if mode == "dark" { return .dark }
            return nil
        }())
        .onAppear {
            selectedTab = activeIndex
        }
        .onChange(of: activeIndex) { _, newIdx in
            if selectedTab != newIdx {
                selectedTab = newIdx
            }
        }
    }

    private func sfSymbol(for iconName: String) -> String {
        switch iconName.lowercased() {
        case "house", "home": return "house.fill"
        case "bag", "orders", "cart": return "bag.fill"
        case "person", "profile", "user": return "person.fill"
        case "gear", "settings": return "gearshape.fill"
        default: return iconName.isEmpty ? "circle.fill" : iconName
        }
    }
}






struct DynamicTextInputWrapper: View {
    let value: String
    let placeholder: String
    let onChangeAction: String
    let dispatch: (String, [String: Any]?) -> Void
    @State private var localText: String = ""
    @State private var lastSent: String = ""
    @FocusState private var isFocused: Bool

    var body: some View {
        TextField(placeholder, text: Binding(
            get: { localText },
            set: { newVal in
                localText = newVal
                if !onChangeAction.isEmpty && newVal != lastSent {
                    lastSent = newVal
                    dispatch(onChangeAction, ["value": newVal])
                }
            }
        ))
        .focused($isFocused)
        .submitLabel(.done)
        .onSubmit { isFocused = false }
        .textFieldStyle(.roundedBorder)
        .onAppear {
            localText = value
            lastSent = value
        }
        .onChange(of: value) { _, newExternal in
            if !isFocused && newExternal != localText {
                localText = newExternal
                lastSent = newExternal
            }
        }
    }
}

struct DynamicNodeView: View {
    let node: DynamicUINode
    let dispatch: (String, [String: Any]?) -> Void

    var body: some View {
        switch node.name {
case "text", "icon", "button", "hstack", "vstack", "switch", "navigationstack", "page":
            renderGeneratedPrimitive(node: node, dispatch: dispatch)
case "list":
            List {
                ForEach(0..<node.children.count, id: \.self) { i in
                    DynamicNodeView(node: node.children[i], dispatch: dispatch)
                }
            }

        case "tabbar":
            DynamicTabBarView(node: node, dispatch: dispatch)
        case "if":
            let cond = node.props["condition"] as? Bool ?? false
            if cond {
                VStack(alignment: .leading, spacing: 0) {
                    ForEach(0..<node.children.count, id: \.self) { i in
                        DynamicNodeView(node: node.children[i], dispatch: dispatch)
                    }
                }
            }
        default:
            renderGeneratedPrimitive(node: node, dispatch: dispatch)
        }
    }
}

@_silgen_name("init_app")
func init_app() -> UnsafeMutableRawPointer?
@_silgen_name("get_ui")
func get_ui(_ ptr: UnsafeMutableRawPointer?) -> UnsafePointer<CChar>?
@_silgen_name("dispatch_event")
func dispatch_event(_ ptr: UnsafeMutableRawPointer?, _ action: UnsafePointer<CChar>?, _ payload: UnsafePointer<CChar>?) -> UnsafePointer<CChar>?
@_silgen_name("register_refresh_callback")
func register_refresh_callback(_ callback: @convention(c) () -> Void)

class RustEngine: ObservableObject {
    static let shared = RustEngine()
    var appPtr: UnsafeMutableRawPointer?
    @Published var liveNode: DynamicUINode? = nil
    init() {
        let ptr = init_app()
        self.appPtr = ptr
        if let ptr = ptr, let cStr = get_ui(ptr) {
            let b64 = String(cString: cStr)
            self.liveNode = DynamicUINode(b64: b64, existing: nil)
        }
        register_refresh_callback({ DispatchQueue.main.async { RustEngine.shared.refreshUI() } })
    }
    func refreshUI() {
        guard let ptr = self.appPtr, let cStr = get_ui(ptr) else { return }
        let b64 = String(cString: cStr)
        self.liveNode = DynamicUINode(b64: b64, existing: self.liveNode)
    }
    func dispatch(action: String, payload: [String: Any]? = nil) {
        guard let ptr = self.appPtr else { return }
        var payloadStr: String? = nil
        if let payload = payload, let data = try? JSONSerialization.data(withJSONObject: payload) {
            payloadStr = String(data: data, encoding: .utf8)
        }
        action.withCString { cAct in
            if let p = payloadStr {
                p.withCString { cPay in self.applyResult(dispatch_event(ptr, cAct, cPay)) }
            } else {
                self.applyResult(dispatch_event(ptr, cAct, nil))
            }
        }
    }
    private func applyResult(_ res: UnsafePointer<CChar>?) {
        guard let cStr = res else { return }
        let b64 = String(cString: cStr)
        self.liveNode = DynamicUINode(b64: b64, existing: self.liveNode)
    }
}

struct ProdEngineView: View {
    @ObservedObject private var engine = RustEngine.shared
    var body: some View {
        if let liveNode = engine.liveNode {
            DynamicNodeView(node: liveNode, dispatch: engine.dispatch)
        } else {
            Color.clear
        }
    }
}
struct DevEngineView: View {
    var onServerOffline: () -> Void = {}
    @State private var liveNode: DynamicUINode? = nil
    @State private var lastReloadTs: UInt64 = 0

    var body: some View {
        Group {
            if let liveNode = self.liveNode {
                DynamicNodeView(node: liveNode, dispatch: dispatchEvent)
            } else {
                ProgressView()
            }
        }
        .task {
            await fetchUIAsync()
            while !Task.isCancelled {
                guard let url = URL(string: "http://127.0.0.1:5824/api/live_reload?wait_for=\(self.lastReloadTs)") else { continue }
                do {
                    var request = URLRequest(url: url)
                    request.cachePolicy = .reloadIgnoringLocalCacheData
                    request.timeoutInterval = 20.0
                    let (data, response) = try await URLSession.shared.data(for: request)
                    guard let httpRes = response as? HTTPURLResponse, httpRes.statusCode == 200,
                          let json = try? JSONSerialization.jsonObject(with: data) as? [String: Any],
                          let ts = json["timestamp"] as? UInt64 else {
                        try? await Task.sleep(nanoseconds: 150_000_000)
                        continue
                    }
                    if ts != self.lastReloadTs {
                        self.lastReloadTs = ts
                        await fetchUIAsync()
                    }
                } catch {
                    try? await Task.sleep(nanoseconds: 250_000_000)
                }
            }
        }
    }

    @MainActor
    private func fetchUIAsync() async {
        guard let uiUrl = URL(string: "http://127.0.0.1:5824/api/ui_layout") else { return }
        var uiReq = URLRequest(url: uiUrl)
        uiReq.cachePolicy = .reloadIgnoringLocalCacheData
        uiReq.timeoutInterval = 1.0
        guard let (uiData, res) = try? await URLSession.shared.data(for: uiReq) else { onServerOffline(); return }
        if
           let b64 = String(data: uiData, encoding: .utf8) {
            if let httpRes = res as? HTTPURLResponse,
               let tsStr = httpRes.value(forHTTPHeaderField: "X-Reload-Timestamp"),
               let ts = UInt64(tsStr) {
                self.lastReloadTs = ts
            }
            self.liveNode = DynamicUINode(b64: b64, existing: self.liveNode)
        }
    }

    private func dispatchEvent(action: String, payload: [String: Any]?) {
        guard !action.isEmpty else { return }
        Task { @MainActor in
            if let url = URL(string: "http://127.0.0.1:5824/api/dispatch") {
                var request = URLRequest(url: url)
                request.cachePolicy = .reloadIgnoringLocalCacheData
                request.httpMethod = "POST"
                let body: [String: Any] = ["action": action, "payload": payload ?? [:]]
                request.httpBody = try? JSONSerialization.data(withJSONObject: body)
                request.timeoutInterval = 1.0
                if let (uiData, res) = try? await URLSession.shared.data(for: request),
                   let b64 = String(data: uiData, encoding: .utf8) {
                    if let httpRes = res as? HTTPURLResponse,
                       let tsStr = httpRes.value(forHTTPHeaderField: "X-Reload-Timestamp"),
                       let ts = UInt64(tsStr) {
                        self.lastReloadTs = ts
                    }
                    self.liveNode = DynamicUINode(b64: b64, existing: self.liveNode)
                }
            }
        }
    }
}
struct EngineView: View {
    @State private var useProdFfi: Bool = isProdBuild
    var body: some View {
        if useProdFfi {
            ProdEngineView()
        } else {
            DevEngineView(onServerOffline: { useProdFfi = true })
        }
    }
}

struct ContentView: View {
    var body: some View {
        EngineView()
    }
}
