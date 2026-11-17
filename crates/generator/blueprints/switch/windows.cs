using Microsoft.UI.Xaml;
using Microsoft.UI.Xaml.Controls;

UIElement Render(DynamicUINode node) {
    var sw = new CheckBox();
    if (node.Props != null && node.Props.ContainsKey("is_on") && node.Props["is_on"] is bool b) {
        sw.IsChecked = b;
    }
    if (node.Props != null && node.Props.ContainsKey("on_toggle")) {
        string action = node.Props["on_toggle"].ToString();
        sw.Checked += (s, e) => Dispatch(action, "true");
        sw.Unchecked += (s, e) => Dispatch(action, "false");
    }
    return sw;
}
