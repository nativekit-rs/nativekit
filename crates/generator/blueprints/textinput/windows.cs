using Microsoft.UI.Xaml;
using Microsoft.UI.Xaml.Controls;

UIElement Render(DynamicUINode node) {
    string valStr = (node.Props != null && node.Props.ContainsKey("value")) ? node.Props["value"].ToString() : "";
    string placeholder = (node.Props != null && node.Props.ContainsKey("placeholder")) ? node.Props["placeholder"].ToString() : "";
    string onChange = (node.Props != null && node.Props.ContainsKey("on_change")) ? node.Props["on_change"].ToString() : "";
    var inputBox = new TextBox {
        Text = valStr,
        PlaceholderText = placeholder,
        HorizontalAlignment = HorizontalAlignment.Stretch,
        CornerRadius = new CornerRadius(6),
        Tag = onChange
    };
    string lastSent = valStr;
    inputBox.TextChanged += (s, e) => {
        if (inputBox.FocusState != FocusState.Unfocused && inputBox.Tag is string act && !string.IsNullOrEmpty(act) && inputBox.Text != lastSent) {
            lastSent = inputBox.Text;
            Dispatch(act, inputBox.Text);
        }
    };
    return inputBox;
}
