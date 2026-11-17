// ==========================================
// ⚠️ GENERATED CODE - DO NOT EDIT ⚠️
// Generated from config/colors.json
// ==========================================
use serde_json::{json, Value};

#[derive(Debug, Clone, PartialEq)]
pub struct GradientStop {
    pub color: String,
    pub position: Option<f32>,
    pub opacity: f32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Gradient {
    pub gradient_type: String,
    pub direction: String,
    pub angle: Option<f32>,
    pub stops: Vec<GradientStop>,
    pub opacity: f32,
}

fn normalize_unit(val: f32) -> f32 {
    if val > 1.0 {
        (val / 100.0).clamp(0.0, 1.0)
    } else {
        val.clamp(0.0, 1.0)
    }
}

impl Gradient {
    pub fn new() -> Self {
        Self {
            gradient_type: "linear".to_string(),
            direction: "to_right".to_string(),
            angle: None,
            stops: Vec::new(),
            opacity: 1.0,
        }
    }

    pub fn linear() -> Self {
        Self::new()
    }

    pub fn radial() -> Self {
        Self {
            gradient_type: "radial".to_string(),
            ..Self::new()
        }
    }

    pub fn angular() -> Self {
        Self {
            gradient_type: "angular".to_string(),
            ..Self::new()
        }
    }

    pub fn conic() -> Self {
        Self::angular()
    }

    /// Valid options: linear, radial, angular
    pub fn kind(mut self, kind: impl Into<String>) -> Self {
        self.gradient_type = kind.into();
        self
    }

    /// Valid options: to_right, to_left, to_bottom, to_top, to_bottom_right, to_bottom_left, to_top_right, to_top_left
    pub fn direction(mut self, direction: impl Into<String>) -> Self {
        self.direction = direction.into();
        self
    }

    pub fn angle(mut self, degrees: f32) -> Self {
        self.angle = Some(degrees);
        self
    }

    /// Valid options: black, blue, brown, cyan, danger, fill, gray, green, grouped_background, indigo, mint, orange, pink, primary, purple, red, secondary, separator, success, surface, surface_secondary, teal, text_primary, text_secondary, text_tertiary, warning, white, yellow
    pub fn stop(mut self, color: impl Into<String>, position: f32) -> Self {
        self.stops.push(GradientStop {
            color: color.into(),
            position: Some(normalize_unit(position)),
            opacity: 1.0,
        });
        self
    }

    /// Valid options: black, blue, brown, cyan, danger, fill, gray, green, grouped_background, indigo, mint, orange, pink, primary, purple, red, secondary, separator, success, surface, surface_secondary, teal, text_primary, text_secondary, text_tertiary, warning, white, yellow
    pub fn stop_with_opacity(mut self, color: impl Into<String>, position: f32, opacity: f32) -> Self {
        self.stops.push(GradientStop {
            color: color.into(),
            position: Some(normalize_unit(position)),
            opacity: normalize_unit(opacity),
        });
        self
    }

    /// Valid options: black, blue, brown, cyan, danger, fill, gray, green, grouped_background, indigo, mint, orange, pink, primary, purple, red, secondary, separator, success, surface, surface_secondary, teal, text_primary, text_secondary, text_tertiary, warning, white, yellow
    pub fn add_stop(mut self, color: impl Into<String>) -> Self {
        self.stops.push(GradientStop {
            color: color.into(),
            position: None,
            opacity: 1.0,
        });
        self
    }

    pub fn opacity(mut self, opacity: f32) -> Self {
        self.opacity = normalize_unit(opacity);
        self
    }

    pub fn to_json_value(&self) -> Value {
        let count = self.stops.len();
        let mut stops_json = Vec::with_capacity(count);
        for (i, s) in self.stops.iter().enumerate() {
            let pos = s.position.unwrap_or_else(|| {
                if count <= 1 { 0.0 } else { (i as f32) / ((count - 1) as f32) }
            });
            stops_json.push(json!({
                "color": s.color,
                "position": pos,
                "opacity": s.opacity,
            }));
        }
        let angle_deg = self.angle.unwrap_or_else(|| match self.direction.as_str() {
            "to_right" => 90.0,
            "to_left" => 270.0,
            "to_bottom" => 180.0,
            "to_top" => 0.0,
            "to_bottom_right" => 135.0,
            "to_bottom_left" => 225.0,
            "to_top_right" => 45.0,
            "to_top_left" => 315.0,
            _ => 90.0,
        });
        json!({
            "type": self.gradient_type,
            "direction": self.direction,
            "angle": angle_deg,
            "opacity": self.opacity,
            "stops": stops_json,
        })
    }
}


pub trait IntoBackgroundImage {
    fn into_bg_image_url(self) -> String;
}
impl IntoBackgroundImage for &str {
    fn into_bg_image_url(self) -> String { self.to_string() }
}
impl IntoBackgroundImage for String {
    fn into_bg_image_url(self) -> String { self }
}
impl IntoBackgroundImage for crate::engine::primitives::Image {
    fn into_bg_image_url(self) -> String {
        self.props.into_iter().find_map(|(k, v)| {
            if k == "src" {
                if let crate::engine::ast::PropValue::String(s) = v { Some(s) } else { None }
            } else { None }
        }).unwrap_or_default()
    }
}

pub enum BackgroundValue {
    Solid(String),
    Gradient(Gradient),
}

pub trait IntoBackground {
    fn into_background(self) -> BackgroundValue;
}

impl IntoBackground for &str {
    fn into_background(self) -> BackgroundValue {
        BackgroundValue::Solid(self.to_string())
    }
}

impl IntoBackground for String {
    fn into_background(self) -> BackgroundValue {
        BackgroundValue::Solid(self)
    }
}

impl IntoBackground for Gradient {
    fn into_background(self) -> BackgroundValue {
        BackgroundValue::Gradient(self)
    }
}

#[macro_export]
macro_rules! Gradient {
    () => {
        $crate::engine::primitives::Gradient::new()
    };
    ([$($color:expr),+ $(,)?]) => {
        let mut g = $crate::engine::primitives::Gradient::new();
        $(
            g = g.add_stop($color);
        )+
        g
    };
    ($c1:expr, $c2:expr $(, $rest:expr)* $(,)?) => {
        let mut g = $crate::engine::primitives::Gradient::new()
            .add_stop($c1)
            .add_stop($c2);
        $(
            g = g.add_stop($rest);
        )*
        g
    };
}

pub const COLOR_IOS_HELPERS: &str = r##"
func parseHexUIColor(_ hex: String) -> UIColor {
    let clean = hex.trimmingCharacters(in: CharacterSet.alphanumerics.inverted)
    var int: UInt64 = 0
    Scanner(string: clean).scanHexInt64(&int)
    let a, r, g, b: UInt64
    switch clean.count {
    case 3:
        (a, r, g, b) = (255, (int >> 8) * 17, (int >> 4 & 0xF) * 17, (int & 0xF) * 17)
    case 6:
        (a, r, g, b) = (255, int >> 16, int >> 8 & 0xFF, int & 0xFF)
    case 8:
        (r, g, b, a) = (int >> 24, int >> 16 & 0xFF, int >> 8 & 0xFF, int & 0xFF)
    default:
        (a, r, g, b) = (255, 0, 0, 0)
    }
    return UIColor(red: CGFloat(r) / 255.0, green: CGFloat(g) / 255.0, blue: CGFloat(b) / 255.0, alpha: CGFloat(a) / 255.0)
}

func resolveAppColor(_ name: String) -> Color? {
    if name.isEmpty { return nil }
    if runtimeAccentMode == "app" {
        if name == "primary" && !runtimePrimaryHex.isEmpty {
            return Color(uiColor: parseHexUIColor(runtimePrimaryHex))
        }
        if name == "secondary" && !runtimeSecondaryHex.isEmpty {
            return Color(uiColor: parseHexUIColor(runtimeSecondaryHex))
        }
    }
    switch name {
    case "black": return Color.black
    case "blue": return Color.blue
    case "brown": return Color.brown
    case "cyan": return Color.cyan
    case "danger": return Color.red
    case "fill": return Color(uiColor: .secondarySystemFill)
    case "gray": return Color.gray
    case "green": return Color.green
    case "grouped_background": return Color(uiColor: .systemGroupedBackground)
    case "indigo": return Color.indigo
    case "mint": return Color.mint
    case "orange": return Color.orange
    case "pink": return Color.pink
    case "primary": return AppTheme.primary
    case "purple": return Color.purple
    case "red": return Color.red
    case "secondary": return Color.secondary
    case "separator": return Color(uiColor: .separator)
    case "success": return Color.green
    case "surface": return Color(uiColor: .systemBackground)
    case "surface_secondary": return Color(uiColor: .secondarySystemBackground)
    case "teal": return Color.teal
    case "text_primary": return Color(uiColor: .label)
    case "text_secondary": return Color(uiColor: .secondaryLabel)
    case "text_tertiary": return Color(uiColor: .tertiaryLabel)
    case "warning": return Color.orange
    case "white": return Color.white
    case "yellow": return Color.yellow
    default:
        if name.hasPrefix("#") || name.hasPrefix("0x") {
            return Color(uiColor: parseHexUIColor(name))
        }
        return nil
    }
}

@ViewBuilder
func renderGradientView(_ grad: [String: Any]) -> some View {
    let gType = grad["type"] as? String ?? "linear"
    let direction = grad["direction"] as? String ?? "to_right"
    let gradOpacity = (grad["opacity"] as? NSNumber)?.doubleValue ?? 1.0
    let rawStops = grad["stops"] as? [[String: Any]] ?? []
    let swiftStops: [Gradient.Stop] = rawStops.compactMap { s in
        let colName = s["color"] as? String ?? ""
        let pos = (s["position"] as? NSNumber)?.doubleValue ?? 0.0
        let op = (s["opacity"] as? NSNumber)?.doubleValue ?? 1.0
        guard let baseCol = resolveAppColor(colName) else { return nil }
        return Gradient.Stop(color: baseCol.opacity(op), location: CGFloat(pos))
    }
    let (startPt, endPt): (UnitPoint, UnitPoint) = {
        switch direction {
        case "to_left": return (.trailing, .leading)
        case "to_bottom": return (.top, .bottom)
        case "to_top": return (.bottom, .top)
        case "to_bottom_right": return (.topLeading, .bottomTrailing)
        case "to_bottom_left": return (.topTrailing, .bottomLeading)
        case "to_top_right": return (.bottomLeading, .topTrailing)
        case "to_top_left": return (.bottomTrailing, .topLeading)
        default: return (.leading, .trailing)
        }
    }()
    if gType == "radial" {
        RadialGradient(gradient: Gradient(stops: swiftStops), center: .center, startRadius: 0, endRadius: 240)
            .opacity(gradOpacity)
    } else if gType == "angular" || gType == "conic" {
        AngularGradient(gradient: Gradient(stops: swiftStops), center: .center)
            .opacity(gradOpacity)
    } else {
        LinearGradient(gradient: Gradient(stops: swiftStops), startPoint: startPt, endPoint: endPt)
            .opacity(gradOpacity)
    }
}

struct UnclipListCell: UIViewRepresentable {
    func makeUIView(context: Context) -> UIView {
        let v = UIView()
        DispatchQueue.main.async {
            var cur: UIView? = v
            while let p = cur?.superview {
                p.clipsToBounds = false
                if String(describing: type(of: p)).contains("Cell") { break }
                cur = p
            }
        }
        return v
    }
    func updateUIView(_ uiView: UIView, context: Context) {}
}

struct NativeFlowLayout: Layout {
    var spacing: CGFloat = 0
    var alignX: String = "left"
    var alignY: String = "center"

    func sizeThatFits(proposal: ProposedViewSize, subviews: Subviews, cache: inout ()) -> CGSize {
        let maxWidth = proposal.width ?? .infinity
        var rowWidth: CGFloat = 0
        var rowHeight: CGFloat = 0
        var totalHeight: CGFloat = 0
        var maxRowWidth: CGFloat = 0
        for subview in subviews {
            let sz = subview.sizeThatFits(.unspecified)
            if rowWidth > 0 && rowWidth + spacing + sz.width > maxWidth {
                maxRowWidth = max(maxRowWidth, rowWidth)
                totalHeight += rowHeight + spacing
                rowWidth = sz.width
                rowHeight = sz.height
            } else {
                rowWidth = rowWidth > 0 ? rowWidth + spacing + sz.width : sz.width
                rowHeight = max(rowHeight, sz.height)
            }
        }
        maxRowWidth = max(maxRowWidth, rowWidth)
        totalHeight += rowHeight
        return CGSize(width: maxWidth.isFinite && alignX == "space_between" ? maxWidth : maxRowWidth, height: totalHeight)
    }

    func placeSubviews(in bounds: CGRect, proposal: ProposedViewSize, subviews: Subviews, cache: inout ()) {
        var rows: [[(LayoutSubviews.Element, CGSize)]] = []
        var currentRow: [(LayoutSubviews.Element, CGSize)] = []
        var rowWidth: CGFloat = 0
        for subview in subviews {
            let sz = subview.sizeThatFits(.unspecified)
            if !currentRow.isEmpty && rowWidth + spacing + sz.width > bounds.width {
                rows.append(currentRow)
                currentRow = [(subview, sz)]
                rowWidth = sz.width
            } else {
                rowWidth = currentRow.isEmpty ? sz.width : rowWidth + spacing + sz.width
                currentRow.append((subview, sz))
            }
        }
        if !currentRow.isEmpty { rows.append(currentRow) }

        var y = bounds.minY
        for row in rows {
            let rHeight = row.map { $0.1.height }.max() ?? 0
            let itemsWidth = row.reduce(0) { $0 + $1.1.width }
            let gapsWidth = CGFloat(max(0, row.count - 1)) * spacing
            let usedWidth = itemsWidth + gapsWidth
            let extraSpace = max(0, bounds.width - usedWidth)
            var x = bounds.minX
            var dynamicGap = spacing
            if alignX == "center" {
                x += extraSpace / 2
            } else if alignX == "right" {
                x += extraSpace
            } else if alignX == "space_between" && row.count > 1 {
                dynamicGap = extraSpace / CGFloat(row.count - 1) + spacing
            }
            for (subview, sz) in row {
                let itemY: CGFloat = alignY == "top" ? y : (alignY == "bottom" ? y + (rHeight - sz.height) : y + (rHeight - sz.height) / 2)
                subview.place(at: CGPoint(x: x, y: itemY), proposal: ProposedViewSize(sz))
                x += sz.width + dynamicGap
            }
            y += rHeight + spacing
        }
    }
}

extension View {
    @ViewBuilder
    func applyListRowBackground(node: DynamicUINode) -> some View {
        let bgName = node.props["background"] as? String ?? ""
        let bgGrad = node.props["background_gradient"] as? [String: Any]
        if let grad = bgGrad {
            self.listRowBackground(renderGradientView(grad))
        } else if !bgName.isEmpty, let col = resolveAppColor(bgName) {
            self.listRowBackground(col)
        } else {
            self
        }
    }

    @ViewBuilder
    func applyContainerStyle(node: DynamicUINode) -> some View {
        let pad = CGFloat(node.props["padding"] as? Int ?? 0)
        let radius = CGFloat(node.props["corner_radius"] as? Int ?? 0)
        let opacity = Double(node.props["opacity"] as? Int ?? 100) / 100.0
        let bgName = node.props["background"] as? String ?? ""
        let bgImg = node.props["background_image"] as? String ?? ""
        let bgGrad = node.props["background_gradient"] as? [String: Any]

        let styled = self
            .padding(pad)
            .background {
                if !bgImg.isEmpty {
                    if (bgImg.hasPrefix("http://") || bgImg.hasPrefix("https://")), let url = URL(string: bgImg) {
                        AsyncImage(url: url) { phase in
                            if case .success(let image) = phase { image.resizable().scaledToFill() }
                            else { Color.secondary.opacity(0.12) }
                        }
                    } else {
                        let clean = ((bgImg as NSString).lastPathComponent as NSString).deletingPathExtension
                        if UIImage(named: clean) != nil {
                            Image(clean).resizable().scaledToFill()
                        } else if let devUrl = URL(string: "http://127.0.0.1:5824/" + bgImg.replacingOccurrences(of: "./", with: "").trimmingCharacters(in: CharacterSet(charactersIn: "/"))) {
                            AsyncImage(url: devUrl) { phase in
                                if case .success(let image) = phase { image.resizable().scaledToFill() }
                                else { Color.secondary.opacity(0.12) }
                            }
                        }
                    }
                } else if let grad = bgGrad {
                    renderGradientView(grad)
                } else if !bgName.isEmpty, let col = resolveAppColor(bgName) {
                    col
                }
            };
        if radius > 0 || !bgImg.isEmpty {
            styled
                .clipShape(RoundedRectangle(cornerRadius: radius, style: .continuous))
                .opacity(opacity)
        } else {
            styled
                .opacity(opacity)
        }
    }
}
"##;

pub const COLOR_ANDROID_HELPERS: &str = r##"
@Composable
fun resolveAppColor(name: String): androidx.compose.ui.graphics.Color? {
    if (name.isEmpty()) return null
    return when (name) {
        "black" -> androidx.compose.ui.graphics.Color(0xFF000000)
        "blue" -> if (isSystemInDarkTheme()) androidx.compose.ui.graphics.Color(0xFF0A84FF) else androidx.compose.ui.graphics.Color(0xFF007AFF)
        "brown" -> if (isSystemInDarkTheme()) androidx.compose.ui.graphics.Color(0xFFAC8E68) else androidx.compose.ui.graphics.Color(0xFFA2845E)
        "cyan" -> if (isSystemInDarkTheme()) androidx.compose.ui.graphics.Color(0xFF64D2FF) else androidx.compose.ui.graphics.Color(0xFF32ADE6)
        "danger" -> MaterialTheme.colorScheme.error
        "fill" -> MaterialTheme.colorScheme.surfaceVariant
        "gray" -> MaterialTheme.colorScheme.outline
        "green" -> if (isSystemInDarkTheme()) androidx.compose.ui.graphics.Color(0xFF30D158) else androidx.compose.ui.graphics.Color(0xFF34C759)
        "grouped_background" -> MaterialTheme.colorScheme.background
        "indigo" -> if (isSystemInDarkTheme()) androidx.compose.ui.graphics.Color(0xFF5E5CE6) else androidx.compose.ui.graphics.Color(0xFF5856D6)
        "mint" -> if (isSystemInDarkTheme()) androidx.compose.ui.graphics.Color(0xFF63E6E2) else androidx.compose.ui.graphics.Color(0xFF00C7BE)
        "orange" -> if (isSystemInDarkTheme()) androidx.compose.ui.graphics.Color(0xFFFF9F0A) else androidx.compose.ui.graphics.Color(0xFFFF9500)
        "pink" -> if (isSystemInDarkTheme()) androidx.compose.ui.graphics.Color(0xFFFF375F) else androidx.compose.ui.graphics.Color(0xFFFF2D55)
        "primary" -> MaterialTheme.colorScheme.primary
        "purple" -> if (isSystemInDarkTheme()) androidx.compose.ui.graphics.Color(0xFFBF5AF2) else androidx.compose.ui.graphics.Color(0xFFAF52DE)
        "red" -> if (isSystemInDarkTheme()) androidx.compose.ui.graphics.Color(0xFFFF453A) else androidx.compose.ui.graphics.Color(0xFFFF3B30)
        "secondary" -> MaterialTheme.colorScheme.secondary
        "separator" -> MaterialTheme.colorScheme.outlineVariant
        "success" -> if (isSystemInDarkTheme()) androidx.compose.ui.graphics.Color(0xFF66BB6A) else androidx.compose.ui.graphics.Color(0xFF2E7D32)
        "surface" -> MaterialTheme.colorScheme.surface
        "surface_secondary" -> MaterialTheme.colorScheme.surfaceVariant
        "teal" -> if (isSystemInDarkTheme()) androidx.compose.ui.graphics.Color(0xFF40C8E0) else androidx.compose.ui.graphics.Color(0xFF30B0C7)
        "text_primary" -> MaterialTheme.colorScheme.onSurface
        "text_secondary" -> MaterialTheme.colorScheme.onSurfaceVariant
        "text_tertiary" -> MaterialTheme.colorScheme.outline
        "warning" -> if (isSystemInDarkTheme()) androidx.compose.ui.graphics.Color(0xFFFFA726) else androidx.compose.ui.graphics.Color(0xFFED6C02)
        "white" -> androidx.compose.ui.graphics.Color(0xFFFFFFFF)
        "yellow" -> if (isSystemInDarkTheme()) androidx.compose.ui.graphics.Color(0xFFFFD60A) else androidx.compose.ui.graphics.Color(0xFFFFCC00)
        else -> {
            if (name.startsWith("#")) {
                try {
                    val clean = name.removePrefix("#")
                    val normalized = when (clean.length) {
                        3 -> "#${clean[0]}${clean[0]}${clean[1]}${clean[1]}${clean[2]}${clean[2]}"
                        8 -> "#${clean.substring(6, 8)}${clean.substring(0, 6)}"
                        else -> name
                    }
                    androidx.compose.ui.graphics.Color(android.graphics.Color.parseColor(normalized))
                } catch (e: Exception) { null }
            } else null
        }
    }
}

@Composable
fun resolveAppGradientBrush(grad: org.json.JSONObject): androidx.compose.ui.graphics.Brush? {
    val gType = grad.optString("type", "linear")
    val direction = grad.optString("direction", "to_right")
    val gradOpacity = grad.optDouble("opacity", 1.0).toFloat()
    val stopsArr = grad.optJSONArray("stops") ?: return null
    val colorStops = mutableListOf<Pair<Float, androidx.compose.ui.graphics.Color>>()
    for (i in 0 until stopsArr.length()) {
        val s = stopsArr.optJSONObject(i) ?: continue
        val colName = s.optString("color", "")
        val pos = s.optDouble("position", 0.0).toFloat()
        val op = (s.optDouble("opacity", 1.0).toFloat() * gradOpacity).coerceIn(0f, 1f)
        val baseCol = resolveAppColor(colName) ?: continue
        colorStops.add(Pair(pos, baseCol.copy(alpha = baseCol.alpha * op)))
    }
    if (colorStops.size < 2) return null
    val stopsArray = colorStops.toTypedArray()
    return when (gType) {
        "radial" -> androidx.compose.ui.graphics.Brush.radialGradient(colorStops = stopsArray)
        "angular", "conic" -> androidx.compose.ui.graphics.Brush.sweepGradient(colorStops = stopsArray)
        else -> {
            val (start, end) = when (direction) {
                "to_left" -> Pair(androidx.compose.ui.geometry.Offset(1000f, 0f), androidx.compose.ui.geometry.Offset(0f, 0f))
                "to_bottom" -> Pair(androidx.compose.ui.geometry.Offset(0f, 0f), androidx.compose.ui.geometry.Offset(0f, 1000f))
                "to_top" -> Pair(androidx.compose.ui.geometry.Offset(0f, 1000f), androidx.compose.ui.geometry.Offset(0f, 0f))
                "to_bottom_right" -> Pair(androidx.compose.ui.geometry.Offset(0f, 0f), androidx.compose.ui.geometry.Offset(1000f, 1000f))
                "to_bottom_left" -> Pair(androidx.compose.ui.geometry.Offset(1000f, 0f), androidx.compose.ui.geometry.Offset(0f, 1000f))
                "to_top_right" -> Pair(androidx.compose.ui.geometry.Offset(0f, 1000f), androidx.compose.ui.geometry.Offset(1000f, 0f))
                "to_top_left" -> Pair(androidx.compose.ui.geometry.Offset(1000f, 1000f), androidx.compose.ui.geometry.Offset(0f, 0f))
                else -> Pair(androidx.compose.ui.geometry.Offset(0f, 0f), androidx.compose.ui.geometry.Offset(1000f, 0f))
            }
            androidx.compose.ui.graphics.Brush.linearGradient(colorStops = stopsArray, start = start, end = end)
        }
    }
}

@Composable
fun Modifier.applyContainerStyle(node: DynamicUINode): Modifier {
    var m = this
    val radius = node.getInt("corner_radius", 0)
    val pad = node.getInt("padding", 0)
    val opacity = node.getInt("opacity", 100)
    val bgName = node.getString("background", "")
    val bgGrad = node.props["background_gradient"] as? org.json.JSONObject

    if (radius > 0) {
        m = m.clip(androidx.compose.foundation.shape.RoundedCornerShape(radius.dp))
    }
    if (bgGrad != null) {
        val brush = resolveAppGradientBrush(bgGrad)
        if (brush != null) {
            m = m.background(brush = brush)
        }
    } else if (bgName.isNotEmpty()) {
        val col = resolveAppColor(bgName)
        if (col != null) {
            m = m.background(color = col)
        }
    }
    if (pad > 0) {
        m = m.padding(pad.dp)
    }
    if (opacity < 100) {
        m = m.graphicsLayer(alpha = opacity.toFloat() / 100f)
    }
    return m
}
"##;

pub const COLOR_GTK_HELPERS: &str = r##"
pub fn resolve_gtk_color_css(name: &str) -> String {
    match name {
        "black" => "@dark_5".to_string(),
        "blue" => "@blue_3".to_string(),
        "brown" => "@brown_3".to_string(),
        "cyan" => "@blue_2".to_string(),
        "danger" => "@error_color".to_string(),
        "fill" => "@headerbar_bg_color".to_string(),
        "gray" => "@dark_1".to_string(),
        "green" => "@green_3".to_string(),
        "grouped_background" => "@window_bg_color".to_string(),
        "indigo" => "@purple_4".to_string(),
        "mint" => "@green_2".to_string(),
        "orange" => "@orange_3".to_string(),
        "pink" => "@red_2".to_string(),
        "primary" => "@accent_color".to_string(),
        "purple" => "@purple_3".to_string(),
        "red" => "@red_3".to_string(),
        "secondary" => "@purple_3".to_string(),
        "separator" => "@shade_color".to_string(),
        "success" => "@success_color".to_string(),
        "surface" => "@card_bg_color".to_string(),
        "surface_secondary" => "@view_bg_color".to_string(),
        "teal" => "@blue_2".to_string(),
        "text_primary" => "@window_fg_color".to_string(),
        "text_secondary" => "#5E5C64".to_string(),
        "text_tertiary" => "#77767B".to_string(),
        "warning" => "@warning_color".to_string(),
        "white" => "@light_1".to_string(),
        "yellow" => "@yellow_3".to_string(),
        other => {
            if other.starts_with('#') || other.starts_with('@') || other.starts_with("rgb") || other.starts_with("hsl") {
                other.to_string()
            } else {
                format!("@{}", other)
            }
        }
    }
}

pub fn apply_gtk_fg_color(widget: &impl gtk4::prelude::IsA<gtk4::Widget>, color: &str) {
    if color.is_empty() {
        return;
    }
    if color.starts_with('#') || color.starts_with("rgb") || color.starts_with("hsl") {
        widget.add_css_class("nk-dyn-fg");
        let provider = gtk4::CssProvider::new();
        provider.load_from_data(&format!(".nk-dyn-fg {{ color: {}; }}", color));
        #[allow(deprecated)]
        widget.style_context().add_provider(&provider, gtk4::STYLE_PROVIDER_PRIORITY_USER);
    } else {
        widget.add_css_class(&format!("nk-fg-{}", color));
    }
}

pub fn apply_gtk_container_style(bx: &impl gtk4::prelude::IsA<gtk4::Widget>, node: &DynamicUINode) {
    let pad = node.get_int("padding", 0);
    let radius = node.get_int("corner_radius", 0);
    let opacity = node.get_int("opacity", 100);
    let bg = node.get_string("background", "");
    let bg_grad = node.props.get("background_gradient");

    if opacity < 100 {
        bx.set_opacity((opacity as f64) / 100.0);
    }

    let mut rules = String::new();
    if !bg.is_empty() && bg_grad.is_none() {
        if bg.starts_with('#') || bg.starts_with("rgb") || bg.starts_with("hsl") {
            rules.push_str(&format!("background-color: {}; ", bg));
        } else {
            bx.add_css_class(&format!("nk-bg-{}", bg));
        }
    }
    if pad > 0 {
        rules.push_str(&format!("padding: {}px; ", pad));
    }
    if radius > 0 {
        rules.push_str(&format!("border-radius: {}px; ", radius));
    }
    if let Some(grad) = bg_grad {
        let g_type = grad.get("type").and_then(|v| v.as_str()).unwrap_or("linear");
        let angle = grad.get("angle").and_then(|v| v.as_f64()).unwrap_or(90.0);
        if let Some(stops) = grad.get("stops").and_then(|v| v.as_array()) {
            let mut stop_strs = Vec::new();
            for s in stops {
                let col_name = s.get("color").and_then(|v| v.as_str()).unwrap_or("primary");
                let pos = s.get("position").and_then(|v| v.as_f64()).unwrap_or(0.0) * 100.0;
                let op = s.get("opacity").and_then(|v| v.as_f64()).unwrap_or(1.0);
                let base_css = resolve_gtk_color_css(col_name);
                let col_css = if op < 0.999 {
                    format!("alpha({}, {:.2})", base_css, op)
                } else {
                    base_css
                };
                stop_strs.push(format!("{} {:.0}%", col_css, pos));
            }
            if stop_strs.len() >= 2 {
                let joined = stop_strs.join(", ");
                if g_type == "radial" {
                    rules.push_str(&format!("background-image: radial-gradient(circle, {}); ", joined));
                } else if g_type == "angular" || g_type == "conic" {
                    rules.push_str(&format!("background-image: conic-gradient({}); ", joined));
                } else {
                    rules.push_str(&format!("background-image: linear-gradient({:.0}deg, {}); ", angle, joined));
                }
            }
        }
    }

    if !rules.is_empty() {
        bx.add_css_class("nk-dyn-box");
        let provider = gtk4::CssProvider::new();
        provider.load_from_data(&format!(".nk-dyn-box {{ {} }}", rules));
        #[allow(deprecated)]
        bx.style_context().add_provider(&provider, gtk4::STYLE_PROVIDER_PRIORITY_USER);
    }
}
"##;

pub const COLOR_WEB_HELPERS: &str = r##"
function isInlineWebColor(name) {
    return Boolean(name && (name.startsWith("#") || name.startsWith("rgb") || name.startsWith("hsl") || name.startsWith("var(")));
}

function resolveWebColor(name) {
    if (!name) return "";
    if (isInlineWebColor(name)) return name;
    return `var(--nk-color-${name})`;
}

function applyWebFgColor(el, color) {
    if (!color) return;
    if (isInlineWebColor(color)) {
        el.style.color = color;
    } else {
        el.classList.add(`nk-fg-${color}`);
    }
}

function applyWebBgColor(el, color) {
    if (!color) return;
    if (isInlineWebColor(color)) {
        el.style.backgroundColor = color;
    } else {
        el.classList.add(`nk-bg-${color}`);
    }
}

function applyWebContainerStyle(el, node) {
    if (!node || !node.props) return;
    if (node.props.spacing !== undefined) el.style.gap = node.props.spacing + "px";
    if (node.props.padding !== undefined) el.style.padding = node.props.padding + "px";
    if (node.props.corner_radius !== undefined) el.style.borderRadius = node.props.corner_radius + "px";
    if (node.props.opacity !== undefined) el.style.opacity = (node.props.opacity / 100.0).toString();
    if (node.props.background_image) {
        el.style.backgroundImage = `url("${node.props.background_image}")`;
        el.style.backgroundSize = "cover";
        el.style.backgroundPosition = "center";
        el.style.backgroundRepeat = "no-repeat";
    } else if (node.props.background_gradient) {
        const grad = node.props.background_gradient;
        const gType = grad.type || "linear";
        const angle = (grad.angle !== undefined) ? grad.angle : 90;
        const stops = (grad.stops || []).map(s => {
            const base = resolveWebColor(s.color);
            const pos = Math.round(((s.position !== undefined) ? s.position : 0) * 100);
            const op = (s.opacity !== undefined) ? s.opacity : 1.0;
            const colExpr = (op < 0.999) ? `color-mix(in srgb, ${base} ${Math.round(op * 100)}%, transparent)` : base;
            return `${colExpr} ${pos}%`;
        });
        if (stops.length >= 2) {
            if (gType === "radial") {
                el.style.backgroundImage = `radial-gradient(circle, ${stops.join(", ")})`;
            } else if (gType === "angular" || gType === "conic") {
                el.style.backgroundImage = `conic-gradient(${stops.join(", ")})`;
            } else {
                el.style.backgroundImage = `linear-gradient(${angle}deg, ${stops.join(", ")})`;
            }
        }
    } else if (node.props.background) {
        applyWebBgColor(el, node.props.background);
    }
}
"##;

pub const COLOR_WINDOWS_HELPERS: &str = r##"
        private string MapColorToken(string name) {
            switch (name) {
                case "black": return "#000000";
                case "blue": return "#0078D4";
                case "brown": return "#8E562E";
                case "cyan": return "#0099BC";
                case "danger": return "SystemFillColorCriticalBrush";
                case "fill": return "SubtleFillColorSecondaryBrush";
                case "gray": return "#8A8886";
                case "green": return "#107C10";
                case "grouped_background": return "SolidBackgroundFillColorBaseBrush";
                case "indigo": return "#4F46E5";
                case "mint": return "#00CC6A";
                case "orange": return "#FF8C00";
                case "pink": return "#E3008C";
                case "primary": return "SystemAccentColor";
                case "purple": return "#881798";
                case "red": return "#D13438";
                case "secondary": return "TextFillColorSecondaryBrush";
                case "separator": return "DividerStrokeColorDefaultBrush";
                case "success": return "SystemFillColorSuccessBrush";
                case "surface": return "CardBackgroundFillColorDefaultBrush";
                case "surface_secondary": return "LayerFillColorDefaultBrush";
                case "teal": return "#00B7C3";
                case "text_primary": return "TextFillColorPrimaryBrush";
                case "text_secondary": return "TextFillColorSecondaryBrush";
                case "text_tertiary": return "TextFillColorTertiaryBrush";
                case "warning": return "SystemFillColorCautionBrush";
                case "white": return "#FFFFFF";
                case "yellow": return "#FFB900";
                default: return name;
            }
        }

        private Windows.UI.Color ParseHexWinColor(string hex, double opacity = 1.0) {
            string clean = hex.TrimStart('#');
            if (clean.Length == 3) {
                clean = $"{clean[0]}{clean[0]}{clean[1]}{clean[1]}{clean[2]}{clean[2]}";
            }
            if (clean.Length == 6) {
                byte r = Convert.ToByte(clean.Substring(0, 2), 16);
                byte g = Convert.ToByte(clean.Substring(2, 2), 16);
                byte b = Convert.ToByte(clean.Substring(4, 2), 16);
                byte a = (byte)Math.Clamp((int)Math.Round(opacity * 255.0), 0, 255);
                return Windows.UI.Color.FromArgb(a, r, g, b);
            } else if (clean.Length == 8) {
                byte r = Convert.ToByte(clean.Substring(0, 2), 16);
                byte g = Convert.ToByte(clean.Substring(2, 2), 16);
                byte b = Convert.ToByte(clean.Substring(4, 2), 16);
                byte hexA = Convert.ToByte(clean.Substring(6, 2), 16);
                byte a = (byte)Math.Clamp((int)Math.Round((hexA / 255.0) * opacity * 255.0), 0, 255);
                return Windows.UI.Color.FromArgb(a, r, g, b);
            }
            return Windows.UI.Color.FromArgb(255, 0, 120, 212);
        }

        private Microsoft.UI.Xaml.Media.Brush ResolveAppBrush(string name, double opacity = 1.0) {
            if (string.IsNullOrEmpty(name)) return null;
            if (name == "primary" && !string.IsNullOrEmpty(runtimePrimaryHex)) {
                return new Microsoft.UI.Xaml.Media.SolidColorBrush(ParseHexWinColor(runtimePrimaryHex, opacity));
            }
            string token = MapColorToken(name);
            if (token.StartsWith("#")) {
                return new Microsoft.UI.Xaml.Media.SolidColorBrush(ParseHexWinColor(token, opacity));
            }
            try {
                if (Application.Current.Resources.ContainsKey(token)) {
                    var res = Application.Current.Resources[token];
                    if (res is Microsoft.UI.Xaml.Media.Brush b) return b;
                    if (res is Windows.UI.Color c) {
                        return new Microsoft.UI.Xaml.Media.SolidColorBrush(c) { Opacity = opacity };
                    }
                }
            } catch { }
            return null;
        }

        private void ApplyWindowsContainerStyle(StackPanel sp, DynamicUINode node) {
            if (node.Props == null) return;
            if (node.Props.ContainsKey("padding") && double.TryParse(node.Props["padding"].ToString(), out double pad)) {
                sp.Padding = new Thickness(pad);
            }
            if (node.Props.ContainsKey("corner_radius") && double.TryParse(node.Props["corner_radius"].ToString(), out double rad)) {
                sp.CornerRadius = new CornerRadius(rad);
            }
            if (node.Props.ContainsKey("opacity") && double.TryParse(node.Props["opacity"].ToString(), out double op)) {
                sp.Opacity = op / 100.0;
            }
            if (node.Props.ContainsKey("background_gradient") && node.Props["background_gradient"] is Dictionary<string, object> gradDict) {
                var lgb = new Microsoft.UI.Xaml.Media.LinearGradientBrush();
                string dir = gradDict.ContainsKey("direction") ? gradDict["direction"].ToString() : "to_right";
                switch (dir) {
                    case "to_left": lgb.StartPoint = new Windows.Foundation.Point(1, 0.5); lgb.EndPoint = new Windows.Foundation.Point(0, 0.5); break;
                    case "to_bottom": lgb.StartPoint = new Windows.Foundation.Point(0.5, 0); lgb.EndPoint = new Windows.Foundation.Point(0.5, 1); break;
                    case "to_top": lgb.StartPoint = new Windows.Foundation.Point(0.5, 1); lgb.EndPoint = new Windows.Foundation.Point(0.5, 0); break;
                    case "to_bottom_right": lgb.StartPoint = new Windows.Foundation.Point(0, 0); lgb.EndPoint = new Windows.Foundation.Point(1, 1); break;
                    default: lgb.StartPoint = new Windows.Foundation.Point(0, 0.5); lgb.EndPoint = new Windows.Foundation.Point(1, 0.5); break;
                }
                if (gradDict.ContainsKey("stops") && gradDict["stops"] is System.Text.Json.JsonElement stopsEl && stopsEl.ValueKind == System.Text.Json.JsonValueKind.Array) {
                    foreach (var s in stopsEl.EnumerateArray()) {
                        string colName = s.TryGetProperty("color", out var cp) ? cp.GetString() : "blue";
                        double pos = s.TryGetProperty("position", out var pp) ? pp.GetDouble() : 0.0;
                        double sOp = s.TryGetProperty("opacity", out var opP) ? opP.GetDouble() : 1.0;
                        string tok = (colName == "primary" && !string.IsNullOrEmpty(runtimePrimaryHex)) ? runtimePrimaryHex : MapColorToken(colName);
                        var winCol = tok.StartsWith("#") ? ParseHexWinColor(tok, sOp) : Windows.UI.Color.FromArgb(255, 0, 120, 212);
                        lgb.GradientStops.Add(new Microsoft.UI.Xaml.Media.GradientStop { Color = winCol, Offset = pos });
                    }
                }
                sp.Background = lgb;
            } else if (node.Props.ContainsKey("background")) {
                var brush = ResolveAppBrush(node.Props["background"].ToString());
                if (brush != null) sp.Background = brush;
            }
        }
"##;
