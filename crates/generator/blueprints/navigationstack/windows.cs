using Microsoft.UI.Xaml;
using Microsoft.UI.Xaml.Controls;

UIElement Render(DynamicUINode node) {
    var navGrid = new Grid { HorizontalAlignment = HorizontalAlignment.Stretch, VerticalAlignment = VerticalAlignment.Stretch };
    if (node.Children != null && node.Children.Count > 0) {
        ExtractNavLayers(node, out var windowPages, out var modalLayers);

        if (windowPages.Count > 0) {
            var activeWin = windowPages[windowPages.Count - 1];
            lastNavDepth = windowPages.Count;
            lastPageTitle = (activeWin.Props != null && activeWin.Props.ContainsKey("title")) ? activeWin.Props["title"].ToString() : "";
            var pageEl = RenderNode(activeWin);
            navGrid.Children.Add(pageEl);
            AnimatePageIn(pageEl, true);
        }

        for (int i = 0; i < modalLayers.Count; i++) {
            var modalOverlay = BuildModalOverlay(modalLayers[i], i, true, false);
            navGrid.Children.Add(modalOverlay);
        }
    }
    return navGrid;
}
