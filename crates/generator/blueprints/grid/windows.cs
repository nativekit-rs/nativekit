using System;
using System.Collections.Generic;
using Microsoft.UI.Xaml;
using Microsoft.UI.Xaml.Controls;

UIElement Render(DynamicUINode node) {
    int cols = (node.Props != null && node.Props.ContainsKey("columns") && int.TryParse(node.Props["columns"].ToString(), out int cVal)) ? Math.Max(1, cVal) : 2;
    double spacing = (node.Props != null && node.Props.ContainsKey("spacing") && double.TryParse(node.Props["spacing"].ToString(), out double sVal)) ? sVal : 12;
    DynamicUINode hdrNode = null, ftrNode = null;
    var body = new List<DynamicUINode>();
    if (node.Children != null) {
        foreach (var c in node.Children) {
            if (c.Name == "section_header" && c.Children != null && c.Children.Count > 0) hdrNode = c.Children[0];
            else if (c.Name == "section_footer" && c.Children != null && c.Children.Count > 0) ftrNode = c.Children[0];
            else body.Add(c);
        }
    }
    var wrap = new StackPanel { Orientation = Orientation.Vertical, Spacing = 6, HorizontalAlignment = HorizontalAlignment.Stretch };
    if (hdrNode != null) wrap.Children.Add(RenderNode(hdrNode));
    var grid = new Grid { ColumnSpacing = spacing, RowSpacing = spacing, HorizontalAlignment = HorizontalAlignment.Stretch };
    for (int c = 0; c < cols; c++) grid.ColumnDefinitions.Add(new ColumnDefinition { Width = new GridLength(1, GridUnitType.Star) });
    int count = body.Count;
    int rows = (count + cols - 1) / cols;
    for (int r = 0; r < rows; r++) grid.RowDefinitions.Add(new RowDefinition { Height = GridLength.Auto });
    for (int i = 0; i < count; i++) {
        var el = RenderNode(body[i]);
        grid.Children.Add(el);
        if (el is FrameworkElement fe) { Grid.SetColumn(fe, i % cols); Grid.SetRow(fe, i / cols); }
    }
    wrap.Children.Add(grid);
    if (ftrNode != null) wrap.Children.Add(RenderNode(ftrNode));
    return wrap;
}
