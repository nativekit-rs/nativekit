using Microsoft.UI.Xaml;
using Microsoft.UI.Xaml.Controls;

UIElement Render(DynamicUINode node) {
    var b = new Border();
    if (node.Props != null && node.Props.ContainsKey("size") && double.TryParse(node.Props["size"].ToString(), out double sz) && sz > 0) {
        b.Width = sz;
        b.Height = sz;
    } else {
        b.HorizontalAlignment = HorizontalAlignment.Stretch;
    }
    return b;
}
