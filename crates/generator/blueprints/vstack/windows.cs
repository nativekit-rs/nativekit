using Microsoft.UI.Xaml;
using Microsoft.UI.Xaml.Controls;

UIElement Render(DynamicUINode node) {
    var vp = new StackPanel { Orientation = Orientation.Vertical };
    if (node.Props != null && node.Props.ContainsKey("spacing") && double.TryParse(node.Props["spacing"].ToString(), out double vSpace)) {
        vp.Spacing = vSpace;
    }
    string widthProp = (node.Props != null && node.Props.ContainsKey("width")) ? node.Props["width"].ToString() : "";
    string alignX = (node.Props != null && node.Props.ContainsKey("align_x")) ? node.Props["align_x"].ToString() : "left";
    string alignY = (node.Props != null && node.Props.ContainsKey("align_y")) ? node.Props["align_y"].ToString() : "top";
    vp.HorizontalAlignment = widthProp != "hug" ? HorizontalAlignment.Stretch : (alignX == "center" ? HorizontalAlignment.Center : (alignX == "right" ? HorizontalAlignment.Right : HorizontalAlignment.Left));
    vp.VerticalAlignment = alignY == "center" ? VerticalAlignment.Center : (alignY == "bottom" ? VerticalAlignment.Bottom : VerticalAlignment.Top);
    ApplyWindowsContainerStyle(vp, node);
    if (node.Children != null) {
        foreach (var c in node.Children) vp.Children.Add(RenderNode(c));
    }
    return vp;
}
