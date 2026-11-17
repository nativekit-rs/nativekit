using Microsoft.UI.Xaml;
using Microsoft.UI.Xaml.Controls;

UIElement Render(DynamicUINode node) {
    var pageScroll = new ScrollViewer {
        IsTabStop = true,
        HorizontalScrollBarVisibility = ScrollBarVisibility.Disabled,
        VerticalScrollBarVisibility = ScrollBarVisibility.Auto,
        HorizontalAlignment = HorizontalAlignment.Stretch,
        VerticalAlignment = VerticalAlignment.Stretch
    };
    int sectionSpacing = 16;
    if (node.Props != null && node.Props.ContainsKey("section_spacing") && int.TryParse(node.Props["section_spacing"].ToString(), out int sVal)) {
        sectionSpacing = sVal;
    }
    var pageContent = new StackPanel {
        HorizontalAlignment = HorizontalAlignment.Stretch,
        Margin = new Thickness(16, 16, 16, 24),
        Spacing = sectionSpacing,
        Background = new Microsoft.UI.Xaml.Media.SolidColorBrush(Microsoft.UI.Colors.Transparent)
    };
    pageContent.PointerPressed += (s, e) => { pageScroll.Focus(FocusState.Programmatic); };

    string pTitle = (node.Props != null && node.Props.ContainsKey("title")) ? node.Props["title"].ToString() : "";
    DynamicUINode toolbarNode = null;
    if (node.Children != null) {
        foreach (var c in node.Children) { if (c.Name == "toolbartop") { toolbarNode = c; break; } }
    }

    if (!string.IsNullOrEmpty(pTitle) || toolbarNode != null) {
        var headerGrid = new Grid { Margin = new Thickness(0, 0, 0, 16), Tag = "__page_header__" };
        headerGrid.ColumnDefinitions.Add(new ColumnDefinition { Width = GridLength.Auto });
        headerGrid.ColumnDefinitions.Add(new ColumnDefinition { Width = new GridLength(1, GridUnitType.Star) });
        headerGrid.ColumnDefinitions.Add(new ColumnDefinition { Width = GridLength.Auto });

        var leadPanel = new StackPanel { Orientation = Orientation.Horizontal, Spacing = 8, VerticalAlignment = VerticalAlignment.Center, Margin = new Thickness(0, 0, 12, 0) };
        var trailPanel = new StackPanel { Orientation = Orientation.Horizontal, Spacing = 8, VerticalAlignment = VerticalAlignment.Center };

        if (toolbarNode != null && toolbarNode.Children != null) {
            foreach (var item in toolbarNode.Children) {
                string pl = (item.Props != null && item.Props.ContainsKey("placement")) ? item.Props["placement"].ToString() : "trailing";
                if (pl == "leading") leadPanel.Children.Add(RenderNode(item));
                else trailPanel.Children.Add(RenderNode(item));
            }
        }
        headerGrid.Children.Add(leadPanel);
        Grid.SetColumn(leadPanel, 0);

        if (!string.IsNullOrEmpty(pTitle)) {
            var titleTb = new TextBlock {
                Text = pTitle,
                FontSize = 28,
                FontWeight = Microsoft.UI.Text.FontWeights.SemiBold,
                VerticalAlignment = VerticalAlignment.Center
            };
            headerGrid.Children.Add(titleTb);
            Grid.SetColumn(titleTb, 1);
        }

        headerGrid.Children.Add(trailPanel);
        Grid.SetColumn(trailPanel, 2);
        pageContent.Children.Add(headerGrid);
    }

    if (node.Children != null) {
        foreach (var c in node.Children) {
            if (c.Name != "toolbartop" && c.Name != "toolbarbottom") pageContent.Children.Add(RenderNode(c));
        }
    }
    pageScroll.Content = pageContent;
    return pageScroll;
}
