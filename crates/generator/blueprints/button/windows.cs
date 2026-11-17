using Microsoft.UI.Xaml;
using Microsoft.UI.Xaml.Controls;

UIElement Render(DynamicUINode node) {
    var btn = new Button();
    string variant = (node.Props != null && node.Props.ContainsKey("variant")) ? node.Props["variant"].ToString() : "";
    if (variant == "primary") {
        try { btn.Style = Application.Current.Resources["AccentButtonStyle"] as Style; var pb = ResolveAppBrush("primary"); if (pb != null) btn.Background = pb; } catch { }
    } else if (variant != "secondary") {
        btn.Background = new Microsoft.UI.Xaml.Media.SolidColorBrush(Microsoft.UI.Colors.Transparent);
        btn.BorderThickness = new Thickness(0);
        var fg = ResolveAppBrush("primary");
        if (fg != null) btn.Foreground = fg;
    }
    bool isFill = (node.Props != null && node.Props.ContainsKey("width") && node.Props["width"].ToString() == "fill");
    btn.HorizontalAlignment = isFill ? HorizontalAlignment.Stretch : HorizontalAlignment.Left;
    btn.HorizontalContentAlignment = HorizontalAlignment.Center;
    if (node.Props != null && node.Props.ContainsKey("color")) {
        var brush = ResolveAppBrush(node.Props["color"].ToString());
        if (brush != null) {
            if (variant == "primary" || variant == "secondary") btn.Background = brush;
            else btn.Foreground = brush;
        }
    }
    if (node.Props != null && node.Props.ContainsKey("opacity") && double.TryParse(node.Props["opacity"].ToString(), out double bOp)) {
        btn.Opacity = bOp / 100.0;
    }
    if (node.Children != null && node.Children.Count > 0) {
        btn.Content = RenderNode(node.Children[0]);
    }
    if (node.Props != null && node.Props.ContainsKey("action")) {
        btn.Tag = node.Props["action"].ToString();
        btn.Click += (s, e) => { if (btn.Tag is string act) Dispatch(act, ""); };
    }
    return btn;
}
