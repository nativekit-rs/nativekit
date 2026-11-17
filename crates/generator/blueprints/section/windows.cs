using System.Collections.Generic;
using Microsoft.UI.Xaml;
using Microsoft.UI.Xaml.Controls;

UIElement Render(DynamicUINode node) {
    var wrap = new StackPanel { Orientation = Orientation.Vertical, Spacing = 6, HorizontalAlignment = HorizontalAlignment.Stretch };
    DynamicUINode hdrNode = null, ftrNode = null;
    var body = new List<DynamicUINode>();
    if (node.Children != null) {
        foreach (var c in node.Children) {
            if (c.Name == "section_header" && c.Children != null && c.Children.Count > 0) hdrNode = c.Children[0];
            else if (c.Name == "section_footer" && c.Children != null && c.Children.Count > 0) ftrNode = c.Children[0];
            else body.Add(c);
        }
    }
    if (hdrNode != null) wrap.Children.Add(RenderNode(hdrNode));
    var rowsPanel = new StackPanel { Orientation = Orientation.Vertical, Spacing = 2, HorizontalAlignment = HorizontalAlignment.Stretch };
    int count = body.Count;
    for (int i = 0; i < count; i++) {
        var c = body[i];
        var rad = count == 1 ? new CornerRadius(8) : (i == 0 ? new CornerRadius(8, 8, 2, 2) : (i == count - 1 ? new CornerRadius(2, 2, 8, 8) : new CornerRadius(2)));
        bool isBtn = (c.Name == "button");
        Microsoft.UI.Xaml.Media.Brush rowBgBrush = null;
        if (c.Props != null && c.Props.ContainsKey("background")) {
            rowBgBrush = ResolveAppBrush(c.Props["background"].ToString());
            c.Props.Remove("background");
        }
        UIElement rowChild;
        if (isBtn && c.Children != null && c.Children.Count > 0) {
            var sp = new StackPanel { Orientation = Orientation.Horizontal, Spacing = 8 };
            foreach (var gc in c.Children) {
                var r = RenderNode(gc);
                if (r is TextBlock rowTb) {
                    var fg = ResolveAppBrush((c.Props != null && c.Props.ContainsKey("color")) ? c.Props["color"].ToString() : "primary");
                    if (fg != null) rowTb.Foreground = fg;
                }
                sp.Children.Add(r);
            }
            rowChild = sp;
        } else {
            rowChild = RenderNode(c);
        }
        var card = new Border {
            CornerRadius = rad, Padding = new Thickness(16, 12, 16, 12), HorizontalAlignment = HorizontalAlignment.Stretch,
            Background = rowBgBrush ?? (Application.Current.Resources["CardBackgroundFillColorDefaultBrush"] as Microsoft.UI.Xaml.Media.Brush),
            BorderBrush = Application.Current.Resources["CardStrokeColorDefaultBrush"] as Microsoft.UI.Xaml.Media.Brush,
            BorderThickness = new Thickness(1), Child = rowChild
        };
        if (isBtn && c.Props != null && c.Props.ContainsKey("action")) {
            string act = c.Props["action"].ToString();
            card.PointerPressed += (s, e) => { if (!string.IsNullOrEmpty(act)) Dispatch(act, ""); };
        }
        rowsPanel.Children.Add(card);
    }
    wrap.Children.Add(rowsPanel);
    if (ftrNode != null) wrap.Children.Add(RenderNode(ftrNode));
    return wrap;
}
