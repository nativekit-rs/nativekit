using Microsoft.UI.Xaml;
using Microsoft.UI.Xaml.Controls;
UIElement Render(DynamicUINode node) {
    var sp = new StackPanel { Orientation = Orientation.Horizontal, Spacing = 6 };
    if (node.Children != null) { foreach (var c in node.Children) sp.Children.Add(RenderNode(c)); }
    return sp;
}
