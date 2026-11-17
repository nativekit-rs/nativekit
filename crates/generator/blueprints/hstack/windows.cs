using Microsoft.UI.Xaml;
using Microsoft.UI.Xaml.Controls;

UIElement Render(DynamicUINode node) {
    double hSpace = (node.Props != null && node.Props.ContainsKey("spacing") && double.TryParse(node.Props["spacing"].ToString(), out double sp)) ? sp : 0;
    string widthProp = (node.Props != null && node.Props.ContainsKey("width")) ? node.Props["width"].ToString() : "";
    string alignX = (node.Props != null && node.Props.ContainsKey("align_x")) ? node.Props["align_x"].ToString() : "left";
    string alignY = (node.Props != null && node.Props.ContainsKey("align_y")) ? node.Props["align_y"].ToString() : "center";
    bool isHdr = (node.Props != null && node.Props.ContainsKey("is_header_bar") && node.Props["is_header_bar"] is bool bHdr && bHdr);
    int childCount = node.Children != null ? node.Children.Count : 0;
    bool isSpaceBetween = alignX == "space_between" || (isHdr && childCount == 2);
    bool hasFlex = false;
    if (node.Children != null) {
        foreach (var c in node.Children) {
            string cw = (c.Props != null && c.Props.ContainsKey("width")) ? c.Props["width"].ToString() : "";
            bool hasSize = c.Props != null && c.Props.ContainsKey("size") && double.TryParse(c.Props["size"].ToString(), out double sz) && sz > 0;
            if (cw == "fill" || (c.Name == "spacer" && !hasSize) || (c.Name == "vstack" && cw != "hug")) {
                hasFlex = true;
                break;
            }
        }
    }
    bool isFill = widthProp == "fill" || (widthProp != "hug" && (hasFlex || isSpaceBetween));
    var vAlign = alignY == "top" ? VerticalAlignment.Top : (alignY == "bottom" ? VerticalAlignment.Bottom : VerticalAlignment.Center);

    if (isFill && (hasFlex || isSpaceBetween) && node.Children != null) {
        var grid = new Grid { ColumnSpacing = hSpace, HorizontalAlignment = HorizontalAlignment.Stretch, VerticalAlignment = vAlign };
        ApplyWindowsContainerStyle(grid, node);
        int colIdx = 0;
        for (int i = 0; i < childCount; i++) {
            if (isSpaceBetween && i > 0) {
                grid.ColumnDefinitions.Add(new ColumnDefinition { Width = new GridLength(1, GridUnitType.Star) });
                colIdx++;
            }
            var c = node.Children[i];
            string cw = (c.Props != null && c.Props.ContainsKey("width")) ? c.Props["width"].ToString() : "";
            bool hasSize = c.Props != null && c.Props.ContainsKey("size") && double.TryParse(c.Props["size"].ToString(), out double sz) && sz > 0;
            bool cFlex = !isSpaceBetween && (cw == "fill" || (c.Name == "spacer" && !hasSize) || (c.Name == "vstack" && cw != "hug"));
            grid.ColumnDefinitions.Add(new ColumnDefinition { Width = cFlex ? new GridLength(1, GridUnitType.Star) : GridLength.Auto });
            var el = RenderNode(c);
            if (el is FrameworkElement fe) {
                fe.VerticalAlignment = vAlign;
                Grid.SetColumn(fe, colIdx);
            }
            grid.Children.Add(el);
            colIdx++;
        }
        return grid;
    }

    var hp = new StackPanel { Orientation = Orientation.Horizontal, Spacing = hSpace };
    hp.HorizontalAlignment = isFill ? HorizontalAlignment.Stretch : (alignX == "center" ? HorizontalAlignment.Center : (alignX == "right" ? HorizontalAlignment.Right : HorizontalAlignment.Left));
    hp.VerticalAlignment = vAlign;
    ApplyWindowsContainerStyle(hp, node);
    if (node.Children != null) {
        foreach (var c in node.Children) {
            var el = RenderNode(c);
            if (el is FrameworkElement fe) fe.VerticalAlignment = vAlign;
            hp.Children.Add(el);
        }
    }
    return hp;
}
