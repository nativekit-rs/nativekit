export default function Page(node) {
    const { title = "", subtitle = "", on_back = "", presentation = "push", top_style = "soft", section_spacing = 16 } = node.props || {};
    const isForm = ["form", "dialog", "modal", "fullscreen"].includes(presentation);
    const children = node.children || [];
    const toolbarNode = children.find(c => c.name === "toolbartop");
    const toolbarItems = (toolbarNode && toolbarNode.children) ? toolbarNode.children : [];
    const getPlacement = (item) => (item.props && item.props.placement) || (item.children && item.children[0] && item.children[0].props && item.children[0].props.placement) || "trailing";
    const leadingItems = toolbarItems.filter(i => getPlacement(i) === "leading").map(i => renderNode(i));
    const trailingItems = toolbarItems.filter(i => getPlacement(i) !== "leading").map(i => renderNode(i));
    const bodyChildren = children.filter(c => c.name !== "toolbartop" && c.name !== "toolbarbottom").map(c => renderNode(c));

    const backBtn = (leadingItems.length === 0 && on_back) ? h("button", {
        className: "back-button",
        html: isForm ? "&#10005;" : "&#8592; Back",
        style: { background: "none", border: "none", color: "var(--rn-primary, #007aff)", cursor: "pointer", fontSize: "15px" },
        onClick: () => dispatch(on_back, null),
    }) : null;

    const navbar = h("nav", {
        className: "native-navbar",
        style: {
            display: "flex",
            alignItems: "center",
            justifyContent: "space-between",
            padding: "12px 16px",
            minHeight: "44px",
            background: top_style === "hard" ? "var(--rn-surface, #ffffff)" : "transparent",
            borderBottom: top_style === "hard" ? "1px solid var(--rn-border, #e5e5ea)" : "none",
        },
    }, [
        h("div", { style: { display: "flex", alignItems: "center", gap: "8px", minWidth: "80px" } }, leadingItems.length > 0 ? leadingItems : backBtn),
        h("div", { style: { textAlign: "center", flex: "1" } }, [
            h("div", { className: "page-title", text: title, style: { fontWeight: "600", fontSize: "17px" } }),
            subtitle ? h("div", { text: subtitle, style: { fontSize: "12px", color: "gray" } }) : null,
        ]),
        h("div", { style: { display: "flex", alignItems: "center", justifyContent: "flex-end", gap: "8px", minWidth: "80px" } }, trailingItems),
    ]);

    const content = h("div", {
        className: "page-content",
        style: { flex: "1", overflowY: "auto", padding: "16px", boxSizing: "border-box", display: "flex", flexDirection: "column", gap: `${section_spacing}px` },
    }, bodyChildren);

    return h("div", {
        className: "page",
        style: { display: "flex", flexDirection: "column", width: "100%", height: "100%" },
    }, [navbar, content]);
}
